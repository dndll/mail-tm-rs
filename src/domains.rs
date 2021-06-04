use anyhow::Error;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{http, MAIL_API_URL};
use crate::http::Client;
use crate::hydra::{Search, View, HydraCollection};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    #[serde(rename = "id")]
    pub id2: String,
    pub domain: String,
    pub is_active: bool,
    pub is_private: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl HydraCollection<Domain> {
    pub fn as_list(&self) -> Vec<String> {
        self.members.iter().map(|domain| domain.domain.to_owned()).collect()
    }
}


// TODO memoise me for some time
pub(crate) async fn domains() -> Result<HydraCollection<Domain>, Error> {
    let client = Client::new()?.build()?;

    log::debug!("Getting domains");

    let response = client
        .get(&format!("{}/domains", MAIL_API_URL))
        .send()
        .await?;

    let code = response.status();

    let response = response
        .text()
        .await?;

    http::check_response_status(&code, &response).await?;

    log::trace!("Retrieved domains: {}", response);
    Ok(serde_json::from_str(&response)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_domains() -> Result<(), Error> {
        let domains = domains().await?;
        assert!(domains.total_items > 0);

        let first = domains.members.first().unwrap().clone();

        let domains = domains.as_list();

        assert!(domains.contains(&first.domain));
        Ok(())
    }
}