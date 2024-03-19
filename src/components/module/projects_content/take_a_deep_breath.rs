use crate::components::shared::back_project_button::BackProjectButton;
use yew::{function_component, html, Html};

#[function_component(TakeADeepBreath)]
pub fn take_a_deep_breath() -> Html {
    html! {
        <div class="main projects-frame">
            <div class="content-frame">
                <div class="content">
                    <BackProjectButton />
                    <div class="blob">
                        {"Take a deep breath"}
                    </div>
                </div>
            </div>
        </div>
    }
}
