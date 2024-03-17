use crate::components::shared::{footer::Footer, header::Header, theme_button::ThemeButton};
use crate::router::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::{HashRouter, Switch};

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <main>
                // web frame
                <div class="site-frame">
                    <section class="header-main-section">
                        <Header />
                        <Switch<Route> render={switch} />
                    </section>

                    <Footer />
                </div>

                <ThemeButton />

                // dark or light theme script
                <script rel="js" src="scripts/jquery.min.js"></script>
                <script rel="js" src="script-dd8916233d930b59.js"></script>
            </main>
        </HashRouter>
    }
}
