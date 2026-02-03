use crate::Route;
use crate::theme::Theme;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
/// This layout component wraps the UI in a common navbar. The contents of the routes
/// will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<Theme>>();
    
    rsx! {
        nav { class: "sticky top-0 z-50 w-full border-b border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-950/80 backdrop-blur supports-[backdrop-filter]:bg-white/60 dark:supports-[backdrop-filter]:bg-zinc-950/60",
            div { class: "container mx-auto px-4 md:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    div { class: "flex items-center gap-8",
                        Link {
                            to: Route::Home {},
                            class: "text-lg font-bold text-zinc-950 dark:text-zinc-50 hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors",
                            "TF Sports"
                        }
                        div { class: "flex gap-6",
                            Link {
                                to: Route::Events {},
                                class: "text-sm font-medium text-zinc-600 dark:text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-zinc-100",
                                "Eventos"
                            }
                            Link {
                                to: Route::Sports {},
                                class: "text-sm font-medium text-zinc-600 dark:text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-zinc-100",
                                "Deportes"
                            }
                            Link {
                                to: Route::Places {},
                                class: "text-sm font-medium text-zinc-600 dark:text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-zinc-100",
                                "Instalaciones"
                            }
                        }
                    }

                    // Botón de tema
                    button {
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-950 dark:focus-visible:ring-zinc-300 h-10 w-10 hover:bg-zinc-100 dark:hover:bg-zinc-800",
                        onclick: move |_| {
                            web_sys::console::log_1(&format!("Tema actual: {:?}", theme()).into());
                            let new_theme = theme().toggle();
                            web_sys::console::log_1(&format!("Nuevo tema: {:?}", new_theme).into());
                            new_theme.apply();
                            new_theme.save_to_storage();
                            theme.set(new_theme);
                        },
                        if matches!(theme(), Theme::Dark) {
                            "☀️"
                        } else {
                            "🌙"
                        }
                    }
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout.
        div { class: "min-h-screen bg-white dark:bg-zinc-950", Outlet::<Route> {} }
    }
}
