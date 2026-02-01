use dioxus::prelude::*;
use web_sys::{window, wasm_bindgen::JsCast};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn apply(&self) {
        web_sys::console::log_1(&format!("Aplicando tema: {:?}", self).into());
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.document_element() {
                    if let Ok(html) = element.dyn_into::<web_sys::HtmlElement>() {
                        let class_list = html.class_list();
                        match self {
                            Theme::Dark => {
                                let _ = class_list.add_1("dark");
                                web_sys::console::log_1(&"Clase dark añadida".into());
                            }
                            Theme::Light => {
                                let _ = class_list.remove_1("dark");
                                web_sys::console::log_1(&"Clase dark removida".into());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn from_storage() -> Self {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("theme") {
                    return match theme.as_str() {
                        "dark" => Theme::Dark,
                        _ => Theme::Light,
                    };
                }
            }
        }
        Theme::Light
    }

    pub fn save_to_storage(&self) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let theme_str = match self {
                    Theme::Dark => "dark",
                    Theme::Light => "light",
                };
                let _ = storage.set_item("theme", theme_str);
            }
        }
    }
}
