use dioxus::prelude::*;

#[component]
pub fn BaseLayout(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    breadcrumb: Option<Element>,
    #[props(default = None)] hero: Option<Element>,
    #[props(default = None)] badge: Option<Element>,
    #[props(default = None)] meta: Option<Element>,
    #[props(default = None)] actions: Option<Element>,
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        crate::components::ui::Container {
            crate::components::ui::Section {
                // Breadcrumb (optional) + Header: badge + title + actions
                if let Some(bc) = breadcrumb {
                    div { class: "mb-2", {bc} }
                }

                div { class: "space-y-4 mt-2",
                    div { class: "flex items-start justify-between gap-4",
                        div { class: "flex-1",
                            if let Some(b) = badge { div { class: "mb-2", {b} } }
                            h1 { class: "text-4xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50", "{title}" }
                            if let Some(sub) = subtitle {
                                p { class: "text-lg text-zinc-600 dark:text-zinc-400", "{sub}" }
                            }
                        }
                        if let Some(act) = actions { div { {act} } }
                    }

                    if let Some(h) = hero {
                        div { class: "mt-4 rounded-lg overflow-hidden", {h} }
                    }
                }

                // Main grid: meta (sidebar) + content
                div { class: "grid gap-6 md:grid-cols-3 mt-8 {class}",
                    if let Some(m) = meta {
                        div { class: "md:col-span-1", {m} }
                    }

                    div { class: "md:col-span-2", {children} }
                }
            }
        }
    }
}
