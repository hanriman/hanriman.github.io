use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="main home-frame">
            <div class="content-frame">
                <div class="content">
                    <h2>
                        {"Hi, I'm Hanif."}
                    </h2>
                    <br/>
                    <p>
                        {"Lead Data and AI Services at IBM-JTI and a Statistics graduate
                        from the Padjadjaran University. My main focus these days is explore 
                        the possibility of generative AI and Rust web development with Axum
                        and Yew."}
                    </p>
                    <br/>
                    <p>
                        {"In my free time you'll find me grinding my programming skills at
                        Leetcode, or write some blogs article to add some internet
                        footprint in this vast and crowded world."}
                    </p>
                    <br/>
                </div>
            </div>
        </div>
    }
}
