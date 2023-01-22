use crate::store::AppStore;
use yew::prelude::*;
use yew::{html, Html};
use yewdux::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let (app_state, _) = use_store::<AppStore>();

    let build_orgs_ui = || -> Html {
        match app_state.orgs.as_ref() {
            Some(orgs) => orgs
                .into_iter()
                .map(|org| {
                    html! {
                        <ybc::Column classes={"is-one-quarter"}>
                            <ybc::Card>
                                <ybc::CardContent>
                                    <ybc::Title>{org.login.clone()}</ybc::Title>
                                </ybc::CardContent>
                                <ybc::CardFooter>
                                        <a href={format!("/manage/org/{}", org.login.clone())} class={"card-footer-item button"}>
                                        {"Manage"}
                                        </a>
                                </ybc::CardFooter>
                            </ybc::Card>
                        </ybc::Column>
                    }
                })
                .collect::<Html>(),
            None => html! {
                <a class={"dropdown-item"}>{"loading"}</a>
            },
        }
    };
    html! {
        <>
            <ybc::Columns multiline={true}>
                {build_orgs_ui()}
            </ybc::Columns>
        </>
    }
}
