use web_common::Org;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct AppStore {
    pub orgs: Option<Vec<Org>>,
    pub selected_org: Option<Org>,
}
