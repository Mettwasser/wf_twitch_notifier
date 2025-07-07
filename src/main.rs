pub mod arbitrations;
pub mod cli;
pub mod config;
pub mod credentials;
pub mod register;
pub mod token_storage;

use anyhow::{
    Context,
    bail,
};
use chrono::{
    DateTime,
    Utc,
};
use clap::Parser;
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
    message::ServerMessage,
};

use crate::{
    arbitrations::load_arbi_data,
    cli::{
        Cli,
        Commands,
    },
    config::Config,
    credentials::ComposedCredentials,
    token_storage::{
        CREDENTIALS_PATH,
        SimpleTokenStorage,
    },
};

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
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            id,
            secret,
            access_token,
            refresh_token,
            expires_at,
        } => init(id, secret, access_token, refresh_token, expires_at).await?,
        Commands::Run { channel_name } => run(channel_name).await?,
    }

    Ok(())
}

async fn init(
    id: String,
    secret: String,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
) -> anyhow::Result<()> {
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
    fs::write("./.credentials.json", contents).await?;

    Ok(())
}

async fn run(channel_name: String) -> anyhow::Result<()> {
    let arbi_data = load_arbi_data()?;
    let credentials = load_credentials().await?;
    let notifier_config = Config::load()?;

    let mut join_set = JoinSet::new();

    let wf = warframe::worldstate::Client::new();

    let (mut incoming_msgs, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<SimpleTokenStorage>,
    >::new(ClientConfig::new_simple(
        RefreshingLoginCredentials::<SimpleTokenStorage>::init(
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
        ),
    ));

    tokio::spawn(async move {
        while let Some(message) = incoming_msgs.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                tracing::debug!(message = ?msg.message_text);
            }
        }
    });

    client.join(channel_name.clone()).unwrap();

    client
        .say(
            channel_name.clone(),
            format!("Hello @{channel_name}, I'm running the setup!"),
        )
        .await
        .unwrap();

    register::register_listeners(
        &mut join_set,
        notifier_config,
        channel_name.clone(),
        client.clone(),
        wf,
        arbi_data,
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
