use dioxus::prelude::*;

#[component]
pub fn Separator(
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "h-px w-full bg-zinc-200 dark:bg-zinc-800 {class}" }
    }
}
