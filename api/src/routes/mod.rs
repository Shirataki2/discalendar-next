use crate::prelude::*;
use actix_web::HttpRequest;
pub mod auth;
pub mod users;
pub mod guilds;

pub fn get_data<T>(req: &HttpRequest) -> Result<&T, crate::error::Error>
where
    T: 'static,
{
    match req.app_data::<T>() {
        Some(data) => Ok(data),
        None => {
            error!("Failed to get application data: {}", stringify!(T));
            Err(Error::Other("Internal Server Setup Failed".into()))
        }
    }
}

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::init_routes);
    cfg.configure(users::init_routes);
    cfg.configure(guilds::init_routes);
}
