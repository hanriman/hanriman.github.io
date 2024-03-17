use yew::{function_component, html, Html};

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        // content-->
        <div class="main projects-frame">
            // content box-->
            <div class="content-frame">
                <p class="content"> {
                    "this is projects"
                }
                </p>
            </div>
        </div>
    }
}
