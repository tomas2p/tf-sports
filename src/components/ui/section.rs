use dioxus::prelude::*;

#[component]
pub fn Section(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        section { class: "overflow-hidden py-8 md:py-10 lg:py-12 {class}", {children} }
    }
}
