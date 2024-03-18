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
        let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
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
            "dark" => html! {<Dark class={Some("h-[1.5rem] w-[1.5rem] fill-slate-300")} />},
            "light" | _ => html! {<Light class={Some("h-[1.5rem] w-[1.5rem] fill-orange-400")} />},
        }
    }

    html! {
            <div class="toggle" onclick={ cycle_theme }>
                <div class="toggle-text">
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
