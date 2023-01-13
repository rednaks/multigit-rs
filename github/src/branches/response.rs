use crate::response::GithubCommit;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Branch {
    pub name: String,
    pub commit: GithubCommit,
    pub protected: bool,
    pub protection: Option<BranchProtection>,
    pub protection_url: Option<String>,
    pub pattern: Option<String>,
    pub _links: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BranchProtection {
    pub required_status_checks: Value,
}
