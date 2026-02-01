use crate::Route;
use crate::components::ui::*;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Container {
            Section {
                // Hero Section
                div { class: "text-center space-y-6 mb-16",
                    h1 { class: "text-5xl md:text-6xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
                        "Eventos Deportivos en Vivo"
                    }
                    p { class: "text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto",
                        "Sigue todos tus deportes favoritos en tiempo real. Resultados, estadísticas y calendario completo."
                    }
                    div { class: "flex gap-4 justify-center mt-8",
                        Link { to: Route::Events {},
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Lg,
                                "Ver Eventos"
                            }
                        }
                        Link {
                            to: Route::Sport {
                                category: "football".to_string(),
                            },
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Lg,
                                "Explorar Deportes"
                            }
                        }
                    }
                }

                // Secciones destacadas
                div { class: "grid gap-6 md:grid-cols-3",
                    Link { to: Route::Events {}, class: "no-underline",
                        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                            CardHeader {
                                Badge { variant: BadgeVariant::Default, "EN VIVO" }
                                CardTitle { class: "mt-4", "Eventos Actuales" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Sigue los partidos y competiciones en vivo. Resultados en tiempo real y estadísticas actualizadas."
                                }
                            }
                        }
                    }

                    Link {
                        to: Route::Sport {
                            category: "football".to_string(),
                        },
                        class: "no-underline",
                        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                            CardHeader {
                                Badge { variant: BadgeVariant::Secondary, "POPULAR" }
                                CardTitle { class: "mt-4", "Ligas y Competiciones" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Explora las mejores ligas del mundo. Premier League, La Liga, Champions League y más."
                                }
                            }
                        }
                    }

                    Link { to: Route::Events {}, class: "no-underline",
                        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                            CardHeader {
                                Badge { variant: BadgeVariant::Outline, "PRÓXIMO" }
                                CardTitle { class: "mt-4", "Calendario" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Revisa los próximos eventos y planifica. No te pierdas ningún partido importante."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
