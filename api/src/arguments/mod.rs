use std::{ops, pin::Pin, future::Future, rc::Rc};

use crate::prelude::*;
use actix_session::Session;
use actix_web::{FromRequest, dev::Payload};
// use ::entity::account::Model as Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User(discord::CurrentUser);

impl User {
    pub fn inner(&self) -> &discord::CurrentUser {
        &self.0
    }
}

impl ops::Deref for User {
    type Target = discord::CurrentUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_auth_token(sess: &Session) -> Result<AuthToken, Error> {
    match sess.get::<AuthToken>("token")? {
        Some(token) => Ok(token),
        None => Err(Error::Unauthorized("Missing auth token".to_string())),
    }
}

pub async fn get_user_from_session(req: &HttpRequest) -> Result<discord::CurrentUser, Error> {
    let sess = actix_session::Session::extract(req).await.map_err(|err| {
        error!("Failed to get session: {}", err);
        Error::Unauthorized("Failed to get session".to_string())
    })?;
    if let Some(user) = sess.get::<discord::CurrentUser>("user")? {
        Ok(user)
    } else {
        let token = get_auth_token(&sess)?;
        let client = crate::discord::DiscordClient::from_token(&token)?;
        let user = client.fetch_current_user().await?;
        sess.insert("user", &user)?;
        Ok(user)
    }
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req2 = Rc::new(req.clone());
        Box::pin(async move {
            let user = get_user_from_session(req2.as_ref()).await?;
            Ok(User(user))
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token(AuthToken);

impl Token {
    pub fn inner(&self) -> &AuthToken {
        &self.0
    }
}

impl ops::Deref for Token {
    type Target = AuthToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for Token {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req2 = Rc::new(req.clone());
        Box::pin(async move {
            let sess = actix_session::Session::extract(req2.as_ref()).await.map_err(|err| {
                error!("Failed to get session: {}", err);
                Error::Unauthorized("Failed to get session".to_string())
            })?;
            let token = get_auth_token(&sess)?;
            Ok(Token(token))
        })
    }
}
