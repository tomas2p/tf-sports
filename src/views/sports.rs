use crate::components::{breadcrumb_items, Breadcrumb, FilterSpec, PaginatedListing, SearchBar};
use crate::data::get_eventos;
use crate::models::{DeporteItem, DEPORTES};
use crate::utils::pagination_filters::{
    paginate, total_pages, use_page_reset, DEFAULT_ITEMS_PER_PAGE,
};
use crate::Route;
use dioxus::prelude::*;

use crate::filters;

/// The Sports page component showing all available sports categories
#[component]
pub fn Sports() -> Element {
    // Cargar eventos para contar por deporte (cacheados)
    let eventos_data = use_memo(move || get_eventos().clone());

    // Contar eventos por deporte
    let eventos_por_deporte = use_memo(move || {
        let data = eventos_data();
        let mut conteo: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for evento in data.eventos.iter() {
            let deporte = evento.get_deporte();
            *conteo.entry(deporte).or_insert(0) += 1;
        }

        conteo
    });

    // Estado para orden, búsqueda y paginación local para la lista de deportes
    let orden = use_signal(|| "nombre_az".to_string());
    let search_query = use_signal(|| String::new());
    let items_per_page = DEFAULT_ITEMS_PER_PAGE;
    let current_page = use_signal(|| 1usize);

    // Lista filtrada y ordenada completa (antes de paginar)
    let filtered_deportes = {
        let orden = orden.clone();
        let search_query = search_query.clone();
        let eventos_por_deporte = eventos_por_deporte.clone();
        use_memo(move || {
            let conteo = eventos_por_deporte();
            let mut items: Vec<DeporteItem> = DEPORTES
                .iter()
                .map(|info| DeporteItem {
                    info,
                    eventos_count: conteo.get(info.nombre).copied().unwrap_or(0),
                })
                .collect();

            // Filtrar por búsqueda
            let q = search_query().to_lowercase();
            if !q.is_empty() {
                items.retain(|i| {
                    i.info.nombre.to_lowercase().contains(&q)
                        || i.info.keyword.to_lowercase().contains(&q)
                });
            }

            // Ordenar según `orden`
            match orden().as_str() {
                "nombre_az" => items.sort_by(|a, b| a.info.nombre.cmp(&b.info.nombre)),
                "nombre_za" => items.sort_by(|a, b| b.info.nombre.cmp(&a.info.nombre)),
                "eventos_desc" => items.sort_by(|a, b| b.eventos_count.cmp(&a.eventos_count)),
                "eventos_asc" => items.sort_by(|a, b| a.eventos_count.cmp(&b.eventos_count)),
                _ => {}
            }

            items
        })
    };

    // Calcular total de páginas a partir de la lista filtrada
    let total_pages = {
        let fd = filtered_deportes.clone();
        let memo = use_memo(move || {
            let total = fd().len();
            total_pages(total, items_per_page)
        });
        memo()
    };

    // Resetear página cuando cambian filtros relevantes
    use_page_reset(current_page, move || {
        orden();
        search_query();
    });

    // Deportes para la página actual como DeporteItem, con orden aplicado
    let paged_deportes = {
        let current_page = current_page.clone();
        let fd = filtered_deportes.clone();
        use_memo(move || {
            let page = current_page();
            let items = fd();
            paginate(&items, page as usize, items_per_page)
        })
    };

    rsx! {
      PaginatedListing {
        title: Some("Deportes".to_string()),
        breadcrumb: Some(rsx! {
          Breadcrumb { items: breadcrumb_items!(("Inicio", Route::Home {}), ("Deportes", Route::Sports {})) }
        }),
        description: Some(format!("Mostrando {} deportes disponibles en la isla", filtered_deportes().len())),
                grid_classes: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
                current_page: Some(current_page),
                search_bar: Some(rsx! { SearchBar { value: search_query, placeholder: "Buscar deportes...".to_string() } }),
        filters: filters!(
            FilterSpec::Orden(orden, Some(vec![("nombre_az".to_string(), "Nombre (A-Z)"
            .to_string()), ("nombre_za".to_string(), "Nombre (Z-A)".to_string()),
            ("eventos_desc".to_string(), "Más eventos primero".to_string()), ("eventos_asc"
            .to_string(), "Menos eventos primero".to_string())]))
        ),
        total_pages,
        paginated_items: Some(paged_deportes),
        show_empty_state: true,
      }
    }
}
