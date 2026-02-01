use dioxus::prelude::*;

#[component]
pub fn Container(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "container mx-auto px-4 md:px-6 lg:px-8 {class}", {children} }
    }
}
