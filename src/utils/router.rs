use crate::components::module::{home::Home, projects::Projects, resume::Resume};
use yew::{html, Html};
use yew_router::prelude::Routable;

/// App routes
#[derive(Routable, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/resume")]
    Resume,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::Resume => html! {<Resume />},
        AppRoute::Projects => html! {<Projects />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
