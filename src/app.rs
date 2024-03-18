use crate::components::shared::{footer::Footer, header::Header, theme_button::ThemeButton};
use crate::utils::{
    router::{switch, AppRoute},
    theme::{AppContext, ThemeState},
};
use gloo_console::{error as console_error, log as console_log};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    let theme: UseReducerHandle<ThemeState> = use_reducer(ThemeState::default);
    let theme_cycle: Vec<&str> = vec!["light", "dark"];

    {
        /* Set the local storage to the current theme (& if it changes) */
        let theme: UseReducerHandle<ThemeState> = theme.clone();
        use_effect_with(
            theme.clone(),
            move |theme: &UseReducerHandle<ThemeState>| {
                match LocalStorage::set("theme", &theme.current) {
					Ok(()) => console_log!(format!("Theme set to {}", &theme.current)),
					_ => console_error!("Couldn't set LocalStorage. Please turn the feature in your Browser on if possible."),
	    		};
                || ()
            },
        )
    }

    html! {
    <ContextProvider<AppContext> context={AppContext {
        theme: theme.clone(),
        theme_cycle: theme_cycle
    }}>
        <HashRouter>
            <main class={theme.current}>
                <div class="site-frame">
                    <section class="header-main-section">
                        <Header />
                        <Switch<AppRoute> render={switch} />
                    </section>
                    <Footer />
                </div>

                <ThemeButton />
            </main>
        </HashRouter>
    </ContextProvider<AppContext>>
    }
}
