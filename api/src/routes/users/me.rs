use crate::prelude::*;

#[get("/me")]
async fn me(
    me: arguments::User,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(me))
}
