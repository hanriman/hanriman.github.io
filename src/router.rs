use crate::components::module::{home::Home, projects::Projects, resume::Resume};
use yew::{html, Html};
use yew_router::prelude::Routable;

/// App routes
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Resume => html! {<Resume />},
        Route::Projects => html! {<Projects />},
        Route::NotFound => html! { "Page not found" },
    }
}
