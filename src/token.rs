use serde::{Deserialize, Serialize};

use crate::http::{Client};
use crate::user::User;
use crate::MAIL_API_URL;
use anyhow::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub token: String,
    pub id: String,
}

pub async fn get_token(user: &User) -> Result<Token, Error> {
    let client = Client::new()?
        .build()?;

    log::debug!("Getting token for user {:?}", user);

    let create_as_string = serde_json::json!({
        "address": format!("{}@{}", user.id, user.domain).to_lowercase(),
        "password": user.password
    });

    let res = client
        .post(format!("{}/token", MAIL_API_URL).as_str())
        .body(create_as_string.to_string())
        .send()
        .await?;

    let body = res.text().await?;

    Ok(serde_json::from_str(&body)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_token() -> Result<(), Error> {
        let user = User::default();
        assert_eq!(get_token(&user).await?.token.is_empty(), false);
        Ok(())
    }
}
