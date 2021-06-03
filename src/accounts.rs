use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::http::{get_headers, Client};
use crate::user::User;
use crate::MAIL_API_URL;

// TODO make error
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "id")]
    pub id2: String,
    pub address: String,
    pub quota: i64,
    pub used: i64,
    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: serde_json::Value,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::serde_json::Value,
}

pub async fn create_email(user: &User) -> Result<Account, Error> {
    let client = Client::new()?.build()?;

    let create_as_string = serde_json::json!(user);
    let string = create_as_string.to_string();
    let res = client
        .post(format!("{}/accounts", MAIL_API_URL).as_str())
        .body(string)
        .send()
        .await?
        .text()
        .await?;

    log::debug!("Response from user creation: {}", res);
    Ok(serde_json::from_str(&res)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() -> Result<(), Error> {
        assert_eq!(
            create_email(&User::new("sd", "s"))
                .await?
                .address
                .as_str()
                .is_empty(),
            false
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_twenty() -> Result<(), Error> {
        let mut emails = vec![];
        for _ in 0..20 {
            let result = create_email(&User::new("", "")).await;
            let response = result?;
            let string = response.address;
            emails.push(string)
        }
        println!("{:?}", emails);
        assert_eq!(emails.len(), 20);
        Ok(())
    }
}
