use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        // content
        <div class="frame resume-frame">
        // content box
            <div class="content-frame">
                <div class="content">
                    <h2>
                        {"About"}
                    </h2>
                    <p>
                        {"Currently working as Application Developer at IBM specialized in Data & Application.
                        I have worked on Data Warehouse, Backend Development, DevOps, 
                        Data Governance, and LLM Projects. Besides my main job, I also 
                        exploring Rust for web development and software 
                        development in general."}
                    </p>
                    <br/>
                    <h2>
                        {"Primary Toolkits"}
                    </h2>
                    <ol>
                        <li>
                            <p>
                                {"- Programming Language: Rust, Python, Golang"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Data Analytics: Pandas, Numpy, Scikit-learn, Tensorflow"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Web Dev: Axum, Yew, HTML5, CSS3"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Databases: PostgreSQL, MongoDB, Pgvector, ELK"}
                            </p>
                        </li>
                        <li>
                            <p>
                                {"- Development Tools: Git, Docker, OKD/K8s, IBM Cloud"}
                            </p>
                        </li>
                    </ol>
                    <br />
                    <h2>
                        {"Currently Reading"}
                    </h2>
                    <p>
                        {"Neural Networks and Deep Learning by Michael Nielsen"}
                    </p>
                    <br/>
                    <h2>
                        {"Download Resume "}
                        <span>
                            {"[ "}
                            <a
                                class="link-new-tab"
                                href="./assets/resume/hanifan-resume.pdf"
                                target="_blank"
                                aria-label="download resume"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            {"]"}
                        </span>
                    </h2>
                    <blockquote>
                        <i>
                            {"Updated on 25 Jul 2024"}
                        </i>
                    </blockquote>
                    <br/>
                    <p>
                        {"If you would like a full CV or would like me to elaborate on any
                        points, please send me an email at hanifanrizki@gmail.com."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
