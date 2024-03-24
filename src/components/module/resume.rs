use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        // content
        <div class="main resume-frame">
        // content box
            <div class="content-frame">
                <div class="content">
                    { "Download Resume " }
                    <span>{"[ "}</span>
                    <a
                        class="_link-new-tab"
                        href="https://drive.google.com/file/d/1cFOMYojhK7rQpwDqr0TukBozEJ9jiIth/view?usp=sharing"
                        target="_blank"
                        >{ "link " }
                        <NewTabSVG />
                        </a>
                    <span>{"]"}</span>
                    <br /><br />
                    { "Primary Toolkits:" }
                    <br />
                    <span class="bold">{ "- Programming Language" }</span>
                    { ": Python, Rust" }
                    <br />
                    <span class="bold">{ "- Data Analytics" }</span>
                    { ": Pandas, Numpy, Scikit-learn, Tensorflow" }
                    <br />
                    <span class="bold">{ "- Backend" }</span>
                    { ": Rust (Axum)" }
                    <br />
                    <span class="bold">{ "- Frontend" }</span>
                    { ": HTML5, CSS3, Rust (Yew)" }
                    <br />
                    <span class="bold">{ "- Databases" }</span>
                    { ": PostgreSQL, MongoDB,
                    CloudantDB, ELK" }
                    <br />
                    <span class="bold">{ "- Development Tools" }</span>
                    { ": Git, Docker, OKD/K8s, IBM Cloud" }
                    <br /><br />
                    { "Currently Reading:" }
                    <br />
                    { "Software Engineering at Google" }
                    <br /><br />
                    { "Hobby:" }
                    <br />
                    { "- Calisthenics" }
                    <br />
                    { "- Meditation" }
                    <br /><br />
                    { "If you would like a full CV or would like me to elaborate on any
                    points, please send me an email at hanifanrizki@gmail.com" }
                </div>
            </div>
        </div>
    }
}
