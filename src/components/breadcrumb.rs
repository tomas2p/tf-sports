use crate::Route;
use dioxus::prelude::*;

/// Item de breadcrumb
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub route: Option<Route>,
}

/// Componente reutilizable para navegación tipo breadcrumb
#[component]
pub fn Breadcrumb(items: Vec<BreadcrumbItem>) -> Element {
    rsx! {
        div { class: "flex items-center gap-2 text-sm mb-8 text-zinc-600 dark:text-zinc-400",
            for (index , item) in items.iter().enumerate() {
                if index > 0 {
                    span { "/" }
                }
                if let Some(route) = &item.route {
                    Link {
                        to: route.clone(),
                        class: "hover:text-zinc-900 dark:hover:text-zinc-100",
                        "{item.label}"
                    }
                } else {
                    span { class: "text-zinc-900 dark:text-zinc-50 font-medium", "{item.label}" }
                }
            }
        }
    }
}
