use crate::router::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        // header-->
        <header class="header">
            <h1 class="header-title">
                <span class="c1">{ "Hanifan Rizki " }</span> <span class="c2">{ "Nurahman" }</span>
            </h1>
            <p class="header-label">{ "Software Engineer" }</p>
            <nav class="header-nav" aria-label="site menu">
                <ol>
                    <li>
                        <div class="_dot">{ "●" }</div>
                        <a class="_text">
                            <Link<Route> to={Route::Home}>
                                { "Home" }
                            </Link<Route>>
                        </a>
                    </li>
                    <li>
                        <div class="_dot">{ "●" }</div>
                        <a class="_text">
                            <Link<Route> to={Route::Resume}>
                                { "Resume" }
                            </Link<Route>>
                        </a>
                    </li>
                    <li>
                        <div class="_dot">{ "●" }</div>
                        <a class="_text">
                            <Link<Route> to={Route::Projects}>
                                { "Projects" }
                            </Link<Route>>
                        </a>
                    </li>
                </ol>
            </nav>
        </header>
    }
}
