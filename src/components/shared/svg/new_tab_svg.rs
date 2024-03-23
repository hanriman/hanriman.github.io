use yew::{function_component, html, Html};

#[function_component(NewTabSVG)]
pub fn new_tab_svg() -> Html {
    html! {
        <svg
            aria-hidden="true"
            width="13"
            height="13"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M2.07102 11.3494L0.963068 10.2415L9.2017 1.98864H2.83807L2.85227 0.454545H11.8438V9.46023H10.2955L10.3097 3.09659L2.07102 11.3494Z"
                fill="currentColor"
            ></path>
        </svg>
    }
}
