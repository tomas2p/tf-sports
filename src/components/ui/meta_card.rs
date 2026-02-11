use dioxus::prelude::*;

#[component]
pub fn MetaCard(
    title: String,
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        crate::components::ui::Card {
            crate::components::ui::CardHeader {
                crate::components::ui::CardTitle { "{title}" }
            }
            crate::components::ui::CardContent {
                div { class: "space-y-3 {class}", {children} }
            }
        }
    }
}
