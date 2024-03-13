use crate::router::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

/// The root app component
#[function_component(MainFrame)]
pub fn main_frame() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}
