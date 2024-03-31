use crate::utils::router::AppRoute;
use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(BackProjectButton)]
pub fn back_project_button() -> Html {
    html! {
        <Link<AppRoute> classes={classes!("link")} to={AppRoute::Projects}>
            {"Back"}
        </Link<AppRoute>>
    }
}
