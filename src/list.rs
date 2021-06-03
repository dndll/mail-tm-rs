use anyhow::Error;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, USER_AGENT as USER_AGENT_PARAM};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::email::{MAIL_API_URL, EmailUser, USER_AGENT};
use crate::email::auth::Token;
use crate::email::create::CreateResponse;

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
    let client = reqwest::Client::builder();
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT_PARAM, USER_AGENT.parse().unwrap());
    header_map.insert("Origin", "https://mail.tm".parse().unwrap());
    header_map.insert("Referer", "https://mail.tm/en".parse().unwrap());
    header_map.insert("TE", "Trailers".parse().unwrap());
    header_map.insert(CONTENT_TYPE, "application/json;charset=utf-8".parse().unwrap()); //TODO memoize me
    header_map.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap()); //TODO memoize me
    let client = client.default_headers(header_map).build()?;

    let res = client.get(format!("{}/messages", MAIL_API_URL).as_str())
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&res)?)
}

fn contains_verification_email(messages: ListMessages) -> bool {
    messages.hydra_member.iter().any(|member| member.from.address.contains("noreply@discord.com"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::email::auth::get_token;
    use crate::user::User;

    #[tokio::test]
    async fn test_list_messages() {
        let user = User::new();
        let token = get_token(&user).await.unwrap();
        assert_eq!(list_messages(token).await.unwrap().hydra_total_items, 1);
    }

    #[tokio::test]
    async fn test_contains_verification() {
        let user = User::new();
        let token = get_token(&user).await.unwrap();
        let messages = list_messages(token).await.unwrap();
        assert_eq!(contains_verification_email(messages), false);
    }
}