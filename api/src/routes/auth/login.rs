use actix_session::Session;

use crate::{oauth2_client::OAuth2Client, prelude::*};

#[get("/login")]
async fn login(req: HttpRequest, sess: Session) -> Result<HttpResponse, Error> {
    let auth_client = get_data::<OAuth2Client>(&req)?;

    let auth = auth_client.get_auth_url()?;

    sess.insert("csrf_token", auth.csrf_state.secret())?;
    sess.insert("pkce_verifier", &auth.pkce_code_verifier)?;

    info!("Redirecting to {}", auth.authorize_url);

    Ok(HttpResponse::Found()
        .append_header((
            actix_web::http::header::LOCATION,
            auth.authorize_url.to_string(),
        ))
        .finish())
}
