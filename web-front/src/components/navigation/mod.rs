use crate::routes::Route;
use crate::store::AppStore;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_common::Org;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();
    let navigator = use_navigator().unwrap();

    {
        let dispatch = dispatch.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_orgs =
                        Request::get(format!("{}/api/orgs", "http://127.0.0.1:8000").as_str())
                            .send()
                            .await;

                    match fetched_orgs {
                        Ok(response) => {
                            let json = response.json::<Vec<Org>>().await;
                            match json {
                                Ok(json_resp) => {
                                    dispatch.reduce_mut(|store| store.orgs = Some(json_resp));
                                }
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    };
                });
            },
            (),
        );
    }

    let onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |evt: Event| {
            let target: Option<EventTarget> = evt.target();

            let select_element = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(sel) = select_element {
                dispatch.reduce_mut(|app_state| {
                    app_state.selected_org = app_state
                        .orgs
                        .clone()
                        .unwrap()
                        .into_iter()
                        .find(|o| o.login == sel.value());
                });

                navigator.push(&Route::ManageOrg { org: sel.value() })
            } else {
                dispatch.reduce_mut(|app_state| app_state.selected_org = None);
            }
        })
    };

    let build_select_options = || -> Html {
        match app_state.orgs.as_ref() {
            Some(orgs) => orgs
                .into_iter()
                .map(|org| {
                    html! {
                        <option value={org.login.clone()}>{org.login.clone()}</option>
                    }
                })
                .collect::<Html>(),
            None => html! {},
        }
    };

    html! {
    <>
        {
            if let Some(org) = app_state.selected_org.clone() {
                html! {
                    <p>{"You are managing "} <strong>{org.login}</strong> {". Switch org:"}</p>
                }
            } else {
                html! {
                    <p>{ "Select an org" }</p>
                }
            }
        }
        <select {onchange}>
         {build_select_options()}
        </select>
    </>
    }
}
