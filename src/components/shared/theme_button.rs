use yew::{function_component, html, Html};

#[function_component(ThemeButton)]
pub fn theme_button() -> Html {
    html! {
        <div class="toggle fa fa-moon-o">
            <div class="toggle-text">
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
