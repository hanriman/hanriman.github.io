use yew::{function_component, html, Html};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        // content
        <main class="resume-frame">
        // content box
            <div class="content-frame">
                <p class="content">
                    { "Download Resume" }
                    <a
                        class="_link"
                        rel="copy-file"
                        href="./assets/Hanifan Rizki Nurahman.pdf"
                        target="_blank"
                        >{ "[link]" }</a
                    ><br /><br />

                    { "Primary Toolkits:" } <br />
                    <span class="bold">{ "- Programming Language" }</span>{ ": Python, Rust,
                    Javascript " }<br />

                    <span class="bold">{ "- Data Analytics" }</span>{ ": Pandas, Numpy,
                    Scikit-learn, Tensorflow" }
                    <br />

                    <span class="bold">{ "- Backend" }</span>{ ": Flask, Axum" }<br />

                    <span class="bold">{ "- Frontend" }</span>{ ": HTML5, CSS3, Javascript" }
                    <br />

                    <span class="bold">{ "- Databases" }</span>{ ": PostgreSQL, MongoDB,
                    CloudantDB, ELK" }<br />

                    <span class="bold">{ "- Development Tools" }</span>{ ": Git, Docker,
                    OKD/K8s, IBM Cloud" }
                    <br />
                    <br />
                    { "Currently Reading:" }
                    <br />

                    { "Software Engineering at Google" }
                    <br />
                    <br />

                    { "Hobby:" }
                    <br />
                    { "- Calisthenics" }
                    <br />
                    { "- Meditation" }
                    <br />
                    <br />
                    { "If you would like a full CV or would like me to elaborate on any
                    points, please send me an email at hanifanrizki@gmail.com" }

                </p>
            </div>
        </main>
    }
}
