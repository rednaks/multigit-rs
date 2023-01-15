use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::repos::response::Repo;
use crate::users::response::User;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubPlan {
    pub collaborators: Option<u32>,
    pub name: String,
    pub space: u32,
    pub private_repos: u32,
    pub filled_seats: Option<u32>,
    pub seats: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GithubDiffStatus {
    Added,
    Removed,
    Modified,
    Renamed,
    Copied,
    Changed,
    Unchanged,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubDiffEntry {
    pub sha: String,
    pub filename: String,
    pub status: GithubDiffStatus,
    pub additions: u64,
    pub deletions: u64,
    pub changes: u64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    pub patch: String,
    pub previous_filename: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GithubMilestoneState {
    Open,
    Closed,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubMilestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u32,
    pub node_id: String,
    pub number: u32,
    pub state: GithubMilestoneState,
    pub title: String,
    pub description: Option<String>,
    pub creator: Option<User>,
    pub open_issues: u16,
    pub closed_issues: u16,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub due_on: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubLabel {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String, // TODO: need validation
    pub default: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubGitPointer {
    pub label: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub repo: Option<Repo>,
    pub sha: String,
    pub user: User,
}
