use crate::components::ui::Container;
use chrono::Datelike;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let year = chrono::Local::now().year();
    rsx! {
        footer { class: "bg-zinc-50 dark:bg-zinc-900 border-t border-zinc-200 dark:border-zinc-800",
            Container {
                div { class: "py-6 text-sm text-zinc-600 dark:text-zinc-400 flex flex-col md:flex-row items-center justify-between gap-3",
                    div { "© {year} TF Sports. Todos los derechos reservados." }
                    nav { class: "flex gap-4",
                    a { href: "https://tomas2p.vercel.app", class: "hover:underline", "By @Tomas2p" }
                        a { href: "https://github.com/tomas2p/tf-sports", class: "hover:underline", "GitHub (Source Code)" }
                        a { href: "https://github.com/tomas2p/tf-sports/releases", class: "hover:underline", "Android (APK)" }
                        // a { href: "/privacy", class: "hover:underline", "Política de privacidad" }
                    }
                }
            }
        }
    }
}
