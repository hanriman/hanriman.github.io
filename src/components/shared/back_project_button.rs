use crate::utils::router::AppRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(BackProjectButton)]
pub fn back_project_button() -> Html {
    html! {
        <button class="back-button">
            <Link<AppRoute> to={AppRoute::Projects}>
                { "< Back" }
            </Link<AppRoute>>
        </button>
    }
}
