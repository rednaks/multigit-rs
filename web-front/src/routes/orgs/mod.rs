use crate::store::AppStore;
use gloo_net::http::Request;
use stylist::style;
use stylist::yew::styled_component;
use web_common::Repo;
use yew::html;
use yew::prelude::*;
use yew::Html;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ManageOrgProps {
    pub org: String,
}

#[styled_component]
pub fn ManageOrgs(props: &ManageOrgProps) -> Html {
    let (app_state, dispatch) = use_store::<AppStore>();
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
