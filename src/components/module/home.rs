use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="frame home-frame">
            <div class="content-frame">
                <div class="content">
                    <h2>
                        {"Hi, I'm Hanif."}
                    </h2>
                    <br/>
                    <p>
                        {"Application Developer at IBM and a Statistics graduate from
                        Universitas Padjadjaran. My main focus these days is to explore the 
                        possibility of generative AI and Rust web development with Axum and Yew."}
                    </p>
                    <br/>
                    <p>
                        {"In my free time, you will find me grinding my programming skills
                        using Rust of Haskell for personal projects, or writing some articles 
                        to add a little internet footprint in this vast and crowded world."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
