use crate::response::GithubDiffEntry;
use crate::users::response::User;
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Commit {
    pub sha: String,
    pub url: String,
    pub node_id: Option<String>,
    pub html_url: Option<String>,
    pub comments_url: Option<String>,
    pub commit: Option<Box<Commit>>,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub parents: Option<Vec<ParentCommit>>,
    pub stats: Option<CommitStats>,
    pub files: Option<Vec<GithubDiffEntry>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ParentCommit {
    pub sha: String,
    pub url: String,
    pub html_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommitStats {
    pub additions: u64,
    pub deletions: u64,
    pub total: u64,
}
