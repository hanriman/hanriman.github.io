use crate::components::shared::back_project_button::BackProjectButton;
use yew::{function_component, html, Html};

#[function_component(TakeADeepBreath)]
pub fn take_a_deep_breath() -> Html {
    html! {
        <div class="frame projects-frame">
            <div class="content-frame">
                <div class="content">
                    <BackProjectButton />
                    <div class="blob-container">
                        <div class="blob">
                            <p>
                                {"Take a deep breath"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
