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

    // ── Android/iOS back-button: delegate to Dioxus Router via window.onAndroidBack ──
    // Dioxus Router uses pushState internally, so window.location.pathname always returns "/".
    // → JS cannot detect root via pathname; we must let Rust decide based on current Route.
    // When at Home: exit the app. Otherwise: navigator.go_back().
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        use dioxus::prelude::document;
        let navigator = use_navigator();
        let current_route = use_route::<Route>();

        // Signal reactivo: true cuando estamos en Home.
        let mut is_home_signal = use_signal(|| matches!(current_route, Route::Home {}));
        use_effect(move || {
            is_home_signal.set(matches!(current_route, Route::Home {}));
        });

        // ── Arquitectura de canal bidireccional (un solo eval) ──────────────────────────
        //
        // Canal Rust→JS: eval.send("home"/"away") → JS `await dioxus.recv()`
        //   Rust actualiza `atHome` dentro del closure del eval cuando cambia la ruta.
        //
        // Canal JS→Rust: JS `dioxus.send('back')` → Rust `eval.recv().await`
        //   El setInterval notifica a Rust que el usuario pulsó back en una sub-ruta.
        //
        // Toda la lógica `atHome` vive en la variable local del closure → nunca hay
        // problemas de contextos JS aislados ni de timing entre evals separados.
        //
        // - En Home: onAndroidBack pone `_android_back_handled = false`
        //   → Kotlin lee "1|root..." → Activity.finish() limpio (sin crash).
        // - En sub-ruta: onAndroidBack pone `_android_back_handled = true`
        //   → Kotlin lee "0|handler..." → swallow; Rust recibe 'back' → navigator.go_back().
        let mut eval = use_hook(|| {
            document::eval(
                r#"(function() {
                    var d = dioxus;
                    var pending = false;
                    var atHome = true; // default conservador: Home es seguro
                    
                    // Recv loop Canal 1 (Rust→JS): actualiza atHome con el estado real de la ruta.
                    (async function() {
                        while (true) {
                            try {
                                var msg = await d.recv();
                                atHome = (msg === 'home');
                            } catch(e) { break; }
                        }
                    })();
                    
                    window.onAndroidBack = function() {
                        if (atHome) {
                            // En Home: Kotlin cierra la Activity limpiamente.
                            window._android_back_handled = false;
                        } else {
                            // En sub-ruta: Rust navega atrás vía navigator.go_back().
                            window._android_back_handled = true;
                            pending = true;
                        }
                    };
                    
                    // Canal 2 (JS→Rust): notifica a Rust cuando hay un back en sub-ruta.
                    setInterval(function() {
                        if (pending) { pending = false; d.send('back'); }
                    }, 50);
                })()"#,
            )
        });

        // Envía el estado de ruta al eval cada vez que cambia (Canal 1: Rust→JS).
        use_effect(move || {
            let msg = if is_home_signal() { "home" } else { "away" };
            eval.send(msg).ok();
        });

        // Recibe eventos 'back' del eval (Canal 2: JS→Rust) y navega atrás.
        use_effect(move || {
            spawn(async move {
                loop {
                    match eval.recv::<String>().await {
                        Ok(_) => {
                            eprintln!("[BackNav] go_back (not at home)");
                            navigator.go_back();
                        }
                        Err(_) => break,
                    }
                }
            });
        });
    }

    // ── Layout Android / iOS ──────────────────────────────────────────────
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
                        div { class: "flex flex-col md:flex-row justify-between gap-x-6",
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
