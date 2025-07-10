use std::sync::Arc;

use arbitration_data::ArbitrationData;
use twitch_irc::{
    SecureTCPTransport,
    TwitchIRCClient,
    login::RefreshingLoginCredentials,
};
use warframe::worldstate;

use crate::{
    config::Config,
    credentials::ComposedCredentials,
    placeholder::ChannelName,
    token_storage::SimpleTokenStorage,
};

#[derive(Clone)]
pub struct State {
    pub client: TwitchIRCClient<SecureTCPTransport, RefreshingLoginCredentials<SimpleTokenStorage>>,
    pub config: Arc<Config>,
    pub credentials: Arc<ComposedCredentials>,
    pub arbi_data: Arc<ArbitrationData>,
    pub wf: worldstate::Client,
    pub channel_name: ChannelName,
}
