use std::vec;

use crate::components::ui::{Button, ButtonVariant, ButtonShape};
use crate::theme::Theme;
use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::{FiMoon, FiSun};
use dioxus_free_icons::Icon;

/// The Navbar layout wraps the app with a sticky navigation bar and a main
/// content area that fills the remaining vertical space. Using a flex column
/// container with `flex-1` on `main` prevents accidental vertical overflow
/// when combining a header and content with `min-h-screen` semantics.
#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<Theme>>();
    let mut show_skip = use_signal(|| false);
    let link_class = "text-sm font-medium text-zinc-600 dark:text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-zinc-100";

    rsx! {
        div { class: "min-h-screen flex flex-col bg-white dark:bg-zinc-950",
            // Skip link for keyboard users: hidden off-screen by default; shown when focused via events
            a {
                href: "#main-content",
                aria_label: "Saltar al contenido",
                onfocus: move |_| show_skip.set(true),
                onblur: move |_| show_skip.set(false),
                style: if show_skip() {
                    "position:absolute; left:8px; top:8px; z-index:50; padding:8px 12px; background:#ffffff; color:#111827; border-radius:6px; box-shadow:0 1px 3px rgba(0,0,0,0.1);"
                } else {
                    "position:absolute; left:-9999px;"
                },
                "Saltar al contenido"
            }
            nav { class: "sticky top-0 z-50 w-full border-b border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-950/80 backdrop-blur supports-[backdrop-filter]:bg-white/60 dark:supports-[backdrop-filter]:bg-zinc-950/60",
                div { class: "container mx-auto px-4 md:px-6 lg:px-8",
                    div { class: "flex h-16 items-center justify-between",
                        div { class: "flex flex-col md:flex-row place-items-baseline gap-x-8",
                            Link {
                                to: Route::Home {},
                                class: "text-lg font-bold text-zinc-950 dark:text-zinc-50 hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors",
                                "TF Sports"
                            }
                            div { class: "flex gap-6",
                                Link { to: Route::Events {}, class: "{link_class}", "Eventos" }
                                Link { to: Route::Sports {}, class: "{link_class}", "Deportes" }
                                Link { to: Route::Places {}, class: "{link_class}", "Instalaciones" }
                            }
                        }

                        // Theme toggle button
                        Button {
                            variant: ButtonVariant::Outline,
                            shape: ButtonShape::Default,
                            is_icon: Some(true),
                            onclick: move |_| {
                                let new_theme = theme().toggle();
                                new_theme.apply();
                                new_theme.save_to_storage();
                                theme.set(new_theme);
                            },
                            { if matches!(theme(), Theme::Dark) { rsx! {
                                Icon {
                                    class: "icon",
                                    icon: FiSun,
                                }
                            }} else { rsx! {
                                Icon {
                                    class: "icon",
                                    icon: FiMoon,
                                }
                            }} }
                        }
                    }
                }
            }

            main { id: "main-content", class: "flex-1 overflow-auto",
                Outlet::<Route> {}
            }
        }
    }
}
