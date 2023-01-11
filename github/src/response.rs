use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubRepo {
    pub id: i32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: GithubRepoOwner,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubRepoOwner {
    pub login: String,
    pub id: i32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gist_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_url: String,

    #[serde(rename = "type")]
    pub owner_type: String,
    pub site_admin: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubBranch {
    pub name: String,
    pub commit: GithubCommit,
    pub protected: bool,
    pub protection: GithubBranchProtection,
    pub protection_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubBranchProtection {
    pub required_status_checks: Value,
}
