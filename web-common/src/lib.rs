use serde::{Deserialize, Serialize};

// org
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Org {
    pub login: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrgRequest {
    pub login: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct OrgResponse {
    pub login: String,
}

impl OrgResponse {
    pub fn of(org: Org) -> OrgResponse {
        OrgResponse { login: org.login }
    }
}
