use dioxus::prelude::*;
use crate::Route;

/// Componente base de tarjeta genérico que soporta imagen opcional, routing y hover effects
#[component]
pub fn BaseCard(
    children: Element,
    #[props(default = None)] header_visual: Option<Element>,
    #[props(default = None)] to_route: Option<Route>,
    #[props(default = "hover:shadow-md".to_string())] hover_class: String,
    #[props(default = false)] overflow_hidden: bool,
    #[props(default = "".to_string())] class: String,
) -> Element {
    let overflow_class = if overflow_hidden { "overflow-hidden" } else { "" };
    let card_classes = format!(
        "rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm {} transition-shadow cursor-pointer h-full {} {}",
        hover_class, overflow_class, class
    );
    
    let card_content = rsx! {
        div { class: "{card_classes}",
            if let Some(visual) = header_visual {
                {visual}
            }
            {children}
        }
    };
    
    if let Some(route) = to_route {
        rsx! {
            Link { to: route, class: "no-underline", {card_content} }
        }
    } else {
        card_content
    }
}

#[component]
pub fn Card(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "rounded-lg border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm {class}",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col space-y-1.5 p-6 {class}", {children} }
    }
}

#[component]
pub fn CardTitle(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        h3 { class: "text-2xl font-semibold leading-none tracking-tight text-zinc-950 dark:text-zinc-50 {class}",
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "p-6 pt-0 {class}", {children} }
    }
}
