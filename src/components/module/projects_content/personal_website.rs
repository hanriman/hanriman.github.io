use crate::components::shared::back_project_button::BackProjectButton;
use yew::{function_component, html, Html};

#[function_component(PersonalWebsite)]
pub fn personal_website() -> Html {
    html! {
        <div class="main projects-frame">
            <div class="content-frame">
                <div class="content">
                    <BackProjectButton />
                    <div>
                        {"my personal website"}
                    </div>
                </div>
            </div>
        </div>
    }
}
