use std::{
    iter,
    sync::Arc,
};

use arbitration_data::ArbitrationData;
use twitch_irc::{
    SecureTCPTransport,
    TwitchIRCClient,
    login::RefreshingLoginCredentials,
};
use warframe::{
    market,
    worldstate,
};

use crate::{
    commands::{
        self,
        config::CommandConfig,
    },
    config::Config,
    credentials::ComposedCredentials,
    listener::config::ListenerConfig,
    placeholder::{
        ChannelName,
        Placeholder,
        apply_placeholders,
    },
    token_storage::SimpleTokenStorage,
};

#[derive(Clone)]
pub struct State {
    pub client: TwitchIRCClient<SecureTCPTransport, RefreshingLoginCredentials<SimpleTokenStorage>>,
    pub config: Arc<Config>,
    pub credentials: Arc<ComposedCredentials>,
    pub arbi_data: Arc<ArbitrationData>,
    pub channel_name: ChannelName,
    pub wf: worldstate::Client,
    pub wfm: Arc<market::Client>,
}

impl State {
    pub async fn send_command_response<P: Placeholder>(
        &self,
        fmt: &str,
        author: &str,
        placeholders: impl IntoIterator<Item = P>,
    ) -> anyhow::Result<()> {
        let intermediate_message = apply_placeholders(fmt, placeholders);

        let author_placeholder = commands::placeholders::Author(author);

        let final_message = apply_placeholders(
            &intermediate_message,
            iter::once(&author_placeholder as &dyn Placeholder),
        );

        self.client
            .say(self.channel_name.to_string(), final_message)
            .await?;

        Ok(())
    }

    pub async fn send_listener_response<P: Placeholder>(
        &self,
        fmt: &str,
        placeholders: impl IntoIterator<Item = P>,
    ) -> anyhow::Result<()> {
        let intermediate_message = apply_placeholders(fmt, placeholders);

        let final_message =
            apply_placeholders(&intermediate_message, iter::once(&self.channel_name));

        self.client
            .say(self.channel_name.to_string(), final_message)
            .await?;

        Ok(())
    }

    pub fn listener_cfg(&self) -> &ListenerConfig {
        &self.config.listener_config
    }

    pub fn command_cfg(&self) -> &CommandConfig {
        &self.config.command_config
    }
}
