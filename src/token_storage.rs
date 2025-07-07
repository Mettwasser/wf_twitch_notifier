use {
    crate::credentials::ComposedCredentials,
    twitch_irc::login::{TokenStorage, UserAccessToken},
};

pub const CREDENTIALS_PATH: &str = "./.credentials.json";

#[derive(Debug)]
pub struct SimpleTokenStorage(pub ComposedCredentials);

#[async_trait::async_trait]
impl TokenStorage for SimpleTokenStorage {
    type LoadError = anyhow::Error;
    type UpdateError = anyhow::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        Ok(self.0.user_access_token.clone())
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        self.0.user_access_token = token.clone();

        let contents = serde_json::to_string_pretty(&self.0)?;
        tokio::fs::write(CREDENTIALS_PATH, contents).await?;

        Ok(())
    }
}
