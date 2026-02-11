use crate::Route;
use crate::components::ui::*;
// EventoData import removed; use `get_eventos()` helper instead when needed
use crate::data::get_eventos;
use crate::models::strip_html;
use dioxus::prelude::*;


#[component]
pub fn Event(id: i32) -> Element {
    // Cargar datos (cacheados)
    let eventos = use_memo(move || get_eventos().clone());

    // Buscar evento por índice
    let evento = use_memo(move || {
        let idx = id as usize;
        eventos().eventos.get(idx).cloned()
    });

    match evento() {
        Some(evt) => {
            let badge_variant = match evt.get_badge_variant() {
                "EN VIVO" => BadgeVariant::Default,
                "PRÓXIMO" => BadgeVariant::Secondary,
                _ => BadgeVariant::Outline,
            };
            
            rsx! {
                Container {
                    Section {
                        // Imagen del deporte
                        div { class: "relative h-48 bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 rounded-lg mb-8 flex items-center justify-center overflow-hidden",
                            span { class: "text-9xl", "{evt.get_deporte_emoji()}" }
                        }

                        // Hero Section
                        div { class: "space-y-4",
                            Badge { variant: badge_variant, "{evt.get_badge_variant()}" }
                            h1 { class: "text-4xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
                                "{evt.evento_nombre}"
                            }
                            p { class: "text-lg text-zinc-600 dark:text-zinc-400",
                                "{evt.get_deporte()} • {evt.format_fecha()} • {evt.evento_lugar.as_ref().unwrap_or(&\"Lugar por determinar\".to_string())}"
                            }
                        }

                        // Información principal
                        div { class: "grid gap-6 md:grid-cols-2 mt-8",
                            Card {
                                CardHeader {
                                    CardTitle { "Información del Evento" }
                                }
                                CardContent {
                                    div { class: "space-y-3",
                                        div {
                                            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                "Organizador"
                                            }
                                            p { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                                "{evt.evento_organizador}"
                                            }
                                        }
                                        if let Some(municipio) = &evt.municipio_nombre {
                                            div {
                                                p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                    "Municipio"
                                                }
                                                p { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                                    "{municipio}"
                                                }
                                            }
                                        }
                                        div {
                                            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                "Deporte"
                                            }
                                            p { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                                "{evt.get_deporte()}"
                                            }
                                        }
                                    }
                                }
                            }

                            Card {
                                CardHeader {
                                    CardTitle { "Fechas" }
                                }
                                CardContent {
                                    div { class: "space-y-3",
                                        div {
                                            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                "Inicio"
                                            }
                                            p { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                                "{evt.format_fecha()}"
                                            }
                                        }
                                        div {
                                            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                                "Finalización"
                                            }
                                            p { class: "text-sm font-medium text-zinc-950 dark:text-zinc-50",
                                                {
                                                    use chrono::NaiveDateTime;
                                                    if let Ok(fecha) = NaiveDateTime::parse_from_str(
                                                        &evt.evento_fecha_fin,
                                                        "%Y-%m-%d %H:%M:%S",
                                                    ) {
                                                        fecha.format("%d/%m/%Y • %H:%M hrs").to_string()
                                                    } else {
                                                        evt.evento_fecha_fin.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        Separator { class: "my-8" }

                        // Descripción
                        div { class: "space-y-4",
                            h2 { class: "text-2xl font-semibold text-zinc-950 dark:text-zinc-50",
                                "Descripción del Evento"
                            }
                            p { class: "text-zinc-600 dark:text-zinc-400 leading-relaxed",
                                "{strip_html(&evt.evento_descripcion)}"
                            }

                            if !evt.evento_url.is_empty() {
                                div { class: "pt-4",
                                    a {
                                        href: "{evt.evento_url}",
                                        target: "_blank",
                                        class: "inline-flex items-center gap-2 text-sm font-medium text-zinc-950 dark:text-zinc-50 hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors",
                                        "Ver más información"
                                        span { "→" }
                                    }
                                }
                            }
                        }

                        // Botón para volver
                        div { class: "mt-8",
                            Link {
                                to: Route::Events {},
                                class: "inline-flex items-center gap-2 text-sm font-medium text-zinc-600 dark:text-zinc-400 hover:text-zinc-950 dark:hover:text-zinc-50 transition-colors",
                                span { "←" }
                                "Volver a eventos"
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                Container {
                    Section {
                        div { class: "text-center py-12",
                            h1 { class: "text-2xl font-bold text-zinc-950 dark:text-zinc-50",
                                "Evento no encontrado"
                            }
                            p { class: "text-zinc-600 dark:text-zinc-400 mt-2",
                                "El evento que buscas no existe."
                            }
                            div { class: "mt-6",
                                Link { to: Route::Events {},
                                    Button { variant: ButtonVariant::Default, "Volver a eventos" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
