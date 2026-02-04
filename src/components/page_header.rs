use dioxus::prelude::*;

/// Componente reutilizable para encabezados de página
#[component]
pub fn PageHeader(
    title: String,
    description: Option<String>,
    #[props(default = None)] breadcrumb: Option<Element>,
    #[props(default = None)] badge: Option<Element>,
    #[props(default = None)] actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "space-y-4 mb-8",
            if let Some(bc) = breadcrumb {
                div { class: "mb-2", {bc} }
            }
            if let Some(badge_content) = badge {
                div { class: "mb-2", {badge_content} }
            }
            h1 { class: "text-4xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
                "{title}"
            }
            if let Some(desc) = description {
                p { class: "text-lg text-zinc-600 dark:text-zinc-400", "{desc}" }
            }
            if let Some(action_content) = actions {
                div { class: "mt-4", {action_content} }
            }
        }
    }
}
