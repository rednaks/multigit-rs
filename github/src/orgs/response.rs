use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::response::GithubPlan;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Org {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub memebers_url: Option<String>,
    pub public_members_url: Option<String>,
    pub avatar_url: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub twitter_username: Option<String>,
    pub is_verified: Option<bool>,
    pub has_organization_projects: Option<bool>,
    pub has_repository_projects: Option<bool>,
    pub public_repos: Option<u64>,
    pub public_gists: Option<u64>,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub html_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub total_private_repos: Option<u64>,
    pub owned_private_repos: Option<u64>,

    #[serde(rename = "type")]
    pub org_type: Option<String>,
    pub private_gists: Option<u64>,
    pub disk_usage: Option<u64>,
    pub collaborators: Option<u64>,
    pub billing_email: Option<String>,
    pub plan: Option<GithubPlan>,
    pub default_repository_permission: Option<OrgRepoPermission>,
    pub members_can_create_repositories: Option<bool>,
    pub two_factors_requirement_enabled: Option<bool>,
    pub members_allowed_repository_creation_type: Option<RepoCreationType>,
    pub members_can_create_public_repositories: Option<bool>,
    pub members_can_create_private_repositories: Option<bool>,
    pub members_can_create_internal_repositories: Option<bool>,
    pub members_can_create_pages: Option<bool>,
    pub members_can_create_public_pages: Option<bool>,
    pub members_can_create_private_pages: Option<bool>,
    pub members_can_fork_private_pages: Option<bool>,
    pub web_commit_signoff_required: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
    pub advanced_security_enabled_for_new_repositories: Option<bool>,
    pub dependabot_security_updates_enabled_for_new_repositories: Option<bool>,
    pub dependency_graph_enabled_for_new_repositories: Option<bool>,
    pub secret_scanning_enabled_for_new_repositories: Option<bool>,
    pub secret_scanning_push_protection_enabled_for_new_repositories: Option<bool>,
    pub secret_scanning_push_protection_enabled_custom_link_enabled: Option<bool>,
    pub secret_scanning_push_protection_enabled_custom_link: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrgRepoPermission {
    Read,
    Write,
    Admin,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RepoCreationType {
    All,
    Private,
}
