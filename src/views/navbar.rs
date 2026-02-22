use crate::components::ui::{Button, ButtonShape, ButtonVariant};
use crate::theme::Theme;
use crate::Route;
use dioxus::prelude::*;
#[cfg(any(target_os = "android", target_os = "ios"))]
use dioxus_free_icons::icons::fi_icons::{FiActivity, FiCalendar, FiHome, FiMapPin, FiMoon, FiSun};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use dioxus_free_icons::icons::fi_icons::{FiMoon, FiSun};
use dioxus_free_icons::Icon;

#[component]
pub fn Navbar() -> Element {
    let mut theme = use_context::<Signal<Theme>>();

    // ── Layout Android / iOS ──────────────────────────────────────────────
    // Flex-column que ocupa exactamente la pantalla. El `main` tiene
    // `overflow-auto flex-1` y es el ÚNICO elemento que hace scroll.
    // La bottom nav es un hijo normal al final del flex → siempre visible.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return rsx! {
        div {
            style: "display:flex; flex-direction:column; height:100vh; overflow:hidden;",
            class: "bg-white dark:bg-zinc-950 transition-colors duration-200",

            // Top bar mínima: solo logo + toggle tema
            div {
                style: "flex-shrink:0;",
                class: "w-full border-b border-zinc-200 dark:border-zinc-800 \
                        bg-white dark:bg-zinc-950 transition-colors duration-200",
                div { class: "flex h-14 items-center justify-between px-4",
                    Link {
                        to: Route::Home {},
                        class: "text-xl font-bold text-zinc-950 dark:text-zinc-50",
                        "TF Sports"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        shape: ButtonShape::Default,
                        is_icon: Some(true),
                        class: "m-2",
                        onclick: move |_| {
                            let new_theme = theme().toggle();
                            new_theme.apply();
                            new_theme.save_to_storage();
                            theme.set(new_theme);
                        },
                        if matches!(theme(), Theme::Dark) {
                            Icon { class: "size-5", icon: FiSun }
                        } else {
                            Icon { class: "size-5", icon: FiMoon }
                        }
                    }
                }
            }

            // Contenido: flex-1 + overflow-auto → el único área scrollable
            main {
                id: "main-content",
                style: "flex:1 1 0%; overflow-y:auto; min-height:0;",
                Outlet::<Route> {}
            }

            // Bottom nav: hijo normal del flex, nunca se desplaza
            div {
                style: "flex-shrink:0;",
                class: "border-t border-zinc-200 dark:border-zinc-800 \
                        bg-white dark:bg-zinc-950 transition-colors duration-200",
                nav { class: "flex h-16 items-stretch justify-around px-2 my-2 border-t border-zinc-200 dark:border-zinc-800",
                    BottomNavItem { to: Route::Home {}, label: "Inicio",
                        Icon { class: "size-4", icon: FiHome }
                    }
                    BottomNavItem { to: Route::Events {}, label: "Eventos",
                        Icon { class: "size-4", icon: FiCalendar }
                    }
                    BottomNavItem { to: Route::Sports {}, label: "Deportes",
                        Icon { class: "size-4", icon: FiActivity }
                    }
                    BottomNavItem { to: Route::Places {}, label: "Lugares",
                        Icon { class: "size-4", icon: FiMapPin }
                    }
                }
            }
        }
    };

    // ── Layout web / desktop ──────────────────────────────────────────────
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    rsx! {
        div { class: "min-h-screen flex flex-col bg-white dark:bg-zinc-950",
            nav {
                class: "sticky top-0 z-50 w-full border-b border-zinc-200 dark:border-zinc-800 \
                        bg-white/80 dark:bg-zinc-950/80 backdrop-blur \
                        supports-[backdrop-filter]:bg-white/60 \
                        dark:supports-[backdrop-filter]:bg-zinc-950/60",
                div { class: "container mx-auto px-4 md:px-6 lg:px-8",
                    div { class: "flex h-14 items-center justify-between",
                        Link {
                            to: Route::Home {},
                            class: "text-base font-bold text-zinc-950 dark:text-zinc-50 \
                                    hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors",
                            "TF Sports"
                        }
                        div { class: "flex items-center gap-6",
                            NavLink { to: Route::Events {}, label: "Eventos" }
                            NavLink { to: Route::Sports {}, label: "Deportes" }
                            NavLink { to: Route::Places {}, label: "Instalaciones" }
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            shape: ButtonShape::Default,
                            is_icon: Some(true),
                            class: "m-2",
                            onclick: move |_| {
                                let new_theme = theme().toggle();
                                new_theme.apply();
                                new_theme.save_to_storage();
                                theme.set(new_theme);
                            },
                            if matches!(theme(), Theme::Dark) {
                                Icon { class: "size-4", icon: FiSun }
                            } else {
                                Icon { class: "size-4", icon: FiMoon }
                            }
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

// ── Link de escritorio ─────────────────────────────────────────────────────────
#[component]
fn NavLink(to: Route, label: String) -> Element {
    rsx! {
        Link {
            to,
            class: "text-sm font-medium text-zinc-600 dark:text-zinc-400 \
                    transition-colors hover:text-zinc-900 dark:hover:text-zinc-100",
            "{label}"
        }
    }
}

// ── Ítem de la bottom nav ─────────────────────────────────────────────────────
#[component]
fn BottomNavItem(to: Route, label: String, children: Element) -> Element {
    let current = use_route::<Route>();
    let is_active = current == to;
    let variant = if is_active {
        ButtonVariant::Default
    } else {
        ButtonVariant::Outline
    };
    rsx! {
            Link {
                to: to.clone(),
                class: "flex-1 flex flex-col justify-center items-center",
                Button {
                    variant,
                    shape: ButtonShape::Default,
                    is_icon: Some(true),
                    class: "m-2",
                    {children}
                }
                span { class: "text-[10px] font-medium leading-none", "{label}" }
            }
    }
}
