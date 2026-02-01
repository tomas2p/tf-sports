use crate::Route;
use crate::components::ui::*;
use dioxus::prelude::*;

#[component]
pub fn Sport(category: String) -> Element {
    rsx! {
        Container {
            Section {
                // Header con breadcrumb
                div { class: "space-y-4 mb-8",
                    div { class: "flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400",
                        Link {
                            to: Route::Home {},
                            class: "hover:text-zinc-900 dark:hover:text-zinc-100",
                            "Inicio"
                        }
                        span { "/" }
                        span { class: "text-zinc-900 dark:text-zinc-50 font-medium",
                            "{category}"
                        }
                    }
                    h1 { class: "text-4xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
                        "Fútbol Internacional"
                    }
                    p { class: "text-lg text-zinc-600 dark:text-zinc-400",
                        "Las mejores ligas y competiciones del mundo"
                    }
                }

                // Featured Card grande
                Card { class: "mb-8 overflow-hidden",
                    div { class: "grid md:grid-cols-2",
                        div { class: "p-8 flex flex-col justify-center",
                            Badge { variant: BadgeVariant::Default, class: "mb-4", "DESTACADO" }
                            h2 { class: "text-3xl font-bold text-zinc-950 dark:text-zinc-50 mb-4",
                                "Copa Mundial 2026"
                            }
                            p { class: "text-zinc-600 dark:text-zinc-400 mb-6",
                                "Sigue todos los partidos de la competición más importante del mundo. Desde la fase de grupos hasta la gran final."
                            }
                            Link { to: Route::Events {},
                                Button { variant: ButtonVariant::Default, "Ver Calendario Completo" }
                            }
                        }
                        div { class: "bg-zinc-100 dark:bg-zinc-800 min-h-[300px] flex items-center justify-center",
                            span { class: "text-zinc-400 dark:text-zinc-600", "[Imagen placeholder]" }
                        }
                    }
                }

                // Secciones por competición
                div { class: "space-y-8",
                    // Ligas Europeas
                    div {
                        h2 { class: "text-2xl font-semibold text-zinc-950 dark:text-zinc-50 mb-4",
                            "Ligas Europeas"
                        }
                        div { class: "grid gap-4 md:grid-cols-2 lg:grid-cols-4",
                            Link {
                                to: Route::Details { id: 10 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        CardTitle { "Premier League" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
                                            "Inglaterra"
                                        }
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                "20 equipos"
                                            }
                                            Badge { variant: BadgeVariant::Secondary,
                                                "En curso"
                                            }
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 11 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        CardTitle { "La Liga" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
                                            "España"
                                        }
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                "20 equipos"
                                            }
                                            Badge { variant: BadgeVariant::Secondary,
                                                "En curso"
                                            }
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 12 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        CardTitle { "Serie A" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
                                            "Italia"
                                        }
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                "20 equipos"
                                            }
                                            Badge { variant: BadgeVariant::Secondary,
                                                "En curso"
                                            }
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 13 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        CardTitle { "Bundesliga" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
                                            "Alemania"
                                        }
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                "18 equipos"
                                            }
                                            Badge { variant: BadgeVariant::Secondary,
                                                "En curso"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Separator {}

                    // Competiciones Internacionales
                    div {
                        h2 { class: "text-2xl font-semibold text-zinc-950 dark:text-zinc-50 mb-4",
                            "Competiciones Internacionales"
                        }
                        div { class: "grid gap-4 md:grid-cols-3",
                            Link {
                                to: Route::Details { id: 20 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        Badge { variant: BadgeVariant::Default, "EN VIVO" }
                                        CardTitle { class: "mt-2", "Champions League" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-4",
                                            "UEFA"
                                        }
                                        div { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                            "Semifinales en curso"
                                        }
                                        p { class: "text-xs text-zinc-500 dark:text-zinc-500 mt-2",
                                            "4 equipos restantes"
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 21 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        Badge { variant: BadgeVariant::Secondary, "PRÓXIMO" }
                                        CardTitle { class: "mt-2", "Copa América" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-4",
                                            "CONMEBOL"
                                        }
                                        div { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                            "Inicia 20 de Junio"
                                        }
                                        p { class: "text-xs text-zinc-500 dark:text-zinc-500 mt-2",
                                            "16 selecciones"
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 22 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                                    CardHeader {
                                        Badge { variant: BadgeVariant::Outline, "PRÓXIMO" }
                                        CardTitle { class: "mt-2", "Eurocopa" }
                                    }
                                    CardContent {
                                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-4",
                                            "UEFA"
                                        }
                                        div { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                            "Inicia 14 de Junio"
                                        }
                                        p { class: "text-xs text-zinc-500 dark:text-zinc-500 mt-2",
                                            "24 selecciones"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Separator {}

                    // Partidos destacados
                    div {
                        h2 { class: "text-2xl font-semibold text-zinc-950 dark:text-zinc-50 mb-4",
                            "Partidos Destacados de Hoy"
                        }
                        div { class: "space-y-3",
                            Link {
                                to: Route::Details { id: 30 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer",
                                    CardContent { class: "py-4",
                                        div { class: "flex items-center justify-between",
                                            div { class: "flex items-center gap-4 flex-1",
                                                Badge { variant: BadgeVariant::Default,
                                                    "EN VIVO"
                                                }
                                                div { class: "flex-1",
                                                    div { class: "flex items-center justify-between mb-1",
                                                        span { class: "font-medium text-zinc-950 dark:text-zinc-50",
                                                            "Real Madrid"
                                                        }
                                                        span { class: "text-2xl font-bold text-zinc-950 dark:text-zinc-50",
                                                            "2"
                                                        }
                                                    }
                                                    div { class: "flex items-center justify-between",
                                                        span { class: "font-medium text-zinc-950 dark:text-zinc-50",
                                                            "Barcelona"
                                                        }
                                                        span { class: "text-2xl font-bold text-zinc-950 dark:text-zinc-50",
                                                            "1"
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "text-right",
                                                p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                    "La Liga"
                                                }
                                                p { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                    "Minuto 67'"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 31 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer",
                                    CardContent { class: "py-4",
                                        div { class: "flex items-center justify-between",
                                            div { class: "flex items-center gap-4 flex-1",
                                                Badge { variant: BadgeVariant::Secondary,
                                                    "20:00"
                                                }
                                                div { class: "flex-1",
                                                    div { class: "flex items-center justify-between mb-1",
                                                        span { class: "font-medium text-zinc-950 dark:text-zinc-50",
                                                            "Manchester City"
                                                        }
                                                        span { class: "text-sm text-zinc-500 dark:text-zinc-500",
                                                            "vs"
                                                        }
                                                    }
                                                    div { class: "flex items-center justify-between",
                                                        span { class: "font-medium text-zinc-950 dark:text-zinc-50",
                                                            "Liverpool"
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "text-right",
                                                p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                    "Premier League"
                                                }
                                                p { class: "text-xs text-zinc-500 dark:text-zinc-500",
                                                    "Hoy 20:00 hrs"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            Link {
                                to: Route::Details { id: 32 },
                                class: "no-underline",
                                Card { class: "hover:shadow-md transition-shadow cursor-pointer",
                                    CardContent { class: "py-4",
                                        div { class: "flex items-center justify-between",
                                            div { class: "flex items-center gap-4 flex-1",
                                                Badge { variant: BadgeVariant::Secondary,
                                                    "22:00"
                                                }
                                                div { class: "flex-1",
                                                    div { class: "flex items-center justify-between mb-1",
                                                        span { class: "font-medium text-zinc-950",
                                                            "Bayern München"
                                                        }
                                                        span { class: "text-sm text-zinc-500",
                                                            "vs"
                                                        }
                                                    }
                                                    div { class: "flex items-center justify-between",
                                                        span { class: "font-medium text-zinc-950",
                                                            "Borussia Dortmund"
                                                        }
                                                    }
                                                }
                                            }
                                            div { class: "text-right",
                                                p { class: "text-sm text-zinc-600",
                                                    "Bundesliga"
                                                }
                                                p { class: "text-xs text-zinc-500",
                                                    "Hoy 22:00 hrs"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
