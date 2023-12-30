use oauth2::TokenResponse;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DiscordClient {
    inner: reqwest::Client,
}

impl DiscordClient {
    pub fn base_url() -> String {
        "https://discord.com/api/v10".to_string()
    }

    pub fn from_token(token: &AuthToken) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!(
            "Bearer {}",
            token.access_token().secret()
        ))
        .map_err(|err| Error::Other(err.to_string()))?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| Error::Other(err.to_string()))?;
        Ok(Self { inner: client })
    }

    pub fn from_bot_token(token: &str) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!("Bot {}", token))
            .map_err(|err| Error::Other(err.to_string()))?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| Error::Other(err.to_string()))?;
        Ok(Self { inner: client })
    }

    pub async fn fetch_current_user(&self) -> Result<discord::CurrentUser, Error> {
        let url = format!("{}/users/@me", Self::base_url());
        let resp = self
            .inner
            .get(&url)
            .send()
            .await
            .map_err(|err| Error::Other(err.to_string()))?;
        let user = resp.json().await?;
        Ok(user)
    }

    pub async fn fetch_current_user_guilds(&self) -> Result<Vec<discord::GuildInfo>, Error> {
        let url = format!("{}/users/@me/guilds", Self::base_url());
        let resp = self
            .inner
            .get(&url)
            .send()
            .await
            .map_err(|err| Error::Other(err.to_string()))?;
        let guilds = resp.json().await?;
        Ok(guilds)
    }
}
