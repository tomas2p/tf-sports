use crate::components::{breadcrumb_items, Breadcrumb, FilterSpec, PaginatedListing, SearchBar};
use crate::data::get_eventos;
use crate::filters;
use crate::models::Evento;
use crate::utils::listing_helpers::unique_sorted;
use crate::utils::pagination_filters::{
    paginate, total_pages, use_page_reset, DEFAULT_ITEMS_PER_PAGE,
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Events() -> Element {
    // Cargar eventos desde JSON (cacheados)
    let eventos_data = use_memo(move || get_eventos().clone());

    // Signals para filtros y paginación
    let filter_estado = use_signal(|| "PRÓXIMO".to_string());
    let filter_deporte = use_signal(|| "Todos".to_string());
    let orden = use_signal(|| "fecha_asc".to_string());
    let page = use_signal(|| 1);
    let search_query = use_signal(|| String::new());
    let items_per_page = DEFAULT_ITEMS_PER_PAGE;

    // Resetear página cuando cambian los filtros
    use_page_reset(page, move || {
        filter_estado();
        filter_deporte();
        orden();
        search_query();
    });

    // Obtener lista de deportes únicos
    let deportes_disponibles = use_memo(move || {
        let data = eventos_data();
        let deportes: Vec<String> = data.eventos.iter().map(|e| e.get_deporte()).collect();
        unique_sorted(deportes)
    });

    // Filtrar eventos según los filtros seleccionados
    let eventos_filtrados = use_memo(move || {
        let data = eventos_data();
        let estado_val = filter_estado();
        let deporte_val = filter_deporte();
        let orden_val = orden();
        let query = search_query().to_lowercase();

        let mut filtered: Vec<Evento> = data
            .eventos
            .into_iter()
            .filter(|e| {
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

                let search_match = if query.is_empty() {
                    true
                } else {
                    let nombre = e.evento_nombre.to_lowercase();
                    let desc = e.evento_descripcion.to_lowercase();
                    let lugar = e.evento_lugar.clone().unwrap_or_default().to_lowercase();
                    let municipio = e
                        .municipio_nombre
                        .clone()
                        .unwrap_or_default()
                        .to_lowercase();
                    let organizador = e.evento_organizador.to_lowercase();

                    nombre.contains(&query)
                        || desc.contains(&query)
                        || lugar.contains(&query)
                        || municipio.contains(&query)
                        || organizador.contains(&query)
                };

                estado_match && deporte_match && search_match
            })
            .collect();

        // Ordenar
        match orden_val.as_str() {
            "fecha_asc" => {
                filtered.sort_by(|a, b| a.evento_fecha_inicio.cmp(&b.evento_fecha_inicio))
            }
            "fecha_desc" => {
                filtered.sort_by(|a, b| b.evento_fecha_inicio.cmp(&a.evento_fecha_inicio))
            }
            "nombre_az" => filtered.sort_by(|a, b| a.evento_nombre.cmp(&b.evento_nombre)),
            "nombre_za" => filtered.sort_by(|a, b| b.evento_nombre.cmp(&a.evento_nombre)),
            "deporte" => filtered.sort_by(|a, b| a.get_deporte().cmp(&b.get_deporte())),
            _ => {}
        }

        web_sys::console::log_1(&format!("Eventos filtrados: {}", filtered.len()).into());
        filtered
    });

    // Paginación
    let eventos_paginados = use_memo(move || {
        let eventos = eventos_filtrados();
        paginate(&eventos, page() as usize, items_per_page)
    });

    // Calcular total de páginas
    let total_pages = use_memo(move || {
        let total = eventos_filtrados().len();
        total_pages(total, items_per_page)
    });

    rsx! {
        PaginatedListing {
            title: Some("Eventos Deportivos".to_string()),
            breadcrumb: Some(rsx! {
                Breadcrumb { items: breadcrumb_items!(("Inicio", Route::Home {}), ("Eventos", Route::Events {})) }
            }),
            description: Some(
                format!(
                    "Mostrando {} de {} eventos",
                    eventos_paginados().len(),
                    eventos_filtrados().len(),
                ),
            ),
            filters: filters!(
                FilterSpec::Orden(orden, Some(vec![
                    ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
                ("fecha_asc".to_string(), "Fecha (próximos primero)".to_string()),
                ("fecha_desc".to_string(), "Fecha (lejanos primero)".to_string())
                ])),
                FilterSpec::Estado(filter_estado, None),
                FilterSpec::Deporte(filter_deporte,                None, Some(deportes_disponibles())),
            ),
            search_bar: Some(rsx! { SearchBar { value: search_query, placeholder: "Buscar eventos...".to_string() } }),
            grid_classes: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
            paginated_items: eventos_paginados,
            current_page: page,
            total_pages: total_pages(),
            show_empty_state: true,
        }
    }
}
