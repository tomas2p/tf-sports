use crate::components::event_card::LayoutVariant;
use crate::components::{breadcrumb_items, Breadcrumb, CategoryCard, PaginatedListing};
use crate::models::{EventoData, DEPORTES};
use crate::Route;
use dioxus::prelude::*;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

/// The Sports page component showing all available sports categories
#[component]
pub fn Sports() -> Element {
    // Cargar eventos para contar por deporte
    let eventos_data = use_memo(
        move || match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
            Ok(data) => data,
            Err(_) => EventoData { eventos: vec![] },
        },
    );

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

    // Paginación local para la lista de deportes
    let items_per_page: usize = 12;
    let current_page = use_signal(|| 1usize);
    let total_pages = {
        let total = DEPORTES.len();
        if total == 0 {
            1
        } else {
            (total + items_per_page - 1) / items_per_page
        }
    };

    // Deportes para la página actual
    let paged_deportes = use_memo({
        let current_page = current_page.clone();
        move || {
            let page = current_page();
            let start = (page.saturating_sub(1)) * items_per_page;
            let end = (start + items_per_page).min(DEPORTES.len());
            DEPORTES[start..end].iter().cloned().collect::<Vec<_>>()
        }
    });

    rsx! {
      PaginatedListing {
        title: Some("Deportes en Tenerife".to_string()),
        breadcrumb: Some(rsx! {
          Breadcrumb { items: breadcrumb_items!(("Inicio", Route::Home {}), ("Deportes", Route::Sports {})) }
        }),
        description: Some(format!("Mostrando {} deportes disponibles en la isla", DEPORTES.len())),
        item_layout: Some(LayoutVariant::Simple),
        current_page: Some(current_page.clone()),
        total_pages,
        items_per_page,
        content: Some(rsx! {
          div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
            for deporte in paged_deportes() {
              {
                  let conteo = eventos_por_deporte();
                  let cantidad = conteo.get(deporte.nombre).copied().unwrap_or(0);
                  let badge_text = if cantidad == 1 {
                      format!("{} evento", cantidad)
                  } else {
                      format!("{} eventos", cantidad)
                  };

                  rsx! {
                    Link {
                      to: Route::Sport {
                          category: deporte.nombre.to_string(),
                      },
                      class: "no-underline",
                      CategoryCard {
                        emoji: deporte.emoji.to_string(),
                        title: deporte.nombre.to_string(),
                        badge_text,
                        description: Some(format!("Descubre eventos de {} en Tenerife", deporte.nombre.to_lowercase())),
                      }
                    }
                  }
              }
            }
          }
        }),
        paginated_items: None,
        show_empty_state: true,
      }
    }
}
