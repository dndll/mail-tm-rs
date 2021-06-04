use anyhow::{Context, Error};

use token::Token;
use accounts::Account;
use tokio::time::Duration;
use user::User;

pub(crate) mod token;
pub(crate) mod accounts;
pub(crate) mod domains;
pub(crate) mod messages;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod user;

pub(crate) const MAIL_API_URL: &str = "https://api.mail.tm";
pub(crate) const USER_AGENT: &str = "Reqwest; mail-tm-rs";

pub async fn create(user: &User) -> Result<Account, Error> {
    log::debug!(
        "Creating email user with id: {} and password {}..",
        user.id,
        user.password
    );
    let response = accounts::create(user).await?;
    log::debug!("Created email user, response: {:?}", response);
    Ok(response)
}

pub async fn token(user: &User) -> Result<Token, Error> {
    log::info!("Retrieving user token..");
    let token = token::get_token(user).await?;
    log::debug!("Retrieved email token, response: {:?}", token);
    Ok(token)
}