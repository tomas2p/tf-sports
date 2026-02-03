use crate::components::ui::*;
use crate::models::Evento;
use crate::utils::date_utils::{day_name_short_es};
use dioxus::prelude::*;
use chrono::{NaiveDate, Datelike, Duration};

/// Componente de calendario semanal compacto
#[component]
pub fn Calendar(
    eventos: Vec<Evento>,
    selected_date: Signal<Option<NaiveDate>>,
    current_week_start: Signal<NaiveDate>,
) -> Element {
    let today = chrono::Local::now().date_naive();
    
    // Calcular días de la semana (7 días desde current_week_start)
    let week_days: Vec<NaiveDate> = (0..7)
        .map(|i| current_week_start() + Duration::days(i))
        .collect();
    
    // Agrupar eventos por fecha
    let eventos_por_fecha = use_memo(move || {
        let mut map: std::collections::HashMap<NaiveDate, usize> = std::collections::HashMap::new();
        for evento in eventos.iter() {
            if let Ok(fecha) = chrono::NaiveDateTime::parse_from_str(&evento.evento_fecha_inicio, "%Y-%m-%d %H:%M:%S") {
                let fecha_date = fecha.date();
                *map.entry(fecha_date).or_insert(0) += 1;
            }
        }
        map
    });
    
    // Activar siempre el día actual como seleccionado si no hay selección
    use_effect(move || {
        if selected_date().is_none() {
            selected_date.set(Some(today));
        }
    });
    
    let month_name = match current_week_start().month() {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "",
    };
    
    rsx! {
        Card { class: "mb-6",
            CardContent { class: "p-4",
                // Header con navegación
                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-base font-semibold text-zinc-950 dark:text-zinc-50",
                        "{month_name} {current_week_start().year()}"
                    }
                    div { class: "flex gap-1",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                current_week_start.set(current_week_start() - Duration::weeks(1));
                            },
                            "←"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                // Encontrar el lunes de esta semana
                                let today = chrono::Local::now().date_naive();
                                let days_from_monday = today.weekday().num_days_from_monday();
                                let monday = today - Duration::days(days_from_monday as i64);
                                current_week_start.set(monday);
                            },
                            "Hoy"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                current_week_start.set(current_week_start() + Duration::weeks(1));
                            },
                            "→"
                        }
                    }
                }

                // Días de la semana
                div { class: "grid grid-cols-7 gap-2",
                    for date in week_days.iter() {
                        {
                            let is_today = *date == today;
                            let is_selected = selected_date().map(|d| d == *date).unwrap_or(false);
                            let eventos_count = eventos_por_fecha().get(date).copied().unwrap_or(0);
                            let date_clone = *date;

                            let day_name = day_name_short_es(date.weekday());

                            rsx! {
                                button {
                                    class: format!(
                                        "flex flex-col items-center justify-center p-3 rounded-lg border transition-all {}",
                                        if is_selected {
                                            "border-zinc-900 dark:border-zinc-100 bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900"
                                        } else if is_today {
                                            "border-zinc-400 dark:border-zinc-600 bg-zinc-100 dark:bg-zinc-800"
                                        } else {
                                            "border-zinc-200 dark:border-zinc-800 hover:border-zinc-400 dark:hover:border-zinc-600 hover:bg-zinc-50 dark:hover:bg-zinc-900"
                                        },
                                    ),
                                    onclick: move |_| {
                                        selected_date.set(Some(date_clone));
                                    },

                                    span { class: "text-xs font-medium mb-1", "{day_name}" }
                                    span { class: "text-lg font-bold", "{date.day()}" }
                                    if eventos_count > 0 {
                                        div { class: "flex gap-1 mt-1",
                                            for _ in 0..eventos_count.min(3) {
                                                div {
                                                    class: format!(
                                                        "w-1 h-1 rounded-full {}",
                                                        if is_selected {
                                                            "bg-white dark:bg-zinc-900"
                                                        } else {
                                                            "bg-zinc-400 dark:bg-zinc-600"
                                                        },
                                                    ),
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
