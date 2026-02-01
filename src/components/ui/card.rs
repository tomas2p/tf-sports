use dioxus::prelude::*;

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
