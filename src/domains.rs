use crate::http::Client;
use crate::{MAIL_API_URL, http};
use serde::{Deserialize, Serialize};
use anyhow::Error;
use rand::Rng;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domains {
    #[serde(rename = "hydra:member")]
    pub domains: Vec<Domain>,
    #[serde(rename = "hydra:totalItems")]
    pub total_items: i64,
    #[serde(rename = "hydra:view")]
    pub view: Option<View>,
    #[serde(rename = "hydra:search")]
    pub search: Option<Search>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "hydra:first")]
    pub first: String,
    #[serde(rename = "hydra:last")]
    pub last: String,
    #[serde(rename = "hydra:next")]
    pub next: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "hydra:template")]
    pub template: String,
    #[serde(rename = "hydra:variableRepresentation")]
    pub variable_representation: String,
    #[serde(rename = "hydra:mapping")]
    pub mapping: Vec<Mapping>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub variable: String,
    pub property: String,
    pub required: bool,
}

impl Domains {
    pub fn as_list(&self) -> Vec<String> {
        self.domains.iter().map(|domain| domain.domain.to_owned()).collect()
    }

    pub fn any(&self) -> Domain {
        self.domains[rand::thread_rng().gen_range(0..self.domains.len())].clone()
    }
}


// TODO memoise me
pub async fn domains() -> Result<Domains, Error> {
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