#[allow(unused_imports)]
#[macro_use]
extern crate actix_web;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate getset;

pub mod arguments;
pub mod discord;
pub mod error;
pub mod oauth2_client;
pub mod prelude;
pub mod routes;

use crate::prelude::*;
use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key, SameSite},
    middleware::Logger,
    App, HttpServer,
};
use base64::{engine::general_purpose::STANDARD as BASE64ENGINE, Engine as _};
use service::sea_orm;

fn get_secret_key() -> anyhow::Result<Key> {
    let key = std::env::var("SECRET_KEY").context("No SECRET_KEY")?;
    let key = BASE64ENGINE.decode(key).context("Invalid Secret Key!")?;
    Ok(Key::from(&key))
}

#[get("/")]
async fn index() -> String {
    String::from("DAug API v1.0!")
}

#[get("/test")]
async fn test() -> String {
    log::trace!("This is a trace message");
    log::debug!("This is a debug message");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
    String::from("DAug API v1.0!")
}


#[derive(Debug, Clone)]
pub struct AppConfig {
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let frontend_url = std::env::var("FRONTEND_URL").context("No FRONTEND_URL")?;

        Ok(Self { frontend_url })
    }
}

pub async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("info");
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = sea_orm::Database::connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let discord_oauth2_client = oauth2_client::OAuth2Client::from_env_discord()?;

    let app_config = AppConfig::from_env()?;

    let sentry_url = std::env::var("SENTRY_URL").expect("SENTRY_URL must be set");
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(cfg!(debug_assertions).then(|| log::LevelFilter::Trace).unwrap_or(log::LevelFilter::Info));
    let _guard = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        dsn: sentry_url.parse().ok(),
        ..Default::default()
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_conn.clone())
            .app_data(app_config.clone())
            .app_data(discord_oauth2_client.clone())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::builder(&redis_url).build(),
                    get_secret_key().unwrap(),
                )
                .cookie_name("session_token".into())
                .cookie_same_site(SameSite::Lax)
                .cookie_secure(cfg!(debug_assertions))
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(28)))
                .build(),
            )
            .service(index)
            .service(test)
            .configure(routes::init_routes)
    })
    .bind((host.as_str(), port))?;

    info!("Starting server at http://{}:{}", host, port);

    server.run().await?;

    Ok(())
}
