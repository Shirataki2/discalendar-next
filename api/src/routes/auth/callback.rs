use actix_session::Session;
use oauth2::{reqwest::async_http_client, CsrfToken, PkceCodeVerifier};

use crate::{
    discord::DiscordClient,
    oauth2_client::{OAuth2Client, OAuth2Query},
    prelude::*,
};
use service::AccountQuery;

#[get("/callback")]
async fn callback(
    req: HttpRequest,
    sess: Session,
    query: web::Query<OAuth2Query>,
) -> Result<HttpResponse, Error> {
    let state = query.into_inner().get_code()?;

    let csrf_token = match sess.get::<CsrfToken>("csrf_token")? {
        Some(csrf_token) => csrf_token,
        None => return Err(Error::Unauthorized("Missing CSRF token".to_string())),
    };

    if state.state.secret() != csrf_token.secret() {
        return Err(Error::Unauthorized("Invalid CSRF token".to_string()));
    }

    let pkce_verifier = match sess.get::<PkceCodeVerifier>("pkce_verifier")? {
        Some(pkce_verifier) => pkce_verifier,
        None => return Err(Error::Unauthorized("Missing PKCE verifier".to_string())),
    };

    let auth_client = get_data::<OAuth2Client>(&req)?.get_client();
    let token = auth_client
        .exchange_code(state.code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|err| Error::AuthCodeError(err.to_string(), "".to_string()))?;

    sess.insert("token", &token)?;

    let discord_client = DiscordClient::from_token(&token)?;
    let user = discord_client.fetch_current_user().await?;

    sess.insert("user", &user)?;

    let app_config = get_data::<crate::AppConfig>(&req)?;
    let db = get_data::<DbConn>(&req)?;
    AccountQuery::upsert(db, user.id.0 as i64, user.name.as_str()).await?;

    Ok(HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, app_config.frontend_url.clone()))
        .finish())
}
