use crate::Route;
use crate::components::ui::*;
use crate::components::{Calendar, EventCardWithImage, EmptyState, calendar::fecha_en_espanol};
use crate::models::{EventoData, Evento};
use dioxus::prelude::*;
use chrono::Datelike;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let eventos_data = use_memo(move || {
        match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
            Ok(data) => data,
            Err(e) => {
                web_sys::console::log_1(&format!("Error parsing JSON: {:?}", e).into());
                EventoData { eventos: vec![] }
            }
        }
    });
    
    let mut selected_date = use_signal(|| None::<chrono::NaiveDate>);
    let today = chrono::Local::now().date_naive();
    let days_from_monday = today.weekday().num_days_from_monday();
    let monday = today - chrono::Duration::days(days_from_monday as i64);
    let mut current_week_start = use_signal(move || monday);
    
    let eventos_filtrados = use_memo(move || {
        let data = eventos_data();
        data.eventos.into_iter().enumerate().collect::<Vec<_>>()
    });
    
    let eventos_del_dia_info = use_memo(move || {
        if let Some(date) = selected_date() {
            let todos_eventos = eventos_filtrados();
            let eventos_del_dia: Vec<(usize, Evento)> = todos_eventos
                .iter()
                .filter(|(_, e)| {
                    if let Ok(fecha) = chrono::NaiveDateTime::parse_from_str(
                        &e.evento_fecha_inicio,
                        "%Y-%m-%d %H:%M:%S",
                    ) {
                        fecha.date() == date
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();
            let eventos_count = eventos_del_dia.len();
            let cols = if eventos_count == 0 {
                1
            } else if eventos_count < 2 {
                1
            } else if eventos_count < 3 {
                2
            } else if eventos_count < 4 {
                3
            } else if eventos_count < 5 {
                4
            } else if eventos_count < 6 {
                5
            } else {
                6
            };
            Some((eventos_del_dia, cols, date))
        } else {
            None
        }
    });
    
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

                // Calendario semanal compacto
                Calendar {
                    eventos: eventos_filtrados().iter().map(|(_, e)| e.clone()).collect(),
                    selected_date,
                    current_week_start,
                }

                // Mostrar eventos de la fecha seleccionada
                if let Some((eventos_del_dia, cols, date)) = eventos_del_dia_info() {
                    Card { class: "mb-6",
                        CardHeader {
                            CardTitle { "Eventos del {fecha_en_espanol(date)}" }
                        }
                        CardContent {
                            div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-{cols}",
                                if eventos_del_dia.is_empty() {
                                    div { class: "col-span-full",
                                        EmptyState {
                                            emoji: "📅".to_string(),
                                            title: "No hay eventos".to_string(),
                                            message: "No hay eventos programados para este día.".to_string(),
                                        }
                                    }
                                } else {
                                    for (idx , evento) in eventos_del_dia {
                                        EventCardWithImage {
                                            key: "{idx}",
                                            evento: evento.clone(),
                                            index: idx as i32,
                                        }
                                    }
                                }
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
