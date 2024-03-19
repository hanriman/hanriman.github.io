use crate::components::module::{
    home::Home,
    page_not_found::NotFound,
    projects::Projects,
    projects_content::{personal_website::PersonalWebsite, take_a_deep_breath::TakeADeepBreath},
    resume::Resume,
};
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
    #[at("/personal-website")]
    PersonalWebsite,
    #[at("/take-a-deep-breath")]
    TakeADeepBreath,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::Resume => html! {<Resume />},
        AppRoute::Projects => html! {<Projects />},
        AppRoute::PersonalWebsite => html! {<PersonalWebsite />},
        AppRoute::TakeADeepBreath => html! {<TakeADeepBreath />},
        AppRoute::NotFound => html! {<NotFound />},
    }
}
