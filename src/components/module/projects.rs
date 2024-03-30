use crate::components::shared::svg::new_tab_svg::NewTabSVG;
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
                    <div style="margin-bottom: 34px;">
                        <h2>{"Featured Projects"}</h2>
                        <ol>
                            <li>
                                <span>{"Apr 2024 | "}</span>
                                <Link<AppRoute> to={AppRoute::PersonalWebsite}>
                                    <a class="_link">{ "Personal Website with Rust" }</a>
                                </Link<AppRoute>>
                            </li>
                            <li>
                                <span>{"Mar 2024 | "}</span>
                                <Link<AppRoute> to={AppRoute::TakeADeepBreath}>
                                    <a class="_link">{ "Take a Deep Breath" }</a>
                                </Link<AppRoute>>
                            </li>
                        </ol>
                    </div>
                    <div>
                        <h2>{"Publications"}</h2>
                        <ol>
                            <li>
                                <div class="publication">
                                    <a
                                        class="_link-new-tab"
                                        href="https://iopscience.iop.org/article/10.1088/1742-6596/1776/1/012057/pdf"
                                        target="_blank"
                                        >
                                        {"Applied of feed-forward neural network and
                                        facebook prophet model for train passengers
                                        forecasting "}
                                        <NewTabSVG />
                                    </a>
                                    <span>{"IOP Publishing 2021"}</span>
                                    <p>
                                        {"RS Pontoh, S Zahroh, HR Nurahman, RI Aprillion, A Ramdani, DI Akmal"}
                                    </p>
                                </div>
                            </li>

                            <li>
                                <div class="publication">
                                    <a
                                        class="_link-new-tab"
                                        href="https://prosiding.statistics.unpad.ac.id/?journal=prosidingsns&page=article&op=view&path%5B%5D=61&path%5B%5D=69"
                                        target="_blank"
                                        >
                                        {"Analisis Data Longitudinal Incident Rate
                                        Pneumonia pada Balita di Kota Bandung 
                                        Menggunakan Generalized Estimating Equation "}
                                        <NewTabSVG />
                                    </a>
                                    <span>{"SNSO 2020"}</span>
                                    <p>
                                        {"HR Nurahman, A Bachrudin, B Tantular"}
                                    </p>
                                </div>
                            </li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
    }
}
