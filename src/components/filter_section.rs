use crate::components::ui::*;
use dioxus::prelude::*;

/// Estructura para definir un filtro individual
#[derive(Clone, PartialEq, Props)]
pub struct FilterConfig {
    pub label: String,
    pub value: Signal<String>,
    pub options: Vec<(String, String)>, // (value, label)
    pub on_change: EventHandler<String>,
}

/// Componente reutilizable para sección de filtros
#[component]
pub fn FilterSection(filters: Vec<FilterConfig>) -> Element {
    let num_filters = filters.len();
    let grid_class = match num_filters {
        1 => "grid gap-4",
        2 => "grid gap-4 md:grid-cols-2",
        3 => "grid gap-4 md:grid-cols-3",
        _ => "grid gap-4 md:grid-cols-2 lg:grid-cols-4",
    };
    
    rsx! {
        div { class: "{grid_class} mb-8",
            for filter in filters {
                div {
                    label { class: "block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2",
                        "{filter.label}"
                    }
                    Select {
                        value: (filter.value)(),
                        onchange: move |val: String| filter.on_change.call(val),
                        for (value , label) in filter.options {
                            option { key: "{value}", value: "{value}", "{label}" }
                        }
                    }
                }
            }
        }
    }
}
