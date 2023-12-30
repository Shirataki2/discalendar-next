#![allow(clippy::collapsible_match)]
use actix_web::ResponseError;
use service::sea_orm;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // * Authentication errors
    #[error("Authorization Code Error ({0}): {1}")]
    AuthCodeError(String, String),

    // * Session errors
    #[error("Failed to get session: {0}")]
    SessionGetError(#[from] actix_session::SessionGetError),
    #[error("Failed to set session: {0}")]
    SessionInsertError(#[from] actix_session::SessionInsertError),

    // * General errors
    #[error("Authorization Error: {0}")]
    Unauthorized(String),

    // * Reqwest errors
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),

    // * Db errors
    #[error("{0}")]
    DbError(#[from] sea_orm::DbErr),

    // * Other errors
    #[error("Error: {0}")]
    Other(String),
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::AuthCodeError(_, _) => actix_web::http::StatusCode::UNAUTHORIZED,
            Error::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            Error::ReqwestError(e) => e
                .status()
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
            Error::DbError(e) => match e {
                sea_orm::DbErr::RecordNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
