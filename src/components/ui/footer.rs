use crate::components::ui::Container;
use chrono::Datelike;
use dioxus::prelude::*;

const CC_SVG: Asset = asset!("/assets/cc.svg");
const BY_SVG: Asset = asset!("/assets/by.svg");
const NC_SVG: Asset = asset!("/assets/nc.svg");

#[component]
pub fn Footer() -> Element {
    let year = chrono::Local::now().year();
    rsx! {
        footer { class: "bg-zinc-50 dark:bg-zinc-900 border-t border-zinc-200 dark:border-zinc-800",
            Container {
                div { class: "py-6 m-4 text-sm text-zinc-600 dark:text-zinc-400 flex flex-col md:flex-row items-center justify-between gap-3",
                    div { class: "flex flex-row flex-wrap gap-2 items-center justify-center",
                        a { href: "https://tomas2p.github.io/tf-sports/", "TF-Sports" }
                        " © {year} by "
                        a { href: "https://tomas2p.vercel.app", "Tomás Pino Pérez" }
                        " is licensed under "
                        a { href: "https://creativecommons.org/licenses/by-nc/4.0/", "CC BY-NC 4.0" }
                        img { src: CC_SVG, alt: "CC", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                        img { src: BY_SVG, alt: "BY", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                        img { src: NC_SVG, alt: "NC", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                    }
                    nav { class: "grid grid-cols-2 md:grid-cols-4 gap-2 items-center justify-center",
                        for &(href, text) in [
                            ("https://tomas2p.vercel.app", "By @Tomas2p"),
                            ("https://github.com/tomas2p/tf-sports", "GitHub (Source Code)"),
                            ("https://github.com/tomas2p/tf-sports/releases", "Android (APK)"),
                            ("https://github.com/tomas2p/tf-sports/LICENSE", "EUPL_v1.2"),
                        ].iter() {
                            a { href: href.to_string(), class: "hover:underline", {text.to_string()} }
                        }
                    }
                }
            }
        }
    }
}
