use crate::components::{breadcrumb_items, Breadcrumb, FilterSpec, PaginatedListing, SearchBar};
use crate::data::{get_espacios, get_instalaciones};
use crate::filters;
use crate::models::{EspacioDeportivo, InstalacionItem};
use crate::utils::pagination_filters::{
    paginate, total_pages, use_page_reset, DEFAULT_ITEMS_PER_PAGE,
};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

/// The Places page component that will be rendered when the current route is `[Route::Places]`
#[component]
pub fn Places() -> Element {
    // Cargar instalaciones deportivas y espacios (cacheados)
    // `get_instalaciones()` y `get_espacios()` devuelven referencias cacheadas

    // Agrupar espacios por instalación
    let espacios_por_instalacion = use_memo(move || {
        let espacios = get_espacios();
        let mut map: HashMap<i64, Vec<EspacioDeportivo>> = HashMap::new();
        for espacio in espacios.iter() {
            map.entry(espacio.instalacion_codigo)
                .or_insert_with(Vec::new)
                .push(espacio.clone());
        }
        map
    });

    let search_query = use_signal(|| String::new());
    let municipio_filter = use_signal(|| String::from("Todos"));
    let orden = use_signal(|| "nombre_az".to_string());
    let current_page = use_signal(|| 1);
    let items_per_page = DEFAULT_ITEMS_PER_PAGE;

    // Resetear página cuando cambian los filtros
    use_page_reset(current_page, move || {
        search_query();
        municipio_filter();
        orden();
    });

    // Obtener municipios únicos
    let municipios_unicos = use_memo(move || {
        let data = get_instalaciones();
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
        let data = get_instalaciones();
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

        // Mapear a InstalacionItem con el conteo de espacios
        filtered
            .into_iter()
            .map(|feature| {
                let espacios_count = espacios_por_instalacion
                    .read()
                    .get(&feature.properties.instalacion_codigo)
                    .map(|v| v.len())
                    .unwrap_or(0);

                InstalacionItem {
                    feature,
                    espacios_count,
                }
            })
            .collect::<Vec<_>>()
    });

    // Calcular paginación
    let total_items = instalaciones_filtradas().len();
    let total_pages = total_pages(total_items, items_per_page);

    // Obtener instalaciones de la página actual
    let instalaciones_paginadas = use_memo(move || {
        let filtered = instalaciones_filtradas();
        paginate(&filtered, current_page() as usize, items_per_page)
    });

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
            filters: filters!(
                FilterSpec::Orden(orden, Some(vec![
                    ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                    ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
                    ("espacios_desc".to_string(), "Más espacios primero".to_string()),
                    ("espacios_asc".to_string(), "Menos espacios primero".to_string()),
                ])),
                FilterSpec::Municipio(municipio_filter, None, Some(municipios_unicos())),
            ),
            search_bar: Some(rsx! { SearchBar { value: search_query, placeholder: "Buscar instalaciones...".to_string() } }),
            grid_classes: "grid gap-6 md:grid-cols-2 lg:grid-cols-4",
            paginated_items: Some(instalaciones_paginadas),
            current_page: Some(current_page),
            total_pages,
            show_empty_state: true,
        }
    }
}
