use crate::components::ui::*;
use dioxus::prelude::*;

/// Componente reutilizable para paginación con manejo inteligente de muchas páginas
#[component]
pub fn Pagination(
    current_page: Signal<usize>,
    total_pages: usize,
) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }
    
    // Calcular qué páginas mostrar
    let current = current_page();
    let max_visible = 7; // Número máximo de botones de página visibles
    
    let pages_to_show: Vec<usize> = if total_pages <= max_visible {
        // Si hay pocas páginas, mostrar todas
        (1..=total_pages).collect()
    } else {
        // Lógica para mostrar rango limitado con elipsis
        let mut pages = vec![1]; // Siempre mostrar primera página
        
        let start = if current <= 3 {
            2
        } else if current >= total_pages - 2 {
            total_pages - 4
        } else {
            current - 1
        };
        
        let end = if current <= 3 {
            5
        } else if current >= total_pages - 2 {
            total_pages - 1
        } else {
            current + 1
        };
        
        // Agregar elipsis inicial si es necesario
        if start > 2 {
            pages.push(0); // 0 representa elipsis
        }
        
        // Agregar páginas del rango
        for p in start..=end.min(total_pages - 1) {
            pages.push(p);
        }
        
        // Agregar elipsis final si es necesario
        if end < total_pages - 1 {
            pages.push(0); // 0 representa elipsis
        }
        
        // Siempre mostrar última página
        pages.push(total_pages);
        pages
    };
    
    rsx! {
        div { class: "flex justify-center items-center gap-2 mt-8 flex-wrap",
            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let current = current_page();
                    if current > 1 {
                        current_page.set(current - 1);
                    }
                },
                disabled: current == 1,
                "← Anterior"
            }

            div { class: "flex gap-1 flex-wrap",
                for p in pages_to_show {
                    if p == 0 {
                        // Mostrar elipsis
                        span {
                            key: "ellipsis-{p}",
                            class: "px-2 py-1 text-zinc-500 dark:text-zinc-400",
                            "..."
                        }
                    } else {
                        Button {
                            key: "{p}",
                            variant: if current == p { ButtonVariant::Default } else { ButtonVariant::Ghost },
                            size: ButtonSize::Sm,
                            onclick: move |_| current_page.set(p),
                            "{p}"
                        }
                    }
                }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let current = current_page();
                    if current < total_pages {
                        current_page.set(current + 1);
                    }
                },
                disabled: current == total_pages,
                "Siguiente →"
            }
        }
    }
}
