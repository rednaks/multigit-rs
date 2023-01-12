use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CompareStatus {
    Ahead,
    Behind,
    Diverged,
    Identical,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommitsComparison {
    pub status: CompareStatus,
    // TODO
}
