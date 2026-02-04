use crate::Route;
use dioxus::prelude::*;

/// Item de breadcrumb
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub route: Option<Route>,
}

/// Macro helper para crear vector de BreadcrumbItem de forma concisa.
/// Uso:
/// breadcrumb_items!(
///     ("Inicio", Route::Home {}),
///     ("Deportes", Route::Sports {}),
///     (category.clone())
/// );
#[macro_export]
macro_rules! breadcrumb_items {
    ($( ($label:expr $(, $route:expr)? ) ),* $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push(crate::components::breadcrumb::BreadcrumbItem {
                label: $label.to_string(),
                route: breadcrumb_items!(@opt $($route)?),
            });
        )*
        v
    }};
    (@opt $route:expr) => { Some($route) };
    (@opt) => { None };
}

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
