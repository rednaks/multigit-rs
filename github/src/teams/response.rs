use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Team {
    pub id: u32,
    pub node_id: String,
    pub name: String,
    pub slug: String,
    pub ldap_dn: Option<String>,
    pub description: Option<String>,
    pub privacy: String,
    pub permissions: TeamPermission,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    pub parent: Option<Box<Team>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TeamPermission {
    pub pull: bool,
    pub triage: bool,
    pub push: bool,
    pub maintain: bool,
    pub admin: bool,
}
