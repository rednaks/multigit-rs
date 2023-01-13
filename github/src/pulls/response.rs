use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::response::{GithubGitPointer, GithubLabel, GithubMilestone};
use crate::teams::response::Team;
use crate::users::response::{User, UserAssociation};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GithubPullRequestState {
    Open,
    Closed,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PullRequest {
    pub id: u32,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub issue_url: String,
    pub patch_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: u64,
    pub state: GithubPullRequestState,
    pub locked: bool,
    pub title: String,
    pub user: Option<User>,
    pub body: Option<String>,
    pub labels: Vec<GithubLabel>,
    pub milestone: Option<GithubMilestone>,
    pub active_lock_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub merged_at: Option<DateTime<Utc>>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<User>,
    pub assignees: Option<Vec<User>>,
    pub requested_reviewers: Option<Vec<User>>,
    pub requested_teams: Option<Vec<Team>>,
    pub head: GithubGitPointer,
    pub base: GithubGitPointer,
    pub _links: Value, // TODO: links type ?
    pub author_association: UserAssociation,
    pub auto_merge: Option<AutoMergeObject>,
    pub draft: bool,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<User>,
    pub comments: Option<u16>,
    pub review_comments: Option<u16>,
    pub maintainer_can_modify: Option<bool>,
    pub additions: Option<u32>,
    pub deletion: Option<u32>,
    pub changed_files: Option<u16>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AutoMergeObject {
    pub enabled_by: User,
    pub merge_method: MergeMethod,
    pub commit_title: String,
    pub commit_message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PullRequestMergeStatus {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}
