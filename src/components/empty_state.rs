use crate::components::ui::*;
use dioxus::prelude::*;

/// Componente reutilizable para estados vacíos
#[component]
pub fn EmptyState(
    emoji: String,
    title: String,
    message: String,
    #[props(default = None)] action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "text-center py-12",
            span { class: "text-6xl mb-4 block", "{emoji}" }
            h2 { class: "text-2xl font-bold text-zinc-950 dark:text-zinc-50 mb-2",
                "{title}"
            }
            p { class: "text-zinc-600 dark:text-zinc-400 mt-2", "{message}" }
            if let Some(action_content) = action {
                div { class: "mt-6", {action_content} }
            }
        }
    }
}
