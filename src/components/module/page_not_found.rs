use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        // content-->
        <div class="main projects-frame">
            // content box-->
            <div class="content-frame">
                {"Page Not Found"}
            </div>
        </div>
    }
}
