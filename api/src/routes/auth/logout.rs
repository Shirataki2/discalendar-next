use actix_session::Session;

use crate::prelude::*;

#[get("/logout")]
async fn logout(req: HttpRequest , sess: Session) -> Result<HttpResponse, Error> {
    sess.clear();

    let app_config = get_data::<crate::AppConfig>(&req)?;

    Ok(HttpResponse::Found()
        .append_header((actix_web::http::header::LOCATION, app_config.frontend_url.clone()))
        .finish())
}
