use crate::response::GithubPlan;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimerContributor,
    Mannequin,
    Member,
    None,
    Owner,
}
