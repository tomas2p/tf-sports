use crate::Route;
use crate::components::ui::*;
use crate::models::Evento;
use dioxus::prelude::*;

/// Variante de EventCard con imagen grande de deporte (para vista Events)
#[component]
pub fn EventCardWithImage(evento: Evento, index: i32) -> Element {
    rsx! {
      Link { to: Route::Event { id: index }, class: "no-underline",
        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full overflow-hidden",
          // Imagen del deporte
          div { class: "h-32 bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900 flex items-center justify-center",
            span { class: "text-6xl", "{evento.get_deporte_emoji()}" }
          }
          CardHeader {
            Badge {
              variant: match evento.get_badge_variant() {
                  "EN VIVO" => BadgeVariant::Default,
                  "FINALIZADO" => BadgeVariant::Outline,
                  _ => BadgeVariant::Secondary,
              },
              "{evento.get_badge_variant()}"
            }
            CardTitle { class: "mt-2 text-lg leading-tight", "{evento.evento_nombre}" }
          }
          CardContent {
            div { class: "space-y-3",
              // Deporte
              p { class: "text-sm font-medium text-zinc-600 dark:text-zinc-400",
                "{evento.get_deporte()}"
              }

              // Fecha
              div { class: "flex items-start gap-2",
                span { class: "text-sm text-zinc-500 dark:text-zinc-500",
                  "📅"
                }
                p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                  "{evento.format_fecha()}"
                }
              }

              // Lugar y municipio
              div { class: "flex items-start gap-2",
                span { class: "text-sm text-zinc-500 dark:text-zinc-500",
                  "📍"
                }
                p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                  "{evento.evento_lugar.as_ref().unwrap_or(&\"Lugar por determinar\".to_string()).clone()}"
                  if let Some(ref municipio) = evento.municipio_nombre {
                    ", {municipio}"
                  }
                }
              }

              // Organizador
              div { class: "flex items-start gap-2",
                span { class: "text-sm text-zinc-500 dark:text-zinc-500",
                  "👥"
                }
                p { class: "text-sm text-zinc-600 dark:text-zinc-400 truncate",
                  "{evento.evento_organizador}"
                }
              }
            }
          }
        }
      }
    }
}
