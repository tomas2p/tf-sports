use crate::components::{breadcrumb_items, Breadcrumb, FilterSpec, PaginatedListing, SearchBar, EventCard};
use crate::components::event_card::LayoutVariant;
use crate::data::get_eventos;
use crate::filters;
use crate::models::{normalize_text, strip_html};
use crate::utils::pagination_filters::{
    paginate, total_pages, use_page_reset, DEFAULT_ITEMS_PER_PAGE,
};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Sport(category: String) -> Element {
    // Cargar eventos desde JSON (cacheados) — mantener referencia sin clonar todo
    let eventos_data = use_memo(move || get_eventos());

    // Indexar eventos por deporte normalizado para iterar solo el subconjunto necesario
    let eventos_por_deporte = use_memo(move || {
        let data = eventos_data();
        let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, e) in data.eventos.iter().enumerate() {
            let deporte = normalize_text(&e.get_deporte());
            map.entry(deporte).or_default().push(i);
        }
        map
    });

    // Signals para filtros, búsqueda y paginación
    let filter_organizador = use_signal(|| "Todos".to_string());
    let filter_municipio = use_signal(|| "Todos".to_string());
    let orden = use_signal(|| "fecha_asc".to_string());
    let page = use_signal(|| 1);
    let search_query = use_signal(|| String::new());
    let items_per_page = DEFAULT_ITEMS_PER_PAGE;

    // Resetear página cuando cambian los filtros
    use_page_reset(page, move || {
        filter_organizador();
        filter_municipio();
        orden();
        search_query();
    });

    // Precomputar la versión normalizada de la categoría (se usa en varios closures)
    let normalized_category = use_signal(|| normalize_text(&category));
    // Obtener listas únicas para filtros (solo de eventos de este deporte)
    let organizadores_disponibles = use_memo(move || {
        let data = eventos_data();
        let deporte_actual = normalized_category();

        let mut orgs: Vec<String> = Vec::new();
        if let Some(indices) = eventos_por_deporte().get(&deporte_actual) {
            for i in indices {
                if let Some(e) = data.eventos.get(*i) {
                    orgs.push(e.evento_organizador.clone());
                }
            }
        }
        orgs.sort();
        orgs.dedup();
        orgs
    });

    let municipios_disponibles = use_memo(move || {
        let data = eventos_data();
        let deporte_actual = normalized_category();

        let mut munis: Vec<String> = Vec::new();
        if let Some(indices) = eventos_por_deporte().get(&deporte_actual) {
            for i in indices {
                if let Some(e) = data.eventos.get(*i) {
                    if let Some(m) = e.municipio_nombre.clone() {
                        munis.push(m);
                    }
                }
            }
        }
        munis.sort();
        munis.dedup();
        munis
    });

    // Filtrar y ordenar eventos
    let eventos_filtrados = use_memo(move || {
        let data = eventos_data();
        let deporte_actual = normalized_category();
        let org_val = filter_organizador();
        let muni_val = filter_municipio();
        let orden_val = orden();
        let query = normalize_text(&search_query());

        // Solo iterar índices del deporte actual usando el índice precomputado
        let mut filtered: Vec<(usize, crate::models::Evento)> = Vec::new();
        if let Some(indices) = eventos_por_deporte().get(&deporte_actual) {
            for i in indices {
                if let Some(e) = data.eventos.get(*i) {
                    let org_match = if org_val == "Todos" {
                        true
                    } else {
                        e.evento_organizador == org_val
                    };

                    let muni_match = if muni_val == "Todos" {
                        true
                    } else {
                        e.municipio_nombre.as_ref() == Some(&muni_val)
                    };

                    let search_match = if query.is_empty() {
                        true
                    } else {
                        let nombre = normalize_text(&strip_html(&e.evento_nombre));
                        let desc = normalize_text(&strip_html(&e.evento_descripcion));
                        let lugar = normalize_text(&e.evento_lugar.clone().unwrap_or_default());
                        let municipio =
                            normalize_text(&e.municipio_nombre.clone().unwrap_or_default());
                        let organizador = normalize_text(&e.evento_organizador);

                        nombre.contains(&query)
                            || desc.contains(&query)
                            || lugar.contains(&query)
                            || municipio.contains(&query)
                            || organizador.contains(&query)
                    };

                    if org_match && muni_match && search_match {
                        filtered.push((*i, e.clone()));
                    }
                }
            }
        }

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
            "organizador" => {
                filtered.sort_by(|a, b| a.1.evento_organizador.cmp(&b.1.evento_organizador))
            }
            "municipio" => filtered.sort_by(|a, b| a.1.municipio_nombre.cmp(&b.1.municipio_nombre)),
            _ => {}
        }

        filtered
    });

    // Paginación
    let eventos_paginados = use_memo(move || {
        let eventos = eventos_filtrados();
        paginate(&eventos, page() as usize, items_per_page)
    });

    let total_pages = use_memo(move || {
        let total = eventos_filtrados().len();
        total_pages(total, items_per_page)
    });

    // No hacer early-return: dejar que `PaginatedListing` muestre el estado vacío

    let filters = filters!(
        FilterSpec::Orden(
            orden,
            Some(vec![
                (
                    "fecha_asc".to_string(),
                    "Fecha (próximos primero)".to_string()
                ),
                (
                    "fecha_desc".to_string(),
                    "Fecha (lejanos primero)".to_string()
                ),
                ("nombre_az".to_string(), "Nombre (A-Z)".to_string()),
                ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
            ])
        ),
        FilterSpec::Custom {
            label: "Organizador:".to_string(),
            value: filter_organizador,
            options: {
                let mut opts = vec![("Todos".to_string(), "Todos los organizadores".to_string())];
                for org in organizadores_disponibles() {
                    opts.push((org.clone(), org));
                }
                opts
            },
        },
        FilterSpec::Municipio(filter_municipio, None, Some(municipios_disponibles())),
    );

    let content = rsx! {
        div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
            for (_index, (orig_idx, evento)) in eventos_paginados().into_iter().enumerate() {
                EventCard {
                    evento: evento,
                    index: orig_idx as i32,
                    layout: LayoutVariant::Detailed,
                    breadcrumb_override: Some(breadcrumb_items!(
                        ("Inicio", Route::Home {}), ("Deportes", Route::Sports {}), (category.clone())
                    )),
                }
            }
        }
    };

    rsx! {
        PaginatedListing::<(usize, crate::models::Evento)> {
            title: Some(format!("{} en Tenerife", category)),
            breadcrumb: Some(rsx! {
                Breadcrumb {
                    items: breadcrumb_items!(
                        ("Inicio", Route::Home {}), ("Deportes", Route::Sports {}), (category.clone())
                    ),
                }
            }),
            description: Some(
                format!(
                    "Mostrando {} de {} eventos",
                    eventos_paginados().len(),
                    eventos_filtrados().len(),
                ),
            ),
            filters,
            search_bar: Some(rsx! { SearchBar { value: search_query, placeholder: format!("Buscar en {}...", category) } }),
            grid_classes: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
            content: Some(content),
            current_page: page,
            total_pages: total_pages(),
            show_empty_state: true,
        }
    }
}
