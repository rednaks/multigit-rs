use gloo_net::{http::Request, Error};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::html;
use yew::prelude::*;
use yew::Html;
use yew_router::prelude::*;

use web_common::Org;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/manage/:login")]
    ManageOrg { login: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn Menu() -> Html {
    let selected_org: UseStateHandle<Option<String>> = use_state(|| None);
    let orgs: UseStateHandle<Option<Vec<Org>>> = use_state(|| None);

    {
        let orgs = orgs.clone();
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
                                Ok(json_resp) => orgs.set(Some(json_resp)),
                                Err(_) => {
                                    //
                                }
                            }
                        }
                        Err(_) => {}
                    }
                });
                || ()
            },
            (),
        );
    }

    let onchange = {
        let selected_org = selected_org.clone();
        Callback::from(move |evt: Event| {
            web_sys::console::log_1(&format!("{:?}", selected_org).into());
            web_sys::console::log_1(&format!("{:?}", evt).into());

            let target: Option<EventTarget> = evt.target();

            let select_element = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(sel) = select_element {
                selected_org.set(Some(sel.value()));
            } else {
                selected_org.set(None);
            }
        })
    };

    let build_select_options = || -> Html {
        match orgs.as_ref() {
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
            if let Some(org) = selected_org.as_ref() {
                html! {
                    <p>{"You are managing "} <strong>{org}</strong> {". Switch org:"}</p>
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
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Menu/> },
        Route::ManageOrg { login } => html! { {format!("Fetching repos for org: {}", login)}
            //
        },
        Route::NotFound => html! { <h1> {"404"}</h1>},
    }
}

//#[function_component(Main)]
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
