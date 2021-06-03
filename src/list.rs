use crate::MAIL_API_URL;
use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::http::Client;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMessages {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "hydra:member")]
    pub hydra_member: Vec<HydraMember>,
    #[serde(rename = "hydra:totalItems")]
    pub hydra_total_items: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HydraMember {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "id")]
    pub id2: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    pub msgid: String,
    pub from: From,
    pub to: Vec<To>,
    pub subject: String,
    pub intro: String,
    pub seen: bool,
    #[serde(rename = "has_attachments")]
    pub has_attachments: bool,
    #[serde(rename = "download_url")]
    pub download_url: String,
    pub size: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    pub address: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct To {
    pub address: String,
    pub name: String,
}

pub async fn list_messages(token: &String) -> Result<ListMessages, Error> {
    let client = Client::new()?.with_auth(&token)?.build()?;

    let res = client
        .get(format!("{}/messages", MAIL_API_URL).as_str())
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&res)?)
}

// TODO move me
fn contains_verification_email(messages: ListMessages) -> bool {
    messages
        .hydra_member
        .iter()
        .any(|member| member.from.address.contains("noreply@discord.com"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::get_token;
    use crate::user::User;

    #[tokio::test]
    async fn test_list_messages() -> Result<(), Error> {
        let user = User::default();
        let token = get_token(&user).await?;
        Ok(assert_eq!(
            list_messages(&token.token).await?.hydra_total_items,
            1
        ))
    }

    #[tokio::test]
    async fn test_contains_verification() -> Result<(), Error> {
        let user = User::default();
        let token = get_token(&user).await?;
        let messages = list_messages(&token.token).await?;
        Ok(assert_eq!(contains_verification_email(messages), false))
    }
}
