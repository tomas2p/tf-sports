use dioxus::prelude::*;

#[component]
pub fn Section(
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        section { class: "py-12 md:py-16 lg:py-20 {class}", {children} }
    }
}
