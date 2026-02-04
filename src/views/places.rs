use crate::components::ui::*;
use crate::components::{
    breadcrumb_items, Breadcrumb, CategoryCard, EmptyState, FilterConfig, FilterSection,
    PageHeader, PaginatedListing, Pagination,
};
use crate::models::{EspacioDeportivo, InstalacionesGeoJSON};
use crate::utils::pagination_filters::{paginate, total_pages};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

const INSTALACIONES_JSON: &str = include_str!("../../data/instalaciones-deportivas.geojson");
const ESPACIOS_JSON: &str = include_str!("../../data/espacios-deportivos.json");

/// The Places page component that will be rendered when the current route is `[Route::Places]`
#[component]
pub fn Places() -> Element {
    // Cargar instalaciones deportivas
    let instalaciones_data =
        use_memo(
            move || match serde_json::from_str::<InstalacionesGeoJSON>(INSTALACIONES_JSON) {
                Ok(data) => data,
                Err(_) => InstalacionesGeoJSON { features: vec![] },
            },
        );

    // Cargar espacios deportivos
    let espacios_data =
        use_memo(
            move || match serde_json::from_str::<EspacioDeportivoData>(ESPACIOS_JSON) {
                Ok(data) => data,
                Err(_) => EspacioDeportivoData {
                    espacios_deportivos: vec![],
                },
            },
        );

    // Agrupar espacios por instalación
    let espacios_por_instalacion = use_memo(move || {
        let espacios = espacios_data.read();
        let mut map: HashMap<i64, Vec<EspacioDeportivo>> = HashMap::new();
        for espacio in &espacios.espacios_deportivos {
            map.entry(espacio.instalacion_codigo)
                .or_insert_with(Vec::new)
                .push(espacio.clone());
        }
        map
    });

    let mut search_query = use_signal(|| String::new());
    let mut municipio_filter = use_signal(|| String::from("Todos"));
    let mut orden = use_signal(|| "nombre_az".to_string());
    let mut current_page = use_signal(|| 1);
    let items_per_page = 12;

    // Obtener municipios únicos
    let municipios_unicos = use_memo(move || {
        let data = instalaciones_data.read();
        let mut municipios: Vec<String> = data
            .features
            .iter()
            .map(|f| f.properties.municipio_nombre.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        municipios.sort();
        municipios
    });

    // Filtrar y ordenar instalaciones
    let instalaciones_filtradas = use_memo(move || {
        let data = instalaciones_data.read();
        let espacios_map = espacios_por_instalacion.read();
        let query = search_query().to_lowercase();
        let municipio = municipio_filter();
        let orden_val = orden();

        let mut filtered = data
            .features
            .iter()
            .filter(|feature| {
                let instalacion = &feature.properties;
                let tiene_espacios = espacios_map
                    .get(&instalacion.instalacion_codigo)
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);
                if !tiene_espacios {
                    return false;
                }
                // Filtro de búsqueda
                let matches_search = query.is_empty()
                    || instalacion
                        .instalacion_nombre
                        .to_lowercase()
                        .contains(&query)
                    || instalacion.municipio_nombre.to_lowercase().contains(&query);
                // Filtro de municipio
                let matches_municipio =
                    municipio == "Todos" || instalacion.municipio_nombre == municipio;
                // Buscar también en espacios de la instalación
                let matches_espacios = if !query.is_empty() {
                    if let Some(espacios) = espacios_map.get(&instalacion.instalacion_codigo) {
                        espacios.iter().any(|e| {
                            e.espacio_nombre.to_lowercase().contains(&query)
                                || e.espacio_tipo
                                    .as_ref()
                                    .map(|t| t.to_lowercase().contains(&query))
                                    .unwrap_or(false)
                        })
                    } else {
                        false
                    }
                } else {
                    false
                };
                (matches_search || matches_espacios) && matches_municipio
            })
            .cloned()
            .collect::<Vec<_>>();

        // Ordenar
        match orden_val.as_str() {
            "nombre_az" => filtered.sort_by(|a, b| {
                a.properties
                    .instalacion_nombre
                    .cmp(&b.properties.instalacion_nombre)
            }),
            "nombre_za" => filtered.sort_by(|a, b| {
                b.properties
                    .instalacion_nombre
                    .cmp(&a.properties.instalacion_nombre)
            }),
            "municipio" => filtered.sort_by(|a, b| {
                a.properties
                    .municipio_nombre
                    .cmp(&b.properties.municipio_nombre)
            }),
            "espacios_desc" => filtered.sort_by(|a, b| {
                let espacios_map = &espacios_por_instalacion.read();
                let a_count = espacios_map
                    .get(&a.properties.instalacion_codigo)
                    .map(|v| v.len())
                    .unwrap_or(0);
                let b_count = espacios_map
                    .get(&b.properties.instalacion_codigo)
                    .map(|v| v.len())
                    .unwrap_or(0);
                b_count.cmp(&a_count)
            }),
            "espacios_asc" => filtered.sort_by(|a, b| {
                let espacios_map = &espacios_por_instalacion.read();
                let a_count = espacios_map
                    .get(&a.properties.instalacion_codigo)
                    .map(|v| v.len())
                    .unwrap_or(0);
                let b_count = espacios_map
                    .get(&b.properties.instalacion_codigo)
                    .map(|v| v.len())
                    .unwrap_or(0);
                a_count.cmp(&b_count)
            }),
            _ => {}
        }

        filtered
    });

    // Calcular paginación
    let total_items = instalaciones_filtradas().len();
    let total_pages = total_pages(total_items, items_per_page);

    // Resetear página cuando cambian los filtros
    use_effect(move || {
        search_query();
        municipio_filter();
        orden();
        current_page.set(1);
    });

    // Obtener instalaciones de la página actual
    let instalaciones_paginadas = use_memo(move || {
        let filtered = instalaciones_filtradas();
        paginate(&filtered, current_page() as usize, items_per_page)
    });

    let filters = vec![
        FilterConfig {
            label: "Ordenar por:".to_string(),
            value: orden,
            options: vec![
                ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
                ("municipio".to_string(), "Municipio".to_string()),
                (
                    "espacios_desc".to_string(),
                    "Más espacios primero".to_string(),
                ),
                (
                    "espacios_asc".to_string(),
                    "Menos espacios primero".to_string(),
                ),
            ],
            on_change: EventHandler::new(move |val: String| orden.set(val)),
        },
        FilterConfig {
            label: "Municipio:".to_string(),
            value: municipio_filter,
            options: {
                let mut opts = vec![("Todos".to_string(), "Todos los municipios".to_string())];
                for municipio in municipios_unicos() {
                    opts.push((municipio.clone(), municipio));
                }
                opts
            },
            on_change: EventHandler::new(move |val: String| {
                municipio_filter.set(val);
            }),
        },
    ];

    rsx! {
        PaginatedListing {
            title: Some("Instalaciones Deportivas".to_string()),
            breadcrumb: Some(rsx! {
                Breadcrumb { items: breadcrumb_items!(("Inicio", Route::Home {}), ("Instalaciones", Route::Places {})) }
            }),
            description: Some(
                format!(
                    "Mostrando {} de {} instalaciones",
                    instalaciones_paginadas().len(),
                    total_items,
                ),
            ),
            featured: None,
            filters,
            content: rsx! {
                // Barra de búsqueda
                div { class: "mb-6",
                    input {
                        r#type: "text",
                        placeholder: "Buscar instalaciones...",
                        value: "{search_query}",
                        class: "w-full px-4 py-2 rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-zinc-950 dark:focus:ring-zinc-300",
                        oninput: move |evt| search_query.set(evt.value()),
                    }
                }
                div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-4",
                    if instalaciones_paginadas().is_empty() {
                        div { class: "col-span-full",
                            EmptyState {
                                emoji: "🏟️".to_string(),
                                title: "No hay instalaciones disponibles".to_string(),
                                message: "No se encontraron instalaciones con los filtros seleccionados.".to_string(),
                            }
                        }
                    }
                    for feature in instalaciones_paginadas() {
                        {
                            let instalacion = &feature.properties;
                            let espacios_map = espacios_por_instalacion.read();
                            let num_espacios = espacios_map
                                .get(&instalacion.instalacion_codigo)
                                .map(|v| v.len())
                                .unwrap_or(0);

                            let badge_text = if num_espacios == 1 {
                                format!("1 espacio")
                            } else {
                                format!("{} espacios", num_espacios)
                            };

                            rsx! {
                                Link {
                                    to: Route::Place {
                                        id: instalacion.instalacion_codigo,
                                    },
                                    class: "no-underline",
                                    CategoryCard {
                                        title: instalacion.instalacion_nombre.clone(),
                                        badge_text,
                                        description: Some(format!("📍 {}", instalacion.municipio_nombre)),
                                    }
                                }
                            }
                        }
                    }
                }
            },
            paginated_items: None,
            current_page,
            total_pages,
            items_per_page,
            item_layout: None,
            show_empty_state: false,
        }
    }
}

#[derive(serde::Deserialize, PartialEq)]
struct EspacioDeportivoData {
    espacios_deportivos: Vec<EspacioDeportivo>,
}
