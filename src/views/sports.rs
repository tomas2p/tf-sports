use crate::Route;
use crate::components::ui::*;
use crate::models::{DEPORTES, EventoData};
use dioxus::prelude::*;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

/// The Sports page component showing all available sports categories
#[component]
pub fn Sports() -> Element {
    // Cargar eventos para contar por deporte
    let eventos_data = use_memo(move || {
        match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
            Ok(data) => data,
            Err(_) => EventoData { eventos: vec![] }
        }
    });
    
    // Contar eventos por deporte
    let eventos_por_deporte = use_memo(move || {
        let data = eventos_data();
        let mut conteo: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        
        for evento in data.eventos.iter() {
            let deporte = evento.get_deporte();
            *conteo.entry(deporte).or_insert(0) += 1;
        }
        
        conteo
    });
    
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

            div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-6",
              for deporte in DEPORTES {
                Link {
                  to: Route::Sport {
                      category: deporte.nombre.to_string(),
                  },
                  class: "no-underline",
                  Card { class: "hover:shadow-lg transition-all cursor-pointer h-full",
                    div { class: "h-32 flex items-center justify-center text-6xl bg-gradient-to-br from-zinc-100 to-zinc-200 dark:from-zinc-800 dark:to-zinc-900",
                      "{deporte.emoji}"
                    }
                    CardHeader {
                      CardTitle { class: "text-center text-xl", "{deporte.nombre}" }
                    }
                    CardContent { class: "text-center",
                      Badge {
                        variant: BadgeVariant::Secondary,
                        class: "text-xs",
                        {
                            let conteo = eventos_por_deporte();
                            let cantidad = conteo.get(deporte.nombre).copied().unwrap_or(0);
                            if cantidad == 1 {
                                format!("{} evento", cantidad)
                            } else {
                                format!("{} eventos", cantidad)
                            }
                        }
                      }
                      p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-3 line-clamp-2",
                        "Descubre eventos de {deporte.nombre.to_lowercase()} en Tenerife"
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
}
