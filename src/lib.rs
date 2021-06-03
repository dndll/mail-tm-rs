use anyhow::{Context, Error};

use auth::Token;
use accounts::Account;
use tokio::time::Duration;
use user::User;

pub(crate) mod auth;
pub(crate) mod domains;
pub(crate) mod accounts;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod inspect;
pub(crate) mod list;
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
    let token = auth::get_token(user).await?;
    log::debug!("Retrieved email token, response: {:?}", token);
    Ok(token)
}

pub async fn get_link(user: &User) -> Result<String, Error> {
    log::info!("Verifying user..");
    log::info!("Listing messages..");
    let mut messages = list::list_messages(&user.email_token).await?;
    while messages.hydra_member.len() == 0 {
        log::info!("Listing messages..");
        messages = list::list_messages(&user.email_token).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    log::info!("Getting first email..");
    let option = messages.hydra_member.first();
    let string = option
        .cloned()
        .context("Failed to get the first email member")?
        .id;
    log::info!("Inspecting email..");
    let message = inspect::inspect_email(string, &user.email_token).await?;
    log::info!("Extracting link..");
    log::info!("Message.. {:?}", message);
    Ok(inspect::extract_link(message)?)
}

pub async fn verify(link: &str) -> Result<bool, Error> {
    log::info!("Verifying..");
    match inspect::verify(&link).await? {
        true => Ok(true),
        false => Err(Error::msg("Nonexistent link")),
    }
}
