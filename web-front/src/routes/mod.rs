use yew::html;
use yew::Html;

use yew_router::prelude::*;

mod home;
use home::Home as HomeRoute;

mod orgs;
use orgs::ManageOrgs;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/manage/org/:org")]
    ManageOrg { org: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomeRoute/ >},
        Route::ManageOrg { org } => html! { <ManageOrgs {org}/>},
        Route::NotFound => html! { <h1> {"404"}</h1>},
    }
}
