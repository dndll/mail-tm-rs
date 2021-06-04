use anyhow::{Context, Error};

use token::Token;
use accounts::Account;
use tokio::time::Duration;
use user::User;
use crate::hydra::HydraCollection;
use crate::domains::Domain;
use crate::messages::Message;

pub mod token;
pub mod accounts;
pub mod domains;
pub mod messages;
pub mod error;
pub mod http;
pub mod hydra;
pub mod user;

pub(crate) const MAIL_API_URL: &str = "https://api.mail.tm";
pub(crate) const USER_AGENT: &str = "Reqwest; mail-tm-rs";

pub async fn create_account(user: &User) -> Result<Account, Error> {
    accounts::create(user).await
}

pub async fn get_account(user: &User, id: &str) -> Result<Account, Error> {
    accounts::get(&user.email_token, id).await
}

pub async fn delete_account(user: &User, id: &str) -> Result<(), Error> {
    accounts::delete(&user.email_token, id).await
}

pub async fn me(user: &User) -> Result<Account, Error> {
    accounts::me(&user.email_token).await
}

pub async fn domains() -> Result<HydraCollection<Domain>, Error> {
    domains::domains().await
}

pub async fn list_messages(user: &User, page: Option<usize>) -> Result<HydraCollection<Message>, Error> {
    messages::messages(&user.email_token, page).await
}

pub async fn get_message(user: &User, id: &str) -> Result<Message, Error> {
    messages::get(&user.email_token, id).await
}

pub async fn delete_message(user: &User, id: &str) -> Result<(), Error> {
    messages::delete(&user.email_token, id).await
}

pub async fn token(user: &User) -> Result<Token, Error> {
    token::token(user).await
}

pub async fn update_token(user: &User, token: &str) -> User {
    User {
        email_token: token.to_string(),
        ..user.clone()
    }
}