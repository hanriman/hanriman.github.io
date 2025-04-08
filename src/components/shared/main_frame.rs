use crate::utils::router::{switch, AppRoute};
use yew::{function_component, html, Html};
use yew_router::{HashRouter, Switch};

/// The root app component
#[function_component(MainFrame)]
pub fn main_frame() -> Html {
    html! {
        <HashRouter>
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}
