use crate::components::ui::*;
use crate::utils::listing_helpers::vec_to_options;
use dioxus::prelude::*;

/// Estructura para definir un filtro individual
#[derive(Clone, PartialEq, Props)]
pub struct FilterConfig {
    pub label: String,
    pub value: Signal<String>,
    pub options: Vec<(String, String)>, // (value, label)
    pub on_change: EventHandler<String>,
}

/// Wrapper para compatibilidad: construir filtros desde especificadores.
impl From<FilterSpec> for Vec<FilterSpec> {
    fn from(s: FilterSpec) -> Vec<FilterSpec> {
        vec![s]
    }
}

pub fn make_filters<S: Into<Vec<FilterSpec>>>(specs: S) -> Vec<FilterConfig> {
    make_filters_from_specs(specs.into())
}

// Macro para permitir `filters!(FilterSpec::Orden(...), FilterSpec::Estado(...))`
#[macro_export]
macro_rules! filters {
    ( $( $spec:expr ),* $(,)? ) => {
        crate::components::filter_section::make_filters(vec![$($spec),*])
    };
}

/// Especificador de filtro para `make_filters` más expresivo.
#[derive(Clone)]
pub enum FilterSpec {
    Orden(Signal<String>, Option<Vec<(String, String)>>),
    Estado(Signal<String>, Option<Vec<(String, String)>>),
    Deporte(
        Signal<String>,
        Option<Vec<(String, String)>>,
        Option<Vec<String>>,
    ),
    Municipio(
        Signal<String>,
        Option<Vec<(String, String)>>,
        Option<Vec<String>>,
    ),
    Custom {
        label: String,
        value: Signal<String>,
        options: Vec<(String, String)>,
    },
}

/// Construye filtros a partir de un vector de `FilterSpec`.
pub fn make_filters_from_specs(specs: Vec<FilterSpec>) -> Vec<FilterConfig> {
    let mut out: Vec<FilterConfig> = Vec::new();

    for spec in specs {
        match spec {
            FilterSpec::Orden(signal, opts) => {
                let mut s = signal.clone();
                let opciones = opts.unwrap_or_else(|| {
                    vec![
                        ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                        ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
                        (
                            "fecha_asc".to_string(),
                            "Fecha (próximos primero)".to_string(),
                        ),
                        (
                            "fecha_desc".to_string(),
                            "Fecha (lejanos primero)".to_string(),
                        ),
                        (
                            "eventos_desc".to_string(),
                            "Más eventos primero".to_string(),
                        ),
                        (
                            "eventos_asc".to_string(),
                            "Menos eventos primero".to_string(),
                        ),
                        (
                            "espacios_desc".to_string(),
                            "Más espacios primero".to_string(),
                        ),
                        (
                            "espacios_asc".to_string(),
                            "Menos espacios primero".to_string(),
                        ),
                    ]
                });
                out.push(FilterConfig {
                    label: "Ordenar por:".to_string(),
                    value: signal.clone(),
                    options: opciones,
                    on_change: EventHandler::new(move |v: String| s.set(v)),
                });
            }
            FilterSpec::Estado(signal, opts) => {
                let mut s = signal.clone();
                let opciones = opts.unwrap_or_else(|| {
                    vec![
                        ("PRÓXIMO".to_string(), "Próximos".to_string()),
                        ("EN VIVO".to_string(), "En Vivo".to_string()),
                        ("FINALIZADO".to_string(), "Finalizados".to_string()),
                        ("Todos".to_string(), "Todos los estados".to_string()),
                    ]
                });
                out.push(FilterConfig {
                    label: "Estado:".to_string(),
                    value: signal.clone(),
                    options: opciones,
                    on_change: EventHandler::new(move |v: String| s.set(v)),
                });
            }
            FilterSpec::Deporte(signal, opts, maybe_values) => {
                let mut s = signal.clone();
                let opciones = if let Some(o) = opts {
                    o
                } else if let Some(vals) = maybe_values {
                    vec_to_options(vals, Some(("Todos", "Todos los deportes")))
                } else {
                    vec![("Todos".to_string(), "Todos los deportes".to_string())]
                };
                out.push(FilterConfig {
                    label: "Deporte:".to_string(),
                    value: signal.clone(),
                    options: opciones,
                    on_change: EventHandler::new(move |v: String| s.set(v)),
                });
            }
            FilterSpec::Municipio(signal, opts, maybe_values) => {
                let mut s = signal.clone();
                let opciones = if let Some(o) = opts {
                    o
                } else if let Some(vals) = maybe_values {
                    let mut res = vec![("Todos".to_string(), "Todos los municipios".to_string())];
                    for v in vals {
                        res.push((v.clone(), v));
                    }
                    res
                } else {
                    vec![("Todos".to_string(), "Todos los municipios".to_string())]
                };
                out.push(FilterConfig {
                    label: "Municipio:".to_string(),
                    value: signal.clone(),
                    options: opciones,
                    on_change: EventHandler::new(move |v: String| s.set(v)),
                });
            }
            FilterSpec::Custom {
                label,
                value,
                options,
            } => {
                let mut s = value.clone();
                out.push(FilterConfig {
                    label,
                    value: value.clone(),
                    options,
                    on_change: EventHandler::new(move |v: String| s.set(v)),
                });
            }
        }
    }

    out
}

/// Componente reutilizable para sección de filtros
#[component]
pub fn FilterSection(
    filters: Vec<FilterConfig>,
    #[props(default = None)] search: Option<Element>,
) -> Element {
    let num_filters = filters.len();
    let grid_class = match num_filters {
        1 => "grid gap-4",
        2 => "grid gap-4 md:grid-cols-2",
        3 => "grid gap-4 md:grid-cols-3",
        _ => "grid gap-4 md:grid-cols-2 lg:grid-cols-4",
    };

    rsx! {
        // Contenedor responsive: columna en móvil, fila en md+
        div { class: "flex flex-col justify-between gap-4",
             // Derecha: Search (ancho fijo en md)
                if let Some(s) = search {
                    {s}
                }
                
            // Izquierda: filtros (ocupan el espacio disponible)
                div { class: "{grid_class}",
                    for filter in filters {
                        div {
                            
                            Select {
                                value: (filter.value)(),
                                onchange: move |val: String| filter.on_change.call(val),
                                for (value , label) in filter.options {
                                    option { key: "{value}", value: "{value}", "{label}" }
                                }
                            }
                            // label { class: "block text-sm font-medium text-zinc-700 dark:text-zinc-300 mt-2",
                            //     "{filter.label}"
                            // }
                        }
                    }
                }


        }
    }
}
