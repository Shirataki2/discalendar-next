use anyhow::Context;
use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2Query {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2State {
    pub code: AuthorizationCode,
    pub state: CsrfToken,
}

impl OAuth2Query {
    pub fn from_query(query: web::Query<Self>) -> Self {
        query.into_inner()
    }

    pub fn get_code(&self) -> Result<OAuth2State, Error> {
        match self {
            Self {
                code: Some(code),
                state: Some(state),
                error: None,
                error_description: None,
            } => Ok(OAuth2State {
                code: AuthorizationCode::new(code.clone()),
                state: CsrfToken::new(state.clone()),
            }),
            Self {
                code: None,
                state: None,
                error: Some(error),
                error_description: Some(error_description),
            } => Err(Error::AuthCodeError(
                error.clone(),
                error_description.clone(),
            )),
            _ => Err(Error::Other("Invalid OAuth2 query".to_string())),
        }
    }
}

#[derive(Debug, Getters, Clone)]
pub struct OAuth2Client {
    #[get = "pub with_prefix"]
    client: BasicClient,
    #[get = "pub"]
    client_id: String,
    #[get = "pub"]
    client_secret: String,
    #[get = "pub"]
    auth_url: String,
    #[get = "pub"]
    token_url: String,
    #[get = "pub"]
    redirect_uri: String,
    #[get = "pub"]
    scopes: String,
}

impl OAuth2Client {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let client_id = std::env::var("CLIENT_ID").context("No CLIENT_ID")?;
        let client_secret = std::env::var("CLIENT_SECRET").context("No CLIENT_SECRET")?;
        let auth_url = std::env::var("AUTH_URL").context("No AUTH_URL")?;
        let token_url = std::env::var("TOKEN_URL").context("No TOKEN_URL")?;
        let redirect_uri = std::env::var("REDIRECT_URI").context("No REDIRECT_URI")?;
        let scopes = std::env::var("SCOPES").context("No SCOPES")?;

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone())?,
            Some(TokenUrl::new(token_url.clone())?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone())?);

        Ok(Self {
            client,
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_uri,
            scopes,
        })
    }

    pub fn from_env_discord() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let client_id = std::env::var("DISCORD_CLIENT_ID").context("No DISCORD_CLIENT_ID")?;
        let client_secret =
            std::env::var("DISCORD_CLIENT_SECRET").context("No DISCORD_CLIENT_SECRET")?;
        let auth_url = std::env::var("DISCORD_AUTH_URL").unwrap_or("https://discord.com/api/oauth2/authorize".to_string());
        let token_url = std::env::var("DISCORD_TOKEN_URL").unwrap_or("https://discord.com/api/oauth2/token".to_string());
        let redirect_uri = std::env::var("DISCORD_REDIRECT_URI").context("No DISCORD_REDIRECT_URI")?;
        let scopes = std::env::var("DISCORD_SCOPES").context("No DISCORD_SCOPES")?;

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone())?,
            Some(TokenUrl::new(token_url.clone())?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone())?);

        Ok(Self {
            client,
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_uri,
            scopes,
        })
    }

    pub fn get_auth_url(&self) -> anyhow::Result<PkceAuthState> {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(self.scopes.clone()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        Ok(PkceAuthState {
            authorize_url,
            csrf_state,
            pkce_code_verifier,
        })
    }
}

pub struct PkceAuthState {
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
}
