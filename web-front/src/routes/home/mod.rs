use crate::containers::home::Home as HomeContainer;
use yew::prelude::*;
use yew::{html, Html};

#[function_component]
pub fn Home() -> Html {
    html! {
        <HomeContainer/>
    }
}
