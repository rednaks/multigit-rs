use gloo_net::http::Request;
use stylist::yew::styled_component;
use stylist::{style, Style};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::html;
use yew::prelude::*;
use yew::Html;
use yew_router::prelude::*;
use yewdux::prelude::*;

use web_common::{Org, Repo};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/manage/:org")]
    ManageOrg { org: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
struct AppState {
    orgs: Option<Vec<Org>>,
    selected_org: Option<Org>,
}

#[function_component]
fn Menu() -> Html {
    let (app_state, dispatch) = use_store::<AppState>();

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
                                Ok(json_resp) => dispatch
                                    .reduce_mut(|app_state| app_state.orgs = Some(json_resp)),
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

#[derive(Properties, PartialEq)]
pub struct ManageOrgProps {
    pub org: String,
}

#[styled_component(ManageOrgComp)]
fn manage_org_comp(props: &ManageOrgProps) -> Html {
    let (app_state, dispatch) = use_store::<AppState>();
    let repos: UseStateHandle<Option<Vec<Repo>>> = use_state(|| None);

    {
        let repos = repos.clone();
        let dispatch = dispatch.clone();
        let org = props.org.clone();
        let capp_state = app_state.clone();
        use_effect_with_deps(
            move |_| {
                match capp_state.selected_org {
                    Some(_) => {}
                    None => wasm_bindgen_futures::spawn_local(async move {
                        let selected_org = Request::get(
                            format!("{}/api/orgs/{}", "http://127.0.0.1:8000", org).as_str(),
                        )
                        .send()
                        .await;

                        match selected_org {
                            Ok(response) => {
                                //
                                match response.json().await {
                                    Ok(an_org) => dispatch.reduce_mut(|app_state| {
                                        app_state.selected_org = Some(an_org)
                                    }),
                                    Err(_) => {
                                        return;
                                    }
                                }
                            }
                            Err(_) => {
                                return;
                            }
                        }
                    }),
                };

                if capp_state.selected_org.is_none() {
                    return;
                }

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_repos = Request::get(
                        format!(
                            "{}/api/orgs/{}/repos",
                            "http://127.0.0.1:8000",
                            capp_state.selected_org.clone().unwrap().login
                        )
                        .as_str(),
                    )
                    .query([(
                        "type",
                        capp_state
                            .selected_org
                            .clone()
                            .unwrap()
                            .org_type
                            .to_string(),
                    )])
                    .send()
                    .await;
                    match fetched_repos {
                        Ok(response) => match response.json::<Vec<Repo>>().await {
                            Ok(json_resp) => repos.set(Some(json_resp)),
                            Err(_) => {}
                        },
                        Err(_) => {}
                    };
                });
            },
            app_state,
        );
    }

    let build_repos = || -> Html {
        match repos.as_ref() {
            Some(repos) => repos
                .into_iter()
                .map(|repo| {
                    html! {
                        <div class={"repo"} value={repo.name.clone()}>{repo.name.clone()}</div>
                    }
                })
                .collect::<Html>(),
            None => html! {},
        }
    };

    let style = style! {
        r#"
        display: flex;
        .repo {
            margin: 10px;
        }
        "#
    };
    html! {
        <>
            {format!("Repos for org: {}", props.org)}
            <div class={style.unwrap()}>
            {build_repos()}
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Menu/> },
        Route::ManageOrg { org } => html! { <ManageOrgComp {org}/>},
        Route::NotFound => html! { <h1> {"404"}</h1>},
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
