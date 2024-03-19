use crate::utils::router::AppRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        // content-->
        <div class="main projects-frame">
            // content box-->
            <div class="content-frame">
                <div class="content">
                    <ol>
                        <li>
                            <a>{"Mar 2023 | "}</a>
                            <Link<AppRoute> to={AppRoute::TakeADeepBreath}>
                                <a class="_link">{ "Take a Deep Breath" }</a>
                            </Link<AppRoute>>
                        </li>
                        <li>
                            <a>{"Mar 2023 | "}</a>
                            <Link<AppRoute> to={AppRoute::PersonalWebsite}>
                                <a class="_link">{ "Personal Website" }</a>
                            </Link<AppRoute>>
                        </li>
                    </ol>
                </div>
            </div>
        </div>
    }
}
