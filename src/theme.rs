use dioxus::prelude::document;

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

    /// Aplica el tema manipulando la clase `dark` del `<html>`.
    /// Usa `document::eval` de Dioxus, que funciona tanto en web (wasm32)
    /// como en Android/desktop (WebView nativa).
    pub fn apply(&self) {
        let js = match self {
            Theme::Dark => "document.documentElement.classList.add('dark');",
            Theme::Light => "document.documentElement.classList.remove('dark');",
        };
        let _ = document::eval(js);
    }

    /// Lee el tema guardado.
    /// En wasm32 lo lee de forma síncrona via web_sys.
    /// En Android/desktop devuelve Light (el componente App lo corrige
    /// de forma asíncrona con `use_effect` + `document::eval`).
    pub fn from_storage() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
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
        }
        Theme::Light
    }

    /// Persiste el tema en localStorage via `document::eval` (funciona en
    /// wasm32 y en Android/desktop WebView).
    pub fn save_to_storage(&self) {
        let theme_str = match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        };
        let js = format!("localStorage.setItem('theme', '{}');", theme_str);
        let _ = document::eval(&js);
    }
}
