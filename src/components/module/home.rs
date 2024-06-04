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
                        {"Data and AI Specialist at IBM and a Statistics graduate from
                        the Padjadjaran University. My main focus these days is to explore the 
                        possibility of generative AI and Rust web development with Axum and Yew."}
                    </p>
                    <br/>
                    <p>
                        {"In my free time, you will find me grinding my programming skills at
                        Leetcode, or writing some articles to add a little internet footprint in this 
                        vast and crowded world."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
