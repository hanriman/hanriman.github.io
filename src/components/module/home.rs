use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="main home-frame">
            <div class="content-frame">
                <div class="content">
                    { "Hi, I'm Hanifan Rizki Nurahman."}
                    <br /><br />
                    {"Lead Data and AI Services at IBM-JTI and a Statistics graduate
                    from the Padjadjaran University. My main focus these days is explore 
                    the possibility of generative AI and Rust web development with Axum
                    and Yew."}
                    <br /><br />
                    {"In my free time you'll find me grinding my programming skills at
                    Leetcode, or write some blogs article to add some internet
                    footprint in this vast and crowded world."}
                    <br /><br />
                    {"Check out my resume for more details."}
                </div>
            </div>
        </div>
    }
}
