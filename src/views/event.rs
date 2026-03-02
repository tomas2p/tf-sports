use crate::components::ui::*;
use crate::components::{breadcrumb_items, Breadcrumb};
use crate::Route;
// EventoData import removed; use `get_eventos()` helper instead when needed
use crate::data::get_eventos;
use crate::models::strip_html;
use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn Event(id: i32) -> Element {
    // Cargar datos (cacheados)
    let eventos = use_memo(move || get_eventos().clone());

    // Buscar evento por índice
    let evento = use_memo(move || {
        let idx = id as usize;
        eventos().eventos.get(idx).cloned()
    });

    // Obtener breadcrumb del contexto de forma no condicional (hooks siempre en nivel superior)
    let breadcrumb_ctx =
        use_context::<Signal<Option<Vec<crate::components::breadcrumb::BreadcrumbItem>>>>();

    match evento() {
        Some(evt) => {
            let badge_variant = match evt.get_badge_variant() {
                "EN VIVO" => BadgeVariant::Default,
                "PRÓXIMO" => BadgeVariant::Secondary,
                _ => BadgeVariant::Outline,
            };

            // Si hay un breadcrumb parcial publicado (por ejemplo desde `Sport`), usarlo y
            // añadir el nombre del evento; si no, usar el breadcrumb por defecto (Eventos).
            // `breadcrumb_ctx` se obtuvo arriba
            let breadcrumb_items_vec = match breadcrumb_ctx.read().clone() {
                Some(mut v) => {
                    v.push(crate::components::breadcrumb::BreadcrumbItem {
                        label: evt.evento_nombre.clone(),
                        route: None::<Route>,
                    });
                    v
                }
                None => breadcrumb_items!(
                    ("Inicio", Route::Home {}),
                    ("Eventos", Route::Events {}),
                    (evt.evento_nombre.clone())
                ),
            };

            // Build JSON-LD for this event
            let ld = json!({
                "@context": "https://schema.org",
                "@type": "Event",
                "name": evt.evento_nombre.clone(),
                "startDate": evt.evento_fecha_inicio.replace(" ", "T"),
                "endDate": evt.evento_fecha_fin.replace(" ", "T"),
                "description": strip_html(&evt.evento_descripcion),
                "location": {
                    "@type": "Place",
                    "name": evt.evento_lugar.clone().unwrap_or_else(|| "Lugar por determinar".to_string()),
                    "address": { "addressLocality": evt.municipio_nombre.clone().unwrap_or_default() }
                },
                "organizer": { "@type": "Organization", "name": evt.evento_organizador.clone() },
                "url": evt.evento_url.clone()
            });

            let ld_json = serde_json::to_string(&ld).unwrap_or_default();

            rsx! {
                BaseLayout {
                    title: evt.evento_nombre.clone(),
                    breadcrumb: rsx! { Breadcrumb { items: breadcrumb_items_vec } },
                    subtitle: Some(format!("{} • {} • {}",
                        evt.get_deporte(),
                        evt.format_fecha(),
                        evt.evento_lugar.as_ref().unwrap_or(&"Lugar por determinar".to_string())
                    )),
                    badge: rsx! { Badge { variant: badge_variant, "{evt.get_badge_variant()}" } },
                    hero: Some(evt.get_deporte_emoji().to_string()),
                    meta: rsx! {
                        div { class: "space-y-6",
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
                                // JSON-LD for the event (in head)
                                document::Script { r#type: "application/ld+json", "{ld_json}" }

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
                    },

                    // Main content (children)
                    div { class: "space-y-4",
                        // Separator { class: "my-8" }

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
