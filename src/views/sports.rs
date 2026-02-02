use crate::Route;
use crate::components::ui::*;
use crate::models::DEPORTES;
use dioxus::prelude::*;

/// The Sports page component showing all available sports categories
#[component]
pub fn Sports() -> Element {
    rsx! {
        Container {
            Section {
                div { class: "space-y-8",
                    div { class: "text-center space-y-4",
                        h1 { class: "text-4xl md:text-5xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
                            "Deportes en Tenerife"
                        }
                        p { class: "text-lg text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto",
                            "Explora todos los deportes disponibles en la isla"
                        }
                    }

                    div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4",
                        for deporte in DEPORTES {
                            Link { to: Route::Events {}, class: "no-underline",
                                Card { class: "hover:shadow-lg transition-all cursor-pointer h-full",
                                    div { class: "h-32 flex items-center justify-center text-6xl bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900",
                                        "{deporte.emoji}"
                                    }
                                    CardHeader {
                                        CardTitle { class: "text-center", "{deporte.nombre}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
