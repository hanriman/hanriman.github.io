use crate::components::shared::{footer::Footer, header::Header, theme_button::ThemeButton};
use crate::router::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <body>

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
                <script src="scripts/jquery.min.js"></script>
                <script src="scripts/script.js"></script>
            </body>
        </HashRouter>
    }
}
