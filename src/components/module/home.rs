use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="frame home-frame">
            <div class="content-frame">
                <div class="content">
                    <h2>
                        <span>{"Hi, I'm "}</span>
                        <span class="c2">{"Han"}</span>
                        <span>{"."}</span>
                    </h2>
                    <br/>
                    <p>
                        {"Currently pursuing a Master's in Data Science at the University of Debrecen.
                        My main focus these days is exploring the possibilities of generative AI and 
                        deepening my knowledge in deep learning."}
                    </p>
                    <br/>
                    <p>
                        {"In my free time, you will find me sharpening my programming skills with Rust
                        through projects or writing articles to leave a small footprint in this vast and 
                        crowded digital world."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
