use crate::utils::router::AppRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::{use_route, Link};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        // header-->
        <header class="header">
            <h1 class="header-title">
                <span class="c2">{ "Hanifan " }</span> <span class="c1">{ "Rizki Nurahman" }</span>
            </h1>
            <p class="header-label">{ "Software Engineer" }</p>
            <nav class="header-nav" aria-label="site menu">
                <ol>
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}>
                            { "Home" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Resume}>
                            { "Resume" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            { "Projects" }
                        </Link<AppRoute>>
                    </li>
                </ol>
            </nav>
        </header>
    }
}
