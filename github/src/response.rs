use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubRepo {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: GithubUser,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubUser {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gist_url: Option<String>,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,

    #[serde(rename = "type")]
    pub owner_type: String, // TODO: maybe enum
    pub site_admin: bool,

    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: Option<u32>,
    pub public_gists: Option<u32>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub private_gists: Option<u32>,
    pub total_private_repos: Option<u32>,
    pub owned_private_repos: Option<u32>,
    pub disk_urage: Option<u64>,
    pub collaborators: Option<u16>,
    pub two_factor_authentication: Option<bool>,
    pub plan: Option<GithubPlan>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub business_plus: Option<bool>,
    pub ldap_dn: Option<String>,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubTeam {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub slug: String,
    pub ldap_dn: Option<String>,
    pub description: Option<String>,
    pub privacy: String,
    pub permissions: GithubTeamPermission,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    pub parent: Option<Box<GithubTeam>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubTeamPermission {
    pub pull: bool,
    pub triage: bool,
    pub push: bool,
    pub maintain: bool,
    pub admin: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GithubPlan {
    pub collaborators: u32,
    pub name: String,
    pub space: u32,
    pub private_repos: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubBranch {
    pub name: String,
    pub commit: GithubCommit,
    pub protected: bool,
    pub protection: Option<GithubBranchProtection>,
    pub protection_url: Option<String>,
    pub pattern: Option<String>,
    pub _links: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubParentCommit {
    pub sha: String,
    pub url: String,
    pub html_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubCommitStats {
    pub additions: u64,
    pub deletions: u64,
    pub total: u64,
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubCommit {
    pub sha: String,
    pub url: String,
    pub node_id: Option<String>,
    pub html_url: Option<String>,
    pub comments_url: Option<String>,
    pub commit: Option<Box<GithubCommit>>,
    pub author: Option<GithubUser>,
    pub committer: Option<GithubUser>,
    pub parents: Option<Vec<GithubParentCommit>>,
    pub stats: Option<GithubCommitStats>,
    pub files: Option<Vec<GithubDiffEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubBranchProtection {
    pub required_status_checks: Value,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GithubUserAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimerContributor,
    Mannequin,
    Member,
    None,
    Owner,
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
    pub creator: Option<GithubUser>,
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
    pub repo: Option<GithubRepo>,
    pub sha: String,
    pub user: GithubUser,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubDeleteReference {}
