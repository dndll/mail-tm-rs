use anyhow::{Error, Context};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, USER_AGENT as USER_AGENT_PARAM};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::email::{MAIL_API_URL, USER_AGENT};
use crate::email::EmailUser;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "@context")]
    pub context: String,
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
    pub cc: Vec<::serde_json::Value>,
    pub bcc: Vec<::serde_json::Value>,
    pub subject: String,
    pub seen: bool,
    pub flagged: bool,
    #[serde(rename = "verification_results")]
    pub verification_results: Vec<::serde_json::Value>,
    pub retention: bool,
    #[serde(rename = "retention_date")]
    pub retention_date: i64,
    pub text: String,
    pub html: Vec<String>,
    #[serde(rename = "has_attachments")]
    pub has_attachments: bool,
    pub attachments: Vec<::serde_json::Value>,
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

pub(crate) async fn inspect_email(id: String, token: &String) -> Result<Message, Error> {
    let client = reqwest::Client::builder();
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT_PARAM, USER_AGENT.parse().unwrap());
    header_map.insert("Origin", "https://mail.tm".parse().unwrap());
    header_map.insert("Referer", "https://mail.tm/en".parse().unwrap());
    header_map.insert("TE", "Trailers".parse().unwrap());
    header_map.insert(CONTENT_TYPE, "application/json;charset=utf-8".parse().unwrap()); //TODO memoize me
    header_map.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap()); //TODO memoize me

    let client = client.default_headers(header_map).build()?;

    let uri = format!("{}{}", MAIL_API_URL, id);
    let res = client.get(uri.as_str());

    let res = res.send()
        .await?;

    let res = res
        .text()
        .await?;

    Ok(serde_json::from_str(&res)?)
}

pub(crate) fn extract_link(message: Message) -> Result<String, Error> {
    let link: Vec<&str> = message.text.split("Email: ").collect();
    Ok(link.last().context("Failed to get the last part of the link")?.to_string())
}

pub(crate) async fn verify(link: &str) -> Result<bool, Error> {
    let client = reqwest::Client::builder();
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT_PARAM, USER_AGENT.parse().unwrap());
    header_map.insert("Origin", "https://discord.com".parse().unwrap());
    header_map.insert("Referer", "https://mail.tm/en".parse().unwrap());
    header_map.insert("Connection", "Keep-Alive".parse().unwrap());
    let client = client.default_headers(header_map).build()?;
    let res = client.get(link);
    let response = res.send().await?;
    let status = response.status();
    let body = response.text().await?;
    log::info!("Received body from verification {}", body);
    Ok(status.as_str() == "200")
}

#[cfg(test)]
mod tests {
    use crate::email::auth::get_token;
    use crate::email::list::list_messages;

    use super::*;
    use crate::user::User;

    #[tokio::test]
    async fn test_inspect_email() {
        let user = User::new();

        let token = get_token(&user).await.unwrap();
        let messages = list_messages(token.clone()).await.unwrap();
        let option = messages.hydra_member.first();
        let string = option.cloned().unwrap().id;
        let result = inspect_email(string, token.token).await;
        let message = result.unwrap();
        let x = message.subject.as_str();
        assert_eq!(x, "Dicks")
    }

    #[tokio::test]
    async fn test_extract_link() {
        let user = User::new();
        let token = get_token(&user).await.unwrap();
        let messages = list_messages(token.clone()).await.unwrap();
        let option = messages.hydra_member.first();
        let string = option.cloned().unwrap().id;
        let result = inspect_email(string, token.token).await;
        let message = result.unwrap();
        let link = extract_link(message);
        assert_eq!(link, "https://click.discord.com/ls/click?upn=qDOo8cnwIoKzt0aLL1cBeFE1RlVCKJFF5zAq8ml-2BFh1dq-2FeX22E9yMPFmLMSO5CYiXhp9YkD384yJYhq5wsezFhmc87h5D0tuuItagq0ug2xmbKhXO-2BSCoRC2t-2FujW1YBv-2FIKZ8vJeJSMJb2PQZlEoLTqLKklLUSoIN1D4HilF29pECfudDdGqGkwyQGpyvDLqKX0wLK42rvINpYIt4cZA-3D-3DqU1n_NyEJlP74kWzsE7McLQfgTaUKC7V3ZvWt7sET3kDzR1JioO9boTqIaISBDsiBMro3kPCdP9P6xMk98HyGTUpXrno2At8MHKd2-2BG5bDxOMj4icRrHs49otfrcyIHIRKTQGZQL5BdLHeRjdQfe-2B7YGKDpfqGyfuNCGbKCIw6Sr8TNEa1ioSqrITYNbup6xXcYcUVn4vEtffdVSORDoIAb-2B6bbksOn7K9IFUfsBSYYZy8XE-3D".to_string());
        assert_eq!(verify(link).await.unwrap(), true);
    }
}