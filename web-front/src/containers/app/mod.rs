use crate::components::navigation::Nav;
use crate::routes::{switch, Route};
use yew::html;
use yew::{function_component, Html};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{"MultiGitRs"}</h1>
            <BrowserRouter>
                <Nav></Nav>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </>
    }
}
