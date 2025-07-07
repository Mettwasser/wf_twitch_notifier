use twitch_irc::login::UserAccessToken;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposedCredentials {
    pub client_id: String,
    pub client_secret: String,

    #[serde(flatten)]
    pub user_access_token: UserAccessToken,
}
