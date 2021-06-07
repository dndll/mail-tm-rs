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

//! Mail-TM API implementation using common HTTP crates
//!
//! Provides an implementation of the Mail-TM 2.0.0 API
//! Largely it is around 80% complete and is missing possibly future deprecations such as sources.
//! At present the dependencies are very strict and requires future testing to open it up.
//!
//! Expect some breaking changes until v1.0.0 but will try to document them as best I can.
//!
//! [`Mail-TM`]: https://mail.tm/


/// Creates an account based on a user
///
/// This will make user of [`User`] to create an account.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token};
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let create = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// ```
pub async fn create_account(user: &User) -> Result<Account, Error> {
    accounts::create(user).await
}

/// Retrieve an account
///
/// Retrieve an account by its id. This uses the [`User::email_token`] field to build the auth header.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, get_account, update_token, token};
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// let account = get_account(&user, &account.id?).await?;
/// ```
pub async fn get_account(user: &User, id: &str) -> Result<Account, Error> {
    accounts::get(&user.email_token, id).await
}

/// Delete an account
///
/// Delete an account by its id. This uses the [`User::email_token`] field to build the auth header.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token, delete_account};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// delete_account(&user, &account.id?).await?;
/// ```
pub async fn delete_account(user: &User, id: &str) -> Result<(), Error> {
    accounts::delete(&user.email_token, id).await
}

/// Retrieve an account
///
/// This will retrieve the account belonging to the token holder
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token, me};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// let user = me(&user).await?;
/// ```
pub async fn me(user: &User) -> Result<Account, Error> {
    accounts::me(&user.email_token).await
}

/// Retrieve all available domains
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{domains};
///
/// let domains = domains().await?;
/// ```
pub async fn domains() -> Result<HydraCollection<Domain>, Error> {
    domains::domains().await
}

/// List messages
///
/// This will list messages belonging to the token holder. Has a page for optional page selection(inclusive).
/// Defaults to `1`
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token, me, list_messages};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// let messages = list_messages(&user, Some(33)).await?;
/// ```
pub async fn list_messages(user: &User, page: Option<usize>) -> Result<HydraCollection<Message>, Error> {
    messages::messages(&user.email_token, page).await
}

/// Get message
///
/// Retrieve a message by its id.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token, get_message};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// let messages = get_message(&user, "somemessageid").await?;
/// ```
pub async fn get_message(user: &User, id: &str) -> Result<Message, Error> {
    messages::get(&user.email_token, id).await
}

/// Delete message
///
/// Delete a message by its id.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token, delete_message};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// let messages = delete_message(&user, "somemessageid").await?;
/// ```
pub async fn delete_message(user: &User, id: &str) -> Result<(), Error> {
    messages::delete(&user.email_token, id).await
}

/// Retrieve a token for a user
///
/// You should update each user's token by using `update_token`. In the future we will support both
/// providing a raw token or a user.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// ```
pub async fn token(user: &User) -> Result<Token, Error> {
    token::token(user).await
}

/// Populates the email token on a user
///
/// This uses a simple builder like pattern. In the future we will support a zero-copy version too.
///
/// # Example
/// ```
/// use mail_tm_rs::user::User;
/// use mail_tm_rs::{create_account, update_token, token};
///
/// let user = User::default().with_domain(&crate::domains::domains().await?.any().domain);
/// let account = create_account(&user).await?;
/// let user = update_token(&user, &token(&user).await?.token);
/// ```
pub fn update_token(user: &User, token: &str) -> User {
    User {
        email_token: token.to_string(),
        ..user.clone()
    }
}