use crate::components::ui::Container;
use chrono::Datelike;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let year = chrono::Local::now().year();
    rsx! {
        footer { class: "bg-zinc-50 dark:bg-zinc-900 border-t border-zinc-200 dark:border-zinc-800",
            Container {
                div { class: "py-6 m-2 text-sm text-zinc-600 dark:text-zinc-400 flex flex-col md:flex-row items-center justify-between gap-3",
                    div {
                        a { href: "https://tomas2p.github.io/tf-sports/", "TF-Sports" }
                        " © {year} by "
                        a { href: "https://github.com/tomas2p", "Tomás Pino Pérez" }
                        " is licensed under "
                        a { href: "https://creativecommons.org/licenses/by-nc/4.0/", "CC BY-NC 4.0" }
                        img { src: "https://mirrors.creativecommons.org/presskit/icons/cc.svg", alt: "", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                        img { src: "https://mirrors.creativecommons.org/presskit/icons/by.svg", alt: "", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                        img { src: "https://mirrors.creativecommons.org/presskit/icons/nc.svg", alt: "", style: "max-width: 1em;max-height:1em;margin-left: .2em;" }
                    }
                    nav { class: "flex gap-4",
                    a { href: "https://tomas2p.vercel.app", class: "hover:underline", "By @Tomas2p" }
                        a { href: "https://github.com/tomas2p/tf-sports", class: "hover:underline", "GitHub (Source Code)" }
                        a { href: "https://github.com/tomas2p/tf-sports/releases", class: "hover:underline", "Android (APK)" }
                        a { href: "https://github.com/tomas2p/tf-sports/LICENSE", class: "hover:underline", "EUPL_v1.2" }
                    }
                }
            }
        }
    }
}
