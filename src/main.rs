pub mod arbitrations;
pub mod cli;
pub mod config;
pub mod credentials;
pub mod listener;
pub mod placeholder;
pub mod register;
pub mod state;
pub mod token_storage;

use std::sync::Arc;

use anyhow::{
    Context,
    bail,
};
use chrono::{
    DateTime,
    Utc,
};
use clap::Parser;
use regex::Regex;
use tokio::{
    fs,
    task::JoinSet,
};
use tracing_subscriber::filter::LevelFilter;
use twitch_irc::{
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
    login::{
        RefreshingLoginCredentials,
        UserAccessToken,
    },
};

use crate::{
    arbitrations::load_arbi_data,
    cli::{
        Cli,
        Commands,
    },
    config::Config,
    credentials::ComposedCredentials,
    placeholder::ChannelName,
    state::State,
    token_storage::{
        CREDENTIALS_PATH,
        SimpleTokenStorage,
    },
};

const INIT_FILE_PATH: &str = "./init.txt";

async fn load_credentials() -> anyhow::Result<ComposedCredentials> {
    let contents = fs::read_to_string(CREDENTIALS_PATH)
        .await
        .context(format!("Failed to read credentials at {CREDENTIALS_PATH}"))?;

    let token: ComposedCredentials = serde_json::from_str(&contents)?;
    Ok(token)
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { id, secret } => init(id, secret).await?,
        Commands::Run { channel_name } => run(channel_name).await?,
    }

    Ok(())
}

fn parse_datetime_utc(s: &str) -> anyhow::Result<DateTime<Utc>> {
    let format = "%Y-%m-%d %H:%M:%S%.f %z %Z";

    Ok(DateTime::parse_from_str(s.trim(), format).map(|dt| dt.with_timezone(&Utc))?)
}

async fn init(id: String, secret: String) -> anyhow::Result<()> {
    Config::load()?;

    let regex = Regex::new(r"User Access Token:\s+(?<accessToken>\w+)\s*[\r\n]+.*?Refresh Token:\s+(?<refreshToken>\w+)\s*[\r\n]+.*?Expires At:\s+(?<expiresAt>.*)").unwrap();

    let contents = fs::read_to_string(INIT_FILE_PATH)
        .await
        .context(format!("Failed to read init file at {INIT_FILE_PATH}"))?;

    let captures = regex
        .captures(&contents)
        .context("Failed to parse init file")?;

    let access_token = captures.name("accessToken").unwrap().as_str().to_string();
    let refresh_token = captures.name("refreshToken").unwrap().as_str().to_string();
    let expires_at = parse_datetime_utc(captures.name("expiresAt").unwrap().as_str())
        .context("Failed to parse expires_at")?;

    let credentials = ComposedCredentials {
        client_id: id,
        client_secret: secret,
        user_access_token: UserAccessToken {
            access_token,
            refresh_token,
            created_at: Utc::now(),
            expires_at: Some(expires_at),
        },
    };

    let contents = serde_json::to_string_pretty(&credentials)?;
    fs::write(CREDENTIALS_PATH, contents).await?;

    fs::remove_file(INIT_FILE_PATH)
        .await
        .context(format!("Failed to remove init file at {INIT_FILE_PATH}"))?;

    Ok(())
}

async fn run(channel_name: String) -> anyhow::Result<()> {
    let arbi_data = load_arbi_data()?;
    let credentials = load_credentials().await?;
    let notifier_config = Config::load()?;

    let mut join_set = JoinSet::new();

    let wf = warframe::worldstate::Client::new();

    let (_, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<SimpleTokenStorage>,
    >::new(ClientConfig::new_simple(RefreshingLoginCredentials::<
        SimpleTokenStorage,
    >::init(
        credentials.client_id.clone(),
        credentials.client_secret.clone(),
        SimpleTokenStorage(match tokio::fs::read_to_string(CREDENTIALS_PATH).await {
            Ok(contents) => {
                let token: ComposedCredentials = serde_json::from_str(&contents)?;
                token
            }
            Err(_) => bail!(
                "Failed to read {}. Please use the init command (`wf_twitch_notifier init -h` for more info)",
                CREDENTIALS_PATH
            ),
        }),
    )));

    client.join(channel_name.clone()).unwrap();

    client
        .say(
            channel_name.clone(),
            format!("Hello @{channel_name}, I'm running the setup!"),
        )
        .await
        .unwrap();

    // notifier_config,
    // channel_name.clone(),
    // client.clone(),
    // wf,
    // arbi_data,
    register::register_listeners(
        &mut join_set,
        State {
            client: client.clone(),
            config: Arc::new(notifier_config),
            credentials: Arc::new(credentials),
            arbi_data: Arc::new(arbi_data),
            wf,
            channel_name: ChannelName::from(channel_name.clone()),
        },
    )?;

    client
        .say(
            channel_name.clone(),
            format!("@{channel_name}, setup successful!"),
        )
        .await
        .unwrap();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(_) => tracing::info!("Task completed successfully"),
            Err(e) => tracing::error!("Task failed: {:?}", e),
        }
    }

    Ok(())
}
