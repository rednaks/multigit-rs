use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteReference {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reference {
    #[serde(rename = "ref")]
    pub reference: String,
    pub node_id: String,
    pub url: String,
    pub object: ReferenceObject,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReferenceObject {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    pub sha: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReferenceType {
    Commit,
    Branch,
}
