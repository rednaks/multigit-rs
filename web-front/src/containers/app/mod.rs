use crate::components::navigation::Nav;
use crate::routes::{switch, Route};
use yew::html;
use yew::{function_component, Html};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Nav></Nav>
                <ybc::Container fluid=true>
                    <Switch<Route> render={switch}/>
                </ybc::Container>
            </BrowserRouter>
        </>
    }
}
