use crate::components::ui::{Container, Section, Separator};
use crate::components::{EmptyState, FilterConfig, FilterSection, PageHeader, Pagination};
use crate::models::Renderable;
use dioxus::prelude::*;

#[component]
pub fn PaginatedListing<T: Renderable>(
    title: Option<String>,
    breadcrumb: Option<Element>,
    description: Option<String>,
    #[props(default = vec![])] filters: Vec<FilterConfig>,
    #[props(default = None)] search_bar: Option<Element>,
    #[props(default = None)] paginated_items: Option<Memo<Vec<T>>>,
    #[props(default = None)] current_page: Option<Signal<usize>>,
    #[props(default = 1)] total_pages: usize,
    #[props(default = "grid gap-6 md:grid-cols-2 lg:grid-cols-3".to_string())] grid_classes: String,
    #[props(default = None)] content: Option<Element>,
    show_empty_state: bool,
) -> Element {
    let empty = if content.is_some() {
        false
    } else if let Some(items) = &paginated_items {
        items().is_empty()
    } else {
        true
    };

    // Asegurar que siempre tenemos una señal de página: usar la proporcionada o crear una local por defecto
    let internal_current = use_signal(|| 1usize);
    let current_page_signal = current_page.clone().unwrap_or(internal_current);

    rsx! {
      Container {
        Section {
          div { class: "flex flex-col lg:flex-row justify-between items-center gap-6 mb-4",
            // Header (toma todo el ancho)
            if title.is_some() || breadcrumb.is_some() || description.is_some() {
                div { class: "w-full md:w-2/3 flex flex-col gap-2",
                  PageHeader {
                    breadcrumb,
                    title: title.unwrap_or_default(),
                    description,
                  }
              }
            }

            // FilterSection en ancho completo: su propio layout colocará search + filtros en md+ en la misma línea
            if !filters.is_empty() || search_bar.is_some() {
              div { class: "w-full md:w-1/3",
                FilterSection { filters, search: search_bar }
              }
            }
          }

          Separator { class: "my-4" }
          // Contenido principal o estado vacío
          if empty {
            if show_empty_state {
              EmptyState {
                emoji: "❌".to_string(),
                title: "No hay elementos".to_string(),
                message: "No hay resultados con los filtros seleccionados.".to_string(),
                action: rsx! {},
              }
            }
          } else {
            if let Some(slot) = content {
              {slot}
            } else if let Some(items_sig) = &paginated_items {
              div { class: "{grid_classes}",
                for (index , item) in items_sig().into_iter().enumerate() {
                  {item.render(index)}
                }
              }
            }

            // Paginación
            Pagination {
              current_page: current_page_signal,
              total_pages,
            }
          }
        }
      }
    }
}
