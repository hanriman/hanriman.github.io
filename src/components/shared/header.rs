use crate::utils::router::AppRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>
                <span class="c2">{"Hanifan "}</span> <span>{"Rizki Nurahman"}</span>
            </h1>
            <p class="header-label">{"Software Engineer"}</p>
            <nav class="header-nav" aria-label="site menu">
                <ol>
                    <li>
                        <Link<AppRoute> to={AppRoute::Home}>
                            {"Home"}
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Resume}>
                            {"Resume"}
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            {"Projects"}
                        </Link<AppRoute>>
                    </li>
                </ol>
            </nav>
        </header>
    }
}
