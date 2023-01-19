mod containers;
use containers::app::App;
mod components;
mod routes;
mod store;

fn main() {
    yew::Renderer::<App>::new().render();
}
