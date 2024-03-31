use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use std::rc::Rc;
use web_sys::{MediaQueryList, Window};
use yew::prelude::*;

pub enum ThemeAction {
    Light,
    Dark,
}

#[derive(PartialEq, Debug)]
pub struct ThemeState {
    pub current: &'static str,
}

impl Default for ThemeState {
    fn default() -> Self {
        // Get the theme stored in local storage
        let ls_theme_option: Result<String, StorageError> = LocalStorage::get("theme");

        let ls_theme: &str = match &ls_theme_option {
            Ok(theme) => theme,
            _ => {
                let window: Window = web_sys::window().expect("No Window Object!");
                let match_media_result = window.match_media("(prefers-color-scheme: dark)"); // : Result<Option<MediaQueryList>, JsValue>
                match match_media_result {
                    Ok(match_media_option) => {
                        let match_media: MediaQueryList =
                            match_media_option.expect("MEDIAQUERYLIST NOT SUPPORTED!");
                        if match_media.matches() {
                            "dark"
                        } else {
                            "light"
                        }
                    }
                    _ => "dark",
                }
            }
        };

        match ls_theme {
            "dark" => Self { current: "dark" },
            "light" => Self { current: "light" },
            _ => Self { current: "dark" },
        }
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_theme = match action {
            ThemeAction::Dark => "dark",
            ThemeAction::Light => "light",
        };

        Self {
            current: next_theme,
        }
        .into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub theme: UseReducerHandle<ThemeState>,
    pub theme_cycle: Vec<&'static str>,
}
