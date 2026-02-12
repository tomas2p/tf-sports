use dioxus::prelude::*;

#[component]
pub fn MetaTags(
    title: String,
    #[props(default = None)] description: Option<String>,
    #[props(default = None)] url: Option<String>,
    #[props(default = None)] image: Option<String>,
) -> Element {
    // Ensure we always have an image URL: use provided image or fallback asset
    let image_url = match image.clone() {
        Some(i) => i,
        None => asset!("/assets/og_fallback.svg").to_string()
    };
    let twitter_card = "summary_large_image";

    rsx! {
        document::Title { "{title}" }

        if let Some(desc) = description.clone() {
            document::Meta { name: "description", content: desc.clone() }
        }

        if let Some(u) = url.clone() {
            document::Link { rel: "canonical", href: u.clone() }
        }

        document::Meta { property: "og:site_name", content: "TF Sports" }
        document::Meta { property: "og:title", content: title.clone() }
        if let Some(desc) = description.clone() {
            document::Meta { property: "og:description", content: desc.clone() }
        }
        if let Some(u) = url.clone() {
            document::Meta { property: "og:url", content: u.clone() }
        }
        document::Meta { property: "og:image", content: image_url.clone() }

        document::Meta { name: "twitter:card", content: twitter_card.to_string() }
        document::Meta { name: "twitter:title", content: title.clone() }
        if let Some(desc) = description.clone() {
            document::Meta { name: "twitter:description", content: desc.clone() }
        }
        document::Meta { name: "twitter:image", content: image_url.clone() }
    }
}
