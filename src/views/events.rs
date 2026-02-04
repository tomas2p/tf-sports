use crate::components::event_card::LayoutVariant;
use crate::components::ui::*;
use crate::components::{
    EmptyState, EventCard, FilterConfig, FilterSection, PageHeader, Pagination,
};
use crate::models::{Evento, EventoData};
use crate::utils::pagination_filters::{paginate, total_pages};
use dioxus::prelude::*;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

#[component]
pub fn Events() -> Element {
    let eventos_data = use_memo(
        move || match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
            Ok(data) => {
                web_sys::console::log_1(
                    &format!("Eventos cargados: {}", data.eventos.len()).into(),
                );
                data
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Error parsing JSON: {:?}", e).into());
                EventoData { eventos: vec![] }
            }
        },
    );

    let mut filter_estado = use_signal(|| "PRÓXIMO".to_string());
    let mut filter_deporte = use_signal(|| "Todos".to_string());
    let mut orden = use_signal(|| "fecha_asc".to_string());
    let mut page = use_signal(|| 1);
    let items_per_page = 12;

    // Obtener lista de deportes únicos
    let deportes_disponibles = use_memo(move || {
        let data = eventos_data();
        let mut deportes: Vec<String> = data.eventos.iter().map(|e| e.get_deporte()).collect();
        deportes.sort();
        deportes.dedup();
        deportes
    });

    let eventos_filtrados = use_memo(move || {
        let data = eventos_data();
        let estado_val = filter_estado();
        let deporte_val = filter_deporte();
        let orden_val = orden();

        let mut filtered: Vec<(usize, Evento)> = data
            .eventos
            .into_iter()
            .enumerate()
            .filter(|(_, e)| {
                let estado_match = if estado_val == "Todos" {
                    true
                } else {
                    e.get_badge_variant() == estado_val
                };

                let deporte_match = if deporte_val == "Todos" {
                    true
                } else {
                    e.get_deporte() == deporte_val
                };

                estado_match && deporte_match
            })
            .collect();

        // Ordenar
        match orden_val.as_str() {
            "fecha_asc" => {
                filtered.sort_by(|a, b| a.1.evento_fecha_inicio.cmp(&b.1.evento_fecha_inicio))
            }
            "fecha_desc" => {
                filtered.sort_by(|a, b| b.1.evento_fecha_inicio.cmp(&a.1.evento_fecha_inicio))
            }
            "nombre_az" => filtered.sort_by(|a, b| a.1.evento_nombre.cmp(&b.1.evento_nombre)),
            "nombre_za" => filtered.sort_by(|a, b| b.1.evento_nombre.cmp(&a.1.evento_nombre)),
            "deporte" => filtered.sort_by(|a, b| a.1.get_deporte().cmp(&b.1.get_deporte())),
            _ => {}
        }

        web_sys::console::log_1(&format!("Eventos filtrados: {}", filtered.len()).into());
        filtered
    });

    let eventos_paginados = use_memo(move || {
        let eventos = eventos_filtrados();
        paginate(&eventos, page() as usize, items_per_page)
    });

    let total_pages = use_memo(move || {
        let total = eventos_filtrados().len();
        total_pages(total, items_per_page)
    });

    rsx! {
        Container {
            Section {
                // Header
                PageHeader {
                    title: "Eventos Deportivos en Tenerife".to_string(),
                    description: Some(
                        format!(
                            "Mostrando {} de {} eventos",
                            eventos_paginados().len(),
                            eventos_filtrados().len(),
                        ),
                    ),
                }

                // Filtros
                FilterSection {
                    filters: vec![
                        FilterConfig {
                            label: "Ordenar por:".to_string(),
                            value: orden,
                            options: vec![
                                ("fecha_asc".to_string(), "Fecha (próximos primero)".to_string()),
                                ("fecha_desc".to_string(), "Fecha (lejanos primero)".to_string()),
                                ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                                ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
                                ("deporte".to_string(), "Deporte".to_string()),
                            ],
                            on_change: EventHandler::new(move |val: String| orden.set(val)),
                        },
                        FilterConfig {
                            label: "Estado:".to_string(),
                            value: filter_estado,
                            options: vec![
                                ("PRÓXIMO".to_string(), "Próximos".to_string()),
                                ("EN VIVO".to_string(), "En Vivo".to_string()),
                                ("FINALIZADO".to_string(), "Finalizados".to_string()),
                                ("Todos".to_string(), "Todos los estados".to_string()),
                            ],
                            on_change: EventHandler::new(move |val: String| {
                                filter_estado.set(val);
                                page.set(1);
                            }),
                        },
                        FilterConfig {
                            label: "Deporte:".to_string(),
                            value: filter_deporte,
                            options: {
                                let mut opts = vec![
                                    ("Todos".to_string(), "Todos los deportes".to_string()),
                                ];
                                for deporte in deportes_disponibles() {
                                    opts.push((deporte.clone(), deporte));
                                }
                                opts
                            },
                            on_change: EventHandler::new(move |val: String| {
                                filter_deporte.set(val);
                                page.set(1);
                            }),
                        },
                    ],
                }

                // Grid de eventos
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
                    if eventos_paginados().is_empty() {
                        div { class: "col-span-full",
                            EmptyState {
                                emoji: "🏆".to_string(),
                                title: "No hay eventos disponibles".to_string(),
                                message: "No se encontraron eventos con los filtros seleccionados.".to_string(),
                            }
                        }
                    }
                    for (original_idx , evento) in eventos_paginados().iter() {
                        EventCard {
                            key: "{original_idx}",
                            evento: evento.clone(),
                            index: *original_idx as i32,
                            layout: LayoutVariant::Detailed,
                        }
                    }
                }

                // Paginación
                Pagination { current_page: page, total_pages: total_pages() }
            }
        }
    }
}
