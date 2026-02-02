use crate::Route;
use crate::components::ui::*;
use crate::models::Evento;
use dioxus::prelude::*;

/// Componente reutilizable para mostrar una tarjeta de evento
#[component]
pub fn EventCard(evento: Evento, index: i32) -> Element {
    rsx! {
      Link { to: Route::Event { id: index }, class: "no-underline",
        Card { class: "hover:shadow-md transition-shadow cursor-pointer h-full",
          CardHeader {
            Badge {
              variant: match evento.get_badge_variant() {
                  "EN VIVO" => BadgeVariant::Default,
                  "FINALIZADO" => BadgeVariant::Outline,
                  _ => BadgeVariant::Secondary,
              },
              "{evento.get_badge_variant()}"
            }
            CardTitle { class: "mt-2", "{evento.evento_nombre}" }
          }
          CardContent {
            p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
              "📅 {evento.format_fecha()}"
            }
            if let Some(lugar) = &evento.evento_lugar {
              p { class: "text-sm text-zinc-500 dark:text-zinc-500 mb-1",
                "📍 {lugar}"
              }
            }
            if let Some(municipio) = &evento.municipio_nombre {
              p { class: "text-xs text-zinc-500 dark:text-zinc-500",
                "{municipio}"
              }
            }
            p { class: "text-xs text-zinc-400 dark:text-zinc-600 mt-2",
              "🏢 {evento.evento_organizador}"
            }
          }
        }
      }
    }
}
