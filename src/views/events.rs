use crate::Route;
use crate::components::ui::*;
use crate::models::{EventoData, Evento};
use dioxus::prelude::*;

const EVENTOS_JSON: &str = include_str!("../../data/agenda-de-eventos-deportivos-en-tenerife.json");

#[component]
pub fn Events() -> Element {
  let eventos_data = use_memo(move || {
      match serde_json::from_str::<EventoData>(EVENTOS_JSON) {
          Ok(data) => {
              web_sys::console::log_1(&format!("Eventos cargados: {}", data.eventos.len()).into());
              data
          },
          Err(e) => {
              web_sys::console::log_1(&format!("Error parsing JSON: {:?}", e).into());
              EventoData { eventos: vec![] }
          }
      }
  });
  
  let mut filter_estado = use_signal(|| "Todos".to_string());
  let mut filter_deporte = use_signal(|| "Todos".to_string());
  let mut page = use_signal(|| 1);
  let items_per_page = 12;
  
  // Obtener lista de deportes únicos
  let deportes_disponibles = use_memo(move || {
      let data = eventos_data();
      let mut deportes: Vec<String> = data.eventos.iter()
          .map(|e| e.get_deporte())
          .collect();
      deportes.sort();
      deportes.dedup();
      deportes
  });
  
  let eventos_filtrados = use_memo(move || {
      let data = eventos_data();
      let estado_val = filter_estado();
      let deporte_val = filter_deporte();
      
      // Fecha límite: hace una semana
      let now = chrono::Local::now().naive_local();
      let una_semana_atras = now - chrono::Duration::days(7);
      
      let filtered: Vec<(usize, Evento)> = data.eventos.into_iter()
          .enumerate()
          .filter(|(_, e)| {
              // Filtrar eventos antiguos
              let fecha_valida = if let Ok(fecha_fin) = chrono::NaiveDateTime::parse_from_str(&e.evento_fecha_fin, "%Y-%m-%d %H:%M:%S") {
                  fecha_fin >= una_semana_atras
              } else {
                  true // Si no se puede parsear, incluir el evento
              };
              
              let estado_match = if estado_val == "Todos" {
                  true
              } else {
                  e.get_badge_variant() == estado_val
              };
              
              let deporte_match = if deporte_val == "Todos" {
                  true
              } else {
                  e.get_deporte() == deporte_val
              };
              
              fecha_valida && estado_match && deporte_match
          })
          .collect();
      
      web_sys::console::log_1(&format!("Eventos filtrados: {}", filtered.len()).into());
      filtered
  });
  
  let eventos_paginados = use_memo(move || {
      let eventos = eventos_filtrados();
      let current_page = page();
      let start = ((current_page - 1) * items_per_page) as usize;
      let end = (start + items_per_page as usize).min(eventos.len());
      eventos[start..end].to_vec()
  });
  
  let total_pages = use_memo(move || {
      let total = eventos_filtrados().len();
      ((total as f64) / (items_per_page as f64)).ceil() as usize
  });
  
  rsx! {
    Container {
      Section {
        // Header
        div { class: "space-y-4 mb-8",
          h1 { class: "text-4xl font-bold tracking-tight text-zinc-950 dark:text-zinc-50",
            "Eventos Deportivos en Tenerife"
          }
          p { class: "text-lg text-zinc-600 dark:text-zinc-400",
            "Mostrando {eventos_paginados().len()} de {eventos_filtrados().len()} eventos"
          }
        }

        // Filtros
        div { class: "grid gap-4 md:grid-cols-2 mb-8",
          // Filtro de Estado
          div {
            label { class: "block text-sm font-medium text-zinc-950 dark:text-zinc-50 mb-2",
              "Estado:"
            }
            Select {
              value: filter_estado(),
              onchange: move |val: String| {
                  filter_estado.set(val);
                  page.set(1);
              },
              option { value: "Todos", "Todos los estados" }
              option { value: "EN VIVO", "En Vivo" }
              option { value: "PRÓXIMO", "Próximos" }
              option { value: "FINALIZADO", "Finalizados" }
            }
          }

          // Filtro de Deporte
          div {
            label { class: "block text-sm font-medium text-zinc-950 dark:text-zinc-50 mb-2",
              "Deporte:"
            }
            Select {
              value: filter_deporte(),
              onchange: move |val: String| {
                  filter_deporte.set(val);
                  page.set(1);
              },
              option { value: "Todos", "Todos los deportes" }
              for deporte in deportes_disponibles() {
                option { key: "{deporte}", value: "{deporte}", "{deporte}" }
              }
            }
          }
        }

        // Grid de eventos
        div { class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
          if eventos_paginados().is_empty() {
            div { class: "col-span-full text-center py-12",
              p { class: "text-lg text-zinc-600 dark:text-zinc-400",
                "No hay eventos disponibles"
              }
            }
          }
          for (original_idx , evento) in eventos_paginados().iter() {
            Link {
              key: "{original_idx}",
              to: Route::Details {
                  id: *original_idx as i32,
              },
              class: "no-underline",
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
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-2",
                    "{evento.get_deporte()}"
                  }
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-4",
                    "📍 {evento.evento_lugar.as_ref().unwrap_or(&\"Lugar por determinar\".to_string()).clone()}"
                    if let Some(ref municipio) = evento.municipio_nombre {
                      ", {municipio}"
                    }
                  }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-500 mt-2",
                    "{evento.format_fecha()}"
                  }
                }
              }
            }
          }
        }

        // Paginación
        if total_pages() > 1 {
          div { class: "flex justify-center items-center gap-2 mt-8",
            Button {
              variant: ButtonVariant::Outline,
              size: ButtonSize::Sm,
              onclick: move |_| {
                  let current = page();
                  if current > 1 {
                      page.set(current - 1);
                  }
              },
              "← Anterior"
            }

            div { class: "flex gap-1",
              for p in 1..=total_pages() {
                Button {
                  key: "{p}",
                  variant: if page() == p { ButtonVariant::Default } else { ButtonVariant::Ghost },
                  size: ButtonSize::Sm,
                  onclick: move |_| page.set(p),
                  "{p}"
                }
              }
            }

            Button {
              variant: ButtonVariant::Outline,
              size: ButtonSize::Sm,
              onclick: move |_| {
                  let current = page();
                  let total = total_pages();
                  if current < total {
                      page.set(current + 1);
                  }
              },
              "Siguiente →"
            }
          }
        }
      }
    }
  }
}