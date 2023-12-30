pub use crate::{error::Error, routes::get_data, arguments, discord::DiscordClient};
pub use actix_web::{
    get,
    http::{header, StatusCode},
    post, put,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
pub use actix_session::Session;
pub use anyhow::Context as _;
use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};
pub use poise::serenity_prelude as discord;
pub use service::{sea_orm::DbConn, self};

pub type AuthToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
