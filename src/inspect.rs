use crate::http::{get_headers, Client};
use crate::{MAIL_API_URL};
use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

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
    let client = Client::new()?.with_auth(token)?.build()?;

    let uri = format!("{}{}", MAIL_API_URL, id);
    let res = client.get(uri.as_str());

    let res = res.send().await?;

    let res = res.text().await?;

    Ok(serde_json::from_str(&res)?)
}

// TODO move me
pub(crate) fn extract_link(message: Message) -> Result<String, Error> {
    let link: Vec<&str> = message.text.split("Email: ").collect();
    Ok(link
        .last()
        .context("Failed to get the last part of the link")?
        .to_string())
}

// TODO move me
pub(crate) async fn verify(link: &str) -> Result<bool, Error> {
    let client = reqwest::Client::builder();
    let mut header_map = get_headers()?;
    header_map.insert("Connection", "Keep-Alive".parse()?); // TODO dont think this is needed
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
    use super::*;
    use crate::auth::get_token;
    use crate::list::list_messages;
    use crate::user::User;

    #[tokio::test]
    async fn test_inspect_email() -> Result<(), Error> {
        let user = User::default();

        let token = get_token(&user).await?;
        let messages = list_messages(&token.token).await?;
        let option = messages.hydra_member.first();
        let string = option.cloned().unwrap().id;
        let result = inspect_email(string, &token.token).await;
        let message = result?;
        let x = message.subject.as_str();
        Ok(assert_eq!(x, "Dicks"))
    }

    #[tokio::test]
    async fn test_extract_link() -> Result<(), Error> {
        let user = User::default();
        let token = get_token(&user).await?;
        let messages = list_messages(&token.token).await?;
        let option = messages.hydra_member.first();
        let string = option.cloned().unwrap().id;
        let result = inspect_email(string, &token.token).await;
        let message = result?;
        let link = extract_link(message)?;
        assert_eq!(link, "https://click.discord.com/ls/click?upn=qDOo8cnwIoKzt0aLL1cBeFE1RlVCKJFF5zAq8ml-2BFh1dq-2FeX22E9yMPFmLMSO5CYiXhp9YkD384yJYhq5wsezFhmc87h5D0tuuItagq0ug2xmbKhXO-2BSCoRC2t-2FujW1YBv-2FIKZ8vJeJSMJb2PQZlEoLTqLKklLUSoIN1D4HilF29pECfudDdGqGkwyQGpyvDLqKX0wLK42rvINpYIt4cZA-3D-3DqU1n_NyEJlP74kWzsE7McLQfgTaUKC7V3ZvWt7sET3kDzR1JioO9boTqIaISBDsiBMro3kPCdP9P6xMk98HyGTUpXrno2At8MHKd2-2BG5bDxOMj4icRrHs49otfrcyIHIRKTQGZQL5BdLHeRjdQfe-2B7YGKDpfqGyfuNCGbKCIw6Sr8TNEa1ioSqrITYNbup6xXcYcUVn4vEtffdVSORDoIAb-2B6bbksOn7K9IFUfsBSYYZy8XE-3D".to_string());
        Ok(assert_eq!(verify(&link).await?, true))
    }
}
