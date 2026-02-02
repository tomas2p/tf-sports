use crate::Route;
use crate::components::ui::*;
use crate::components::{EventCard, PageHeader, EmptyState, Breadcrumb, BreadcrumbItem, Pagination, FilterSection, FilterConfig};
use crate::models::{EventoData, DEPORTES};
use dioxus::prelude::*;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

#[component]
pub fn Sport(category: String) -> Element {
    // Cargar eventos desde JSON
    let eventos_data = use_memo(move || {
        match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
            Ok(data) => data,
            Err(_) => EventoData { eventos: vec![] }
        }
    });
    
    // Clonar category para usarlo en múltiples closures
    let category_clone1 = category.clone();
    let category_clone2 = category.clone();
    
    // Buscar información del deporte
    let deporte_info = use_memo(move || {
        DEPORTES.iter()
            .find(|d| d.nombre == category_clone1)
            .cloned()
    });
    
    // Signals para filtros y paginación
    let mut filter_organizador = use_signal(|| "Todos".to_string());
    let mut filter_municipio = use_signal(|| "Todos".to_string());
    let mut orden = use_signal(|| "fecha_asc".to_string());
    let mut page = use_signal(|| 1);
    let items_per_page = 6; // Una fila de 6 eventos
    
    // Clonar category para filtros
    let category_clone3 = category.clone();
    let category_clone4 = category.clone();
    
    // Obtener listas únicas para filtros (solo de eventos de este deporte)
    let organizadores_disponibles = use_memo(move || {
        let data = eventos_data();
        let info = deporte_info();
        let deporte_actual = category_clone3.clone();
        
        let mut orgs: Vec<String> = data.eventos.iter()
            .filter(|e| {
                // Filtrar solo eventos de este deporte
                if let Some(ref info) = info {
                    let nombre_lower = e.evento_nombre.to_lowercase();
                    let desc_lower = e.evento_descripcion.to_lowercase();
                    nombre_lower.contains(info.keyword) || desc_lower.contains(info.keyword)
                } else {
                    e.get_deporte() == deporte_actual
                }
            })
            .map(|e| e.evento_organizador.clone())
            .collect();
        orgs.sort();
        orgs.dedup();
        orgs
    });
    
    let municipios_disponibles = use_memo(move || {
        let data = eventos_data();
        let info = deporte_info();
        let deporte_actual = category_clone4.clone();
        
        let mut munis: Vec<String> = data.eventos.iter()
            .filter(|e| {
                // Filtrar solo eventos de este deporte
                if let Some(ref info) = info {
                    let nombre_lower = e.evento_nombre.to_lowercase();
                    let desc_lower = e.evento_descripcion.to_lowercase();
                    nombre_lower.contains(info.keyword) || desc_lower.contains(info.keyword)
                } else {
                    e.get_deporte() == deporte_actual
                }
            })
            .filter_map(|e| e.municipio_nombre.clone())
            .collect();
        munis.sort();
        munis.dedup();
        munis
    });
    
    // Filtrar y ordenar eventos
    let eventos_filtrados = use_memo(move || {
        let data = eventos_data();
        let deporte_actual = category_clone2.clone();
        let info = deporte_info();
        let org_val = filter_organizador();
        let muni_val = filter_municipio();
        let orden_val = orden();
        
        let mut filtered: Vec<(usize, crate::models::Evento)> = data.eventos.into_iter()
            .enumerate()
            .filter(|(_, e)| {
                // Filtro por deporte (usando el keyword del deporte)
                let deporte_match = if let Some(ref info) = info {
                    let nombre_lower = e.evento_nombre.to_lowercase();
                    let desc_lower = e.evento_descripcion.to_lowercase();
                    nombre_lower.contains(info.keyword) || desc_lower.contains(info.keyword)
                } else {
                    e.get_deporte() == deporte_actual
                };
                
                // Filtro por organizador
                let org_match = if org_val == "Todos" {
                    true
                } else {
                    e.evento_organizador == org_val
                };
                
                // Filtro por municipio
                let muni_match = if muni_val == "Todos" {
                    true
                } else {
                    e.municipio_nombre.as_ref() == Some(&muni_val)
                };
                
                deporte_match && org_match && muni_match
            })
            .collect();
        
        // Ordenar
        match orden_val.as_str() {
            "fecha_asc" => filtered.sort_by(|a, b| 
                a.1.evento_fecha_inicio.cmp(&b.1.evento_fecha_inicio)),
            "fecha_desc" => filtered.sort_by(|a, b| 
                b.1.evento_fecha_inicio.cmp(&a.1.evento_fecha_inicio)),
            "nombre_az" => filtered.sort_by(|a, b| 
                a.1.evento_nombre.cmp(&b.1.evento_nombre)),
            "nombre_za" => filtered.sort_by(|a, b| 
                b.1.evento_nombre.cmp(&a.1.evento_nombre)),
            "organizador" => filtered.sort_by(|a, b| 
                a.1.evento_organizador.cmp(&b.1.evento_organizador)),
            "municipio" => filtered.sort_by(|a, b| 
                a.1.municipio_nombre.cmp(&b.1.municipio_nombre)),
            _ => {}
        }
        
        filtered
    });
    
    // Paginación
    let eventos_paginados = use_memo(move || {
        let eventos = eventos_filtrados();
        let current_page = page();
        let start = ((current_page - 1) * items_per_page) as usize;
        let end = (start + items_per_page as usize).min(eventos.len());
        eventos[start..end].to_vec()
    });
    
    let total_pages = use_memo(move || {
        let total = eventos_filtrados().len();
        ((total as f64) / (items_per_page as f64)).ceil() as usize
    });
    
    // Si no hay eventos, mostrar estado vacío
    if eventos_filtrados().is_empty() {
        return rsx! {
            Container {
                Section {
                    EmptyState {
                        emoji: deporte_info().map(|d| d.emoji.to_string()).unwrap_or("🏆".to_string()),
                        title: "No hay eventos disponibles".to_string(),
                        message: format!("No se encontraron eventos de {} con los filtros seleccionados.", category),
                        action: rsx! {
                            Link { to: Route::Sports {},
                                Button { variant: ButtonVariant::Default, "Ver todos los deportes" }
                            }
                        },
                    }
                }
            }
        };
    }
    
    // Buscar el evento más próximo (que no haya finalizado)
    let evento_destacado = use_memo(move || {
        let eventos = eventos_filtrados();
        let now = chrono::Local::now().naive_local();
        
        eventos.into_iter()
            .filter(|(_, e)| {
                // Filtrar eventos que no han finalizado
                if let Ok(fecha_fin) = chrono::NaiveDateTime::parse_from_str(&e.evento_fecha_fin, "%Y-%m-%d %H:%M:%S") {
                    fecha_fin >= now
                } else {
                    true
                }
            })
            .min_by_key(|(_, e)| {
                // Ordenar por fecha de inicio más cercana
                e.evento_fecha_inicio.clone()
            })
    });
    
    rsx! {
        Container {
            Section {
                // Breadcrumb
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            label: "Inicio".to_string(),
                            route: Some(Route::Home {}),
                        },
                        BreadcrumbItem {
                            label: "Deportes".to_string(),
                            route: Some(Route::Sports {}),
                        },
                        BreadcrumbItem {
                            label: category.clone(),
                            route: None,
                        },
                    ],
                }

                // Header
                PageHeader {
                    title: format!("{} en Tenerife", category),
                    description: Some(format!("Mostrando {} eventos", eventos_filtrados().len())),
                }

                // Featured Card con evento destacado
                if let Some((idx, evento)) = evento_destacado() {
                    Card { class: "mb-8 overflow-hidden",
                        div { class: "grid md:grid-cols-2",
                            div { class: "p-8 flex flex-col justify-center",
                                Badge {
                                    variant: match evento.get_badge_variant() {
                                        "EN VIVO" => BadgeVariant::Default,
                                        "FINALIZADO" => BadgeVariant::Outline,
                                        _ => BadgeVariant::Secondary,
                                    },
                                    class: "mb-4",
                                    "{evento.get_badge_variant()}"
                                }
                                h2 { class: "text-3xl font-bold text-zinc-950 dark:text-zinc-50 mb-4",
                                    "{evento.evento_nombre}"
                                }
                                p { class: "text-zinc-600 dark:text-zinc-400 mb-6",
                                    "{evento.evento_descripcion}"
                                }
                                Link { to: Route::Details { id: idx as i32 },
                                    Button { variant: ButtonVariant::Default, "Ver Detalles" }
                                }
                            }
                            div { class: "bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 min-h-[300px] flex items-center justify-center",
                                span { class: "text-9xl", "{evento.get_deporte_emoji()}" }
                            }
                        }
                    }
                }

                Separator { class: "my-8" }

                // Filtros y Ordenación
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
                                ("organizador".to_string(), "Organizador".to_string()),
                                ("municipio".to_string(), "Municipio".to_string()),
                            ],
                            on_change: EventHandler::new(move |val: String| orden.set(val)),
                        },
                        FilterConfig {
                            label: "Organizador:".to_string(),
                            value: filter_organizador,
                            options: {
                                let mut opts = vec![
                                    ("Todos".to_string(), "Todos los organizadores".to_string()),
                                ];
                                for org in organizadores_disponibles() {
                                    opts.push((org.clone(), org));
                                }
                                opts
                            },
                            on_change: EventHandler::new(move |val: String| {
                                filter_organizador.set(val);
                                page.set(1);
                            }),
                        },
                        FilterConfig {
                            label: "Municipio:".to_string(),
                            value: filter_municipio,
                            options: {
                                let mut opts = vec![
                                    ("Todos".to_string(), "Todos los municipios".to_string()),
                                ];
                                for muni in municipios_disponibles() {
                                    opts.push((muni.clone(), muni));
                                }
                                opts
                            },
                            on_change: EventHandler::new(move |val: String| {
                                filter_municipio.set(val);
                                page.set(1);
                            }),
                        },
                    ],
                }

                // Grid de eventos
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for (idx , evento) in eventos_paginados() {
                        EventCard { evento, index: idx as i32 }
                    }
                }

                // Paginación
                Pagination { current_page: page, total_pages: total_pages() }
            }
        }
    }
}
