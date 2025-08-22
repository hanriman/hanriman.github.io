use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <h2>
                {"Get in touch | "}
                <span class="email"> { "hanifanrizki@gmail.com" }</span>
            </h2>

            <div class="get-in-touch-frame">
                <div class="get-in-touch-container">
                    <ol>
                        <li>
                            <a
                                class="_text"
                                href="https://github.com/hanriman"
                                target="_blank"
                                >
                                {"Github "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://linkedin.com/in/hanriman"
                                target="_blank"
                                >
                                {"Linkedin "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://leetcode.com/hanriman"
                                target="_blank"
                                >
                                {"Leetcode "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://medium.com/@hanriman"
                                target="_blank"
                                >
                                {"Medium "}
                                <NewTabSVG />
                            </a>
                        </li>
                        <li>
                            <a
                                class="_text"
                                href="https://twitter.com/hanriman"
                                target="_blank"
                                >
                                {"Twitter "}
                                <NewTabSVG />
                            </a>
                        </li>
                    </ol>
                </div>
            </div>
        </footer>
    }
}
