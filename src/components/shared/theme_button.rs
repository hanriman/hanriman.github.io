use crate::components::shared::svg::themes_svg::{Dark, Light};
use crate::utils::theme::{AppContext, ThemeAction};
use yew::prelude::*;

#[function_component(ThemeButton)]
pub fn theme_button() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    let cycle_theme = {
        let app_context = app_context.clone();
        let current_theme: &str = app_context.theme.current;
        let current_theme_index: usize = app_context
            .theme_cycle
            .iter()
            .position(|x: &&str| x == &current_theme)
            .unwrap_or(0);
        let nth_theme = app_context.theme_cycle.get(current_theme_index + 1);
        let next_theme: &str = match nth_theme {
            Some(nt) => nt,
            None => "light",
        };
        Callback::from(move |_| match next_theme {
            "light" => app_context.theme.dispatch(ThemeAction::Light),
            "dark" => app_context.theme.dispatch(ThemeAction::Dark),
            _ => app_context.theme.dispatch(ThemeAction::Light),
        })
    };

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "dark" => html! {<Dark/>},
            "light" => html! {<Light/>},
            _ => html! {<Dark/>},
        }
    }

    html! {
            <div role="button" aria-lable="dark or light theme button" class="toggle" onclick={ cycle_theme }>
                <div aria-hidden="true" class="toggle-text">
                    { handle_theme_icon(app_context.clone()) }
                    <div class="_theme-1"></div>
                    <div class="_theme-2"></div>
                    <div class="_theme-3"></div>
                    <div class="_theme-4"></div>
                    <div class="_theme-5"></div>
                    <div class="_theme-6"></div>
                    <div class="_theme-7"></div>
                    <div class="_theme-8"></div>
                    <div class="_theme-9"></div>
                    <div class="_theme-10"></div>
                </div>
            </div>
    }
}
