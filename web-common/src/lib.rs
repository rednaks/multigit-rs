use serde::{Deserialize, Serialize};

// org
#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrgType {
    User,
    Organization,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Org {
    pub login: String,
    #[serde(rename = "type")]
    pub org_type: OrgType,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrgRequest {
    pub login: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrgResponse {
    pub login: String,
    #[serde(rename = "type")]
    pub org_type: OrgType,
}

impl OrgResponse {
    pub fn of(org: Org) -> OrgResponse {
        OrgResponse {
            login: org.login,
            org_type: org.org_type,
        }
    }
}
