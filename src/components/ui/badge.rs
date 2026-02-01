use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Outline,
}

#[component]
pub fn Badge(
    children: Element,
    #[props(default = BadgeVariant::Default)] variant: BadgeVariant,
    #[props(default = "".to_string())] class: String,
) -> Element {
    let base_classes = "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold transition-colors";
    
    let variant_classes = match variant {
        BadgeVariant::Default => "bg-zinc-900 dark:bg-zinc-50 text-white dark:text-zinc-900",
        BadgeVariant::Secondary => "bg-zinc-100 dark:bg-zinc-800 text-zinc-900 dark:text-zinc-50",
        BadgeVariant::Outline => "border border-zinc-300 dark:border-zinc-700 text-zinc-900 dark:text-zinc-50",
    };
    
    rsx! {
        div { class: "{base_classes} {variant_classes} {class}", {children} }
    }
}
