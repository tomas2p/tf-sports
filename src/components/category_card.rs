use crate::components::ui::*;
use dioxus::prelude::*;

/// Props for the CategoryCard component
#[derive(Props, Clone, PartialEq)]
pub struct CategoryCardProps {
    /// The emoji to display at the top of the card (optional)
    #[props(default = None)]
    pub emoji: Option<String>,
    /// The title/name of the category
    pub title: String,
    /// The badge text (e.g., "5 eventos", "3 espacios")
    pub badge_text: String,
    /// Optional description text to show at the bottom
    #[props(default = None)]
    pub description: Option<String>,
    /// Optional footer element for custom badges or controls
    #[props(default = None)]
    pub footer: Option<Element>,
}

/// A reusable card component for displaying categories with emoji, title, badge and description
#[component]
pub fn CategoryCard(props: CategoryCardProps) -> Element {
    let header_visual = props.emoji.as_ref().map(|emoji| rsx! {
        div { class: "h-32 flex items-center justify-center text-6xl bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900",
            "{emoji}"
        }
    });

    rsx! {
        BaseCard { hover_class: "hover:shadow-lg".to_string(), header_visual,
            CardHeader {
                CardTitle { class: "text-center text-xl", "{props.title}" }
            }
            CardContent { class: "text-center",
                Badge { variant: BadgeVariant::Secondary, class: "text-xs", "{props.badge_text}" }
                if let Some(desc) = &props.description {
                    p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-3 line-clamp-2",
                        "{desc}"
                    }
                }

                if let Some(footer) = &props.footer {
                    div { class: "mt-3", {footer.clone()} }
                }
            }
        }
    }
}
