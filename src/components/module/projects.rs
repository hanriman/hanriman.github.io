use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use crate::utils::router::AppRoute;
use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="frame projects-frame">
            <div class="content-frame">
                <div class="content">
                    <section class="featured-projects">
                        <h2>{"Featured Projects"}</h2>
                        <ol>
                            <li>
                                <span>{"Apr 2024 | "}</span>
                                <Link<AppRoute> classes={classes!("link")} to={AppRoute::PersonalWebsiteWithRust}>
                                    {"Personal Website with Rust"}
                                </Link<AppRoute>>
                            </li>
                            <li>
                                <span>{"Mar 2024 | "}</span>
                                <Link<AppRoute> classes={classes!("link")} to={AppRoute::TakeADeepBreath}>
                                    {"Take a Deep Breath"}
                                </Link<AppRoute>>
                            </li>
                        </ol>
                    </section>
                    <section class="publications">
                        <h2>{"Publications"}</h2>
                        <ol>
                            <li>
                                <div class="publication">
                                    <a
                                        class="link-new-tab"
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
                                        class="link-new-tab"
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
                    </section>
                    <section class="featured-on">
                        <h2>{"Featured on"}</h2>
                        <ol>
                            <li>
                                <span>{"Feb 2024 | "}</span>
                                <a
                                    class="link-new-tab"
                                    href="https://blog.bangkit.academy/2024/02/the-survival-drive-can-always-be-best.html"
                                    target="_blank"
                                    >
                                    {"Bangkit Academy Blog "}
                                    <NewTabSVG />
                                </a>
                            </li>
                            <li>
                                <span>{"Feb 2024 | "}</span>
                                <a
                                    class="link-new-tab"
                                    href="https://www.instagram.com/p/C3o9yqByUGd/?img_index=1"
                                    target="_blank"
                                    >
                                    {"Google Indonesia Instagram "}
                                    <NewTabSVG />
                                </a>
                            </li>
                        </ol>
                    </section>
                </div>
            </div>
        </div>
    }
}
