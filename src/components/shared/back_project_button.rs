use crate::utils::router::AppRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(BackProjectButton)]
pub fn back_project_button() -> Html {
    html! {
        <Link<AppRoute> to={AppRoute::Projects}>
            <span class="_link">
                {"< Back"}
            </span>
        </Link<AppRoute>>
    }
}
