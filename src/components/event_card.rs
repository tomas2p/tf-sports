use crate::Route;
use crate::components::ui::*;
use crate::models::Evento;
use dioxus::prelude::*;

/// Define las variantes de layout para EventCard
#[derive(Clone, Copy, PartialEq)]
pub enum LayoutVariant {
    /// Layout detallado con imagen del deporte y nombre del deporte visible
    Detailed,
    /// Layout simple sin imagen, más compacto
    Simple,
}

/// Componente unificado para mostrar tarjetas de evento con diferentes layouts
#[component]
pub fn EventCard(
    evento: Evento,
    index: i32,
    #[props(default = LayoutVariant::Simple)] layout: LayoutVariant,
) -> Element {
    let header_visual = match layout {
        LayoutVariant::Detailed => Some(rsx! {
            div { class: "h-32 bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 flex items-center justify-center",
                span { class: "text-6xl", "{evento.get_deporte_emoji()}" }
            }
        }),
        LayoutVariant::Simple => None,
    };
    
    let overflow_hidden = matches!(layout, LayoutVariant::Detailed);
    
    rsx! {
        BaseCard {
            to_route: Route::Event { id: index },
            header_visual,
            overflow_hidden,
            CardHeader {
                Badge {
                    variant: match evento.get_badge_variant() {
                        "EN VIVO" => BadgeVariant::Default,
                        "FINALIZADO" => BadgeVariant::Outline,
                        _ => BadgeVariant::Secondary,
                    },
                    "{evento.get_badge_variant()}"
                }
                CardTitle {
                    class: match layout {
                        LayoutVariant::Detailed => "mt-2 text-lg leading-tight",
                        LayoutVariant::Simple => "mt-2",
                    },
                    "{evento.evento_nombre}"
                }
            }
            CardContent {
                class: match layout {
                    LayoutVariant::Detailed => "space-y-3",
                    LayoutVariant::Simple => "",
                },

                // Mostrar nombre del deporte solo en layout Detailed
                if layout == LayoutVariant::Detailed {
                    p { class: "text-sm font-medium text-zinc-600 dark:text-zinc-400",
                        "{evento.get_deporte()}"
                    }
                }

                // Fecha
                match layout {
                    LayoutVariant::Detailed => rsx! {
                        div { class: "flex items-start gap-2",
                            span { class: "text-sm text-zinc-500 dark:text-zinc-500", "📅" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "{evento.format_fecha()}" }
                        }
                    },
                    LayoutVariant::Simple => rsx! {
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2", "📅 {evento.format_fecha()}" }
                    },
                }

                // Lugar y municipio
                match layout {
                    LayoutVariant::Detailed => rsx! {
                        div { class: "flex items-start gap-2",
                            span { class: "text-sm text-zinc-500 dark:text-zinc-500", "📍" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                                "{evento.evento_lugar.as_ref().unwrap_or(&\"Lugar por determinar\".to_string()).clone()}"
                                if let Some(ref municipio) = evento.municipio_nombre {
                                    ", {municipio}"
                                }
                            }
                        }
                    },
                    LayoutVariant::Simple => rsx! {
                        if let Some(lugar) = &evento.evento_lugar {
                            p { class: "text-sm text-zinc-500 dark:text-zinc-500 mb-1", "📍 {lugar}" }
                        }
                        if let Some(municipio) = &evento.municipio_nombre {
                            p { class: "text-xs text-zinc-500 dark:text-zinc-500", "{municipio}" }
                        }
                    },
                }

                // Organizador (usando emoji 👥)
                match layout {
                    LayoutVariant::Detailed => rsx! {
                        div { class: "flex items-start gap-2",
                            span { class: "text-sm text-zinc-500 dark:text-zinc-500", "👥" }
                            p { class: "text-sm text-zinc-600 dark:text-zinc-400 truncate", "{evento.evento_organizador}" }
                        }
                    },
                    LayoutVariant::Simple => rsx! {
                        p { class: "text-xs text-zinc-400 dark:text-zinc-600 mt-2", "👥 {evento.evento_organizador}" }
                    },
                }
            }
        }
    }
}
