use crate::components::ui::*;
use crate::components::MetaTags;
use crate::components::PageHeader;
use dioxus::prelude::*;

#[component]
pub fn BaseLayout(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    breadcrumb: Option<Element>,
    #[props(default = None)] hero: Option<String>,
    #[props(default = None)] badge: Option<Element>,
    #[props(default = None)] meta: Option<Element>,
    #[props(default = None)] actions: Option<Element>,
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
            // Meta tags for SEO: title/description/og/twitter. Pages can pass subtitle as description.
            MetaTags { title: title.clone(), description: subtitle.clone(), url: None, image: None }

            crate::components::ui::Container {
                crate::components::ui::Section {
                    // Encabezado de página: reutilizar `PageHeader` para mantener consistencia
                    // Usar `md:items-stretch` para que el hero tenga la misma altura que el PageHeader
                    div { class: "flex flex-col md:flex-row gap-4 justify-between items-start md:items-stretch",
                        PageHeader {
                            title: title.clone(),
                            description: subtitle.clone(),
                            breadcrumb,
                            badge,
                            actions,
                        }

                        if let Some(h) = hero {
                            // Mobile: full width (w-full) and fixed height; md+: match PageHeader height and keep aspect square
                            div { class: "relative w-full h-48 md:h-auto md:w-auto md:p-5 md:aspect-square bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 rounded-lg flex items-center justify-center overflow-hidden",
                            span { class: "text-6xl md:aspect-square", "{h}" }
                        }
                    }
                }

                Separator { class: "my-4" }

                    // Main grid: meta (sidebar) + content
    div { class: "flex flex-col md:flex-row gap-6 {class}",
        if let Some(m) = meta {
            div { class: "md:w-1/3", {m} } }
            div { class: "md:w-2/3", {children} } } }
            }
        }
}
