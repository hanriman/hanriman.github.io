use crate::components::shared::back_project_button::BackProjectButton;
use crate::components::shared::svg::new_tab_svg::NewTabSVG;
use yew::{function_component, html, Html};

#[function_component(PersonalWebsiteWithRust)]
pub fn personal_website_with_rust() -> Html {
    html! {
        <div class="frame projects-frame">
            <div class="content-frame">
                <div class="content">
                    <BackProjectButton />
                    <div>
                        <h2 class="title">{"Personal Website with Rust"}</h2>
                        <br />
                        <h3>{"Code Repository [ "}
                            <a
                                class="link-new-tab"
                                href="https://github.com/hanifanrn/hanifanrn.github.io"
                                target="_blank"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            <span>{"]."}</span>
                        </h3>
                        <br />
                        <p>
                            {"I love something "}
                            <strong>{"sophisticated"}</strong>
                            {", something that mindfully crafted with a well-thought-out design.
                            But when I reflect on some of personal projects that I have done in the past
                            few years, I feel that my works are not "} 
                            <i>{"sophisticated enough"}</i>
                            {". there is a tendency to just move to a new project before the
                            current project is ready to be released or else I just 
                            release it without really thinking about the quality. 
                            Those sentences contradict each other and the moment I realized
                            it, it got me thinking:"}
                        </p>
                        <br />
                        <blockquote>
                            <i>
                                {"“I want to create a personal website with a well-thought-out design.
                                A website that can contain my other projects. Something that 
                                I can feel "} <strong>{"statisfied"}</strong> {" with.”"}
                            </i>
                            <p>
                                {"- Hanifan 2023"}
                            </p>
                        </blockquote>
                        <br />
                        <p>
                            {"The motivation is set up, and the next thing are doing some research on
                            how the website should be designed. I don't want my website to be "}
                            <strong>{"centrally"}</strong>
                            {" oriented. To avoid that, inspired by nature, I use Golden Ratio and
                            Fibonacci number as my padding, margin, font size and even the RGB color number. 
                            So that even if the contents placement are not centrally oriented,
                            the overall contents placement still has structure."}
                        </p>
                        <br />
                        <p>
                            {"As for the tech stack, previously in 2023 I only use html, css and js
                            for static website that get deployed on github pages because I still 
                            don’t know how to use Rust and after around a year of learning and 
                            exploring Rust frontend framework and webassembly I decided to rewrite 
                            my website using Leptos."} 
                        </p>
                        <div class="image-container">
                            <img
                                src="./assets/personal_website_with_rust/rewrite_to_rust_meme.jpeg"
                                alt="why should not I rewrite it to rust meme"
                                class="rewrite-to-rust-meme"
                            />
                        </div>
                        <p>
                            {"Just to find out that Leptos still not yet support
                            Hash Router for static website in Mar 2024 "}
                            <span>{"[ "}</span>
                            <a
                                class="link-new-tab"
                                href="https://github.com/leptos-rs/leptos/issues/2184"
                                target="_blank"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            <span>{"]"}</span>
                            {". I need hash router so that the
                            website won't return 404 when the pages refreshed or accessed directly to a 
                            specific route besides its base route. More detail about the problem in 
                            Stack Overflow that also happen with React "}
                            <span>{"[ "}</span>
                            <a
                                class="link-new-tab"
                                href="https://stackoverflow.com/questions/71984401/react-router-not-working-with-github-pages"
                                target="_blank"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            <span>{"]."}</span>
                        </p>
                        <br />
                        <p>
                            {"So after doing some more research I rewrite my website again using Yew
                            because yew-router already support hash router functionality and works fine with 
                            github pages. Finally after years of learning, designing, and debugging
                            I can feel "} 
                            <strong>{"satisfied"}</strong>
                            {" with this project."}
                        </p>
                        <br />
                        <h3>{"Errata Policy"}</h3>
                        <p>
                            {"I am trying my best to make this website as error-free as possible.
                            If you find any errors, please let me know by open an issue on GitHub [ "}
                            <a
                                class="link-new-tab"
                                href="https://github.com/hanifanrn/hanifanrn.github.io/issues"
                                target="_blank"
                                >
                                {"link "}
                                <NewTabSVG />
                            </a>
                            <span>{"]."}</span>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
