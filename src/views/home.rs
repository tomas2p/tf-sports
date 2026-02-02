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
                        "Agenda Deportiva de Tenerife"
                    }
                    p { class: "text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto",
                        "Descubre todos los eventos deportivos en Tenerife. Competiciones, entrenamientos y actividades para toda la isla."
                    }
                    div { class: "flex gap-4 justify-center mt-8",
                        Link { to: Route::Events {},
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Lg,
                                "Ver Eventos"
                            }
                        }
                        Link { to: Route::Sports {},
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
                                Badge { variant: BadgeVariant::Default, "ACTIVOS" }
                                CardTitle { class: "mt-4", "Eventos Próximos" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Consulta los eventos deportivos programados en toda la isla. Filtra por deporte, estado y municipio."
                                }
                            }
                        }
                    }

                    Link { to: Route::Sports {}, class: "no-underline",
                        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                            CardHeader {
                                Badge { variant: BadgeVariant::Secondary, "14 DEPORTES" }
                                CardTitle { class: "mt-4", "Categorías Deportivas" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Explora todos los deportes disponibles: ciclismo, natación, voleibol, ajedrez, taekwondo y más."
                                }
                            }
                        }
                    }

                    Link { to: Route::Events {}, class: "no-underline",
                        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
                            CardHeader {
                                Badge { variant: BadgeVariant::Outline, "INFORMACIÓN" }
                                CardTitle { class: "mt-4", "Detalles Completos" }
                            }
                            CardContent {
                                p { class: "text-zinc-600 dark:text-zinc-400",
                                    "Encuentra organizadores, ubicaciones, fechas y toda la información necesaria para participar."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
