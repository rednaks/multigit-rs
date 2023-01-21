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
                                        <ybc::Button classes={"card-footer-item"}>{"Manage"}</ybc::Button>
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
