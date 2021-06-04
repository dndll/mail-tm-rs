use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HydraCollection<T>  {
    #[serde(rename = "hydra:member")]
    pub members: Vec<T>,
    #[serde(rename = "hydra:totalItems")]
    pub total_items: i64,
    #[serde(rename = "hydra:view")]
    pub view: Option<View>,
    #[serde(rename = "hydra:search")]
    pub search: Option<Search>,
}

impl <T: Clone> HydraCollection<T> {
    pub fn any(&self) -> T {
        self.members[rand::thread_rng().gen_range(0..self.members.len())].clone()
    }
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

