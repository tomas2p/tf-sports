use crate::components::event_card::LayoutVariant;
use crate::components::ui::{Badge, BadgeVariant, Button, Card, Container, Section, Separator};
use crate::components::{
    Breadcrumb, EmptyState, EventCard, FilterConfig, FilterSection, PageHeader, Pagination,
};
use crate::models::Evento;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PaginatedListing(
    title: Option<String>,
    breadcrumb: Option<Element>,
    description: Option<String>,
    #[props(default = None)] featured: Option<(usize, Evento)>,
    #[props(default = vec![])] filters: Vec<FilterConfig>,
    #[props(default = None)] paginated_items: Option<Memo<Vec<(usize, Evento)>>>,
    #[props(default = None)] current_page: Option<Signal<usize>>,
    #[props(default = 1)] total_pages: usize,
    #[props(default = 0)] items_per_page: usize,
    item_layout: Option<LayoutVariant>,
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
    let layout_choice = item_layout.unwrap_or(LayoutVariant::Simple);
    let grid_classes = match layout_choice {
        LayoutVariant::Detailed => "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
        _ => "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
    };

    // Asegurar que siempre tenemos una señal de página: usar la proporcionada o crear una local por defecto
    let internal_current = use_signal(|| 1usize);
    let current_page_signal = current_page.clone().unwrap_or(internal_current);

    rsx! {
        Container {
            Section {
                // Header
                if title.is_some() || breadcrumb.is_some() || description.is_some() {
                    PageHeader {
                        breadcrumb,
                        title: title.unwrap_or_default(),
                        description,
                    }
                }

                // Featured (opcional)
                if let Some((idx, evento)) = featured {
                    Card { class: "mb-8 overflow-hidden",
                        div { class: "grid md:grid-cols-2",
                            div { class: "p-8 flex flex-col justify-center",
                                Badge {
                                    variant: match evento.get_badge_variant() {
                                        "EN VIVO" => BadgeVariant::Default,
                                        "FINALIZADO" => BadgeVariant::Outline,
                                        _ => BadgeVariant::Secondary,
                                    },
                                    class: "mb-4",
                                    "{evento.get_badge_variant()}"
                                }
                                h2 { class: "text-3xl font-bold text-zinc-950 dark:text-zinc-50 mb-4",
                                    "{evento.evento_nombre}"
                                }
                                p { class: "text-zinc-600 dark:text-zinc-400 mb-6",
                                    "{evento.evento_descripcion}"
                                }
                                Link { to: Route::Event { id: idx as i32 },
                                    Button { variant: crate::components::ui::ButtonVariant::Default,
                                        "Ver Detalles"
                                    }
                                }
                            }
                            div { class: "bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 min-h-[300px] flex items-center justify-center",
                                span { class: "text-9xl", "{evento.get_deporte_emoji()}" }
                            }
                        }
                    }
                }

                Separator { class: "my-8" }

                // Filtros
                if !filters.is_empty() {
                    FilterSection { filters }
                }

                // Contenido principal o estado vacío
                if empty {
                    if show_empty_state {
                        EmptyState {
                            emoji: "🏆".to_string(),
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
                            for (idx , evento) in items_sig() {
                                EventCard {
                                    evento,
                                    index: idx as i32,
                                    layout: layout_choice,
                                }
                            }
                        }
                    }

                    // Paginación
                    Pagination { current_page: current_page_signal, total_pages }
                }
            }
        }
    }
}
