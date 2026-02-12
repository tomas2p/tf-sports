use crate::components::ui::*;
use crate::components::{breadcrumb_items, Breadcrumb, CategoryCard};
use crate::data::{get_espacios, get_instalaciones};
use crate::models::get_deporte_info;
use crate::models::EspacioDeportivo;
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Place(id: i64) -> Element {
    // Usar datos cacheados

    let instalacion_feature = use_memo(move || {
        get_instalaciones()
            .features
            .iter()
            .find(|f| f.properties.instalacion_codigo == id)
            .cloned()
    });

    let espacios = use_memo(move || {
        get_espacios()
            .iter()
            .filter(|e| e.instalacion_codigo == id)
            .cloned()
            .collect::<Vec<_>>()
    });

    match instalacion_feature() {
        Some(feature) => rsx! {
            BaseLayout {
                title: feature.properties.instalacion_nombre.clone(),
                breadcrumb: rsx! { Breadcrumb { items: breadcrumb_items!( ("Inicio", Route::Home {}), ("Instalaciones", Route::Places {}), (feature.properties.instalacion_nombre.clone()) ) } },
                subtitle: Some(format!("📍 {}", feature.properties.municipio_nombre.clone())),
                hero: Some("🏟️".to_string()),
                meta: rsx! {
                    div { class: "space-y-6",
                        MetaCard { title: "Información de Contacto".to_string(),
                            div { class: "space-y-3",
                                if let Some(telefono) = &feature.properties.telefono_fijo {
                                    div {
                                        p { class: "text-zinc-600 dark:text-zinc-400 text-sm", "Teléfono" }
                                        a {
                                            href: "tel:{telefono}",
                                            class: "text-zinc-950 dark:text-zinc-50 text-sm font-medium hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors",
                                            "{telefono}"
                                        }
                                    }
                                }
                                if let Some(email) = &feature.properties.email {
                                    div {
                                        p { class: "text-zinc-600 dark:text-zinc-400 text-sm", "Email" }
                                        a {
                                            href: "mailto:{email}",
                                            class: "text-zinc-950 dark:text-zinc-50 text-sm font-medium hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors break-all",
                                            "{email}"
                                        }
                                    }
                                }
                                if let Some(web) = &feature.properties.web {
                                    div {
                                        p { class: "text-zinc-600 dark:text-zinc-400 text-sm", "Sitio Web" }
                                        a {
                                            href: "https://{web}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: "text-zinc-950 dark:text-zinc-50 text-sm font-medium hover:text-zinc-600 dark:hover:text-zinc-400 transition-colors break-all",
                                            "{web}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },

                // Main content: espacios
                div {
                    // Separator { class: "vertical my-8" }

                    {
                        let espacios_list = espacios();
                        if espacios_list.is_empty() {
                            rsx! {
                                Card {
                                    CardHeader {
                                        CardTitle { "Espacios Deportivos" }
                                    }
                                    CardContent {
                                        div { class: "text-center py-8",
                                            p { class: "text-4xl mb-4", "🏟️" }
                                            p { class: "text-zinc-600 dark:text-zinc-400",
                                                "Esta instalación no tiene espacios deportivos registrados."
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let mut espacios_por_actividad: HashMap<String, Vec<EspacioDeportivo>> = HashMap::new();
                            for espacio in espacios_list.iter() {
                                let actividad = espacio
                                    .espacio_actividad_principal
                                    .as_ref()
                                    .map(|a: &String| a.clone())
                                    .unwrap_or_else(|| "Otros".to_string());
                                espacios_por_actividad
                                    .entry(actividad)
                                    .or_insert_with(Vec::new)
                                    .push(espacio.clone());
                            }
                            let mut actividades: Vec<String> = espacios_por_actividad
                                .keys()
                                .cloned()
                                .collect();
                            actividades.sort();
                            rsx! {
                                Card { 
                                    class: "border-none dark:border-none bg-none dark:bg-none",
                                    CardHeader { 
                                        class:"p-0 mb-4",
                                        div { class: "flex items-center justify-between",
                                            CardTitle { "Espacios Deportivos" }
                                            Badge { variant: BadgeVariant::Outline, "{espacios_list.len()} espacios" }
                                        }
                                    }
                                    CardContent {
                                        class:"p-0",
                                        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4",
                                            for actividad in actividades {
                                                {
                                                    let espacios_actividad = espacios_por_actividad.get(&actividad).unwrap();
                                                    let mut tipos: Vec<String> = espacios_actividad
                                                        .iter()
                                                        .filter_map(|e| e.espacio_tipo.clone())
                                                        .collect::<std::collections::HashSet<_>>()
                                                        .into_iter()
                                                        .collect();
                                                    tipos.sort();
                                                    let mut pavimentos: Vec<String> = espacios_actividad
                                                        .iter()
                                                        .filter_map(|e| e.pavimento_tipo.clone())
                                                        .collect::<std::collections::HashSet<_>>()
                                                        .into_iter()
                                                        .collect();
                                                    pavimentos.sort();
                                                    let tiene_cubierto = espacios_actividad
                                                        .iter()
                                                        .any(|e| {
                                                            e.espacio_cerramiento
                                                                .as_ref()
                                                                .map(|c| {
                                                                    c.to_lowercase().contains("cubierto")
                                                                        || c.to_lowercase().contains("cerrado")
                                                                })
                                                                .unwrap_or(false)
                                                        });
                                                    let tiene_abierto = espacios_actividad
                                                        .iter()
                                                        .any(|e| {
                                                            e.espacio_cerramiento
                                                                .as_ref()
                                                                .map(|c| c.to_lowercase().contains("abierto"))
                                                                .unwrap_or(false)
                                                        });
                                                    let tiene_nocturno = espacios_actividad
                                                        .iter()
                                                        .any(|e| {
                                                            e.espacio_iluminacion
                                                                .as_ref()
                                                                .map(|i| i.to_lowercase().contains("nocturno"))
                                                                .unwrap_or(false)
                                                        });
                                                    let badge_text = format!(
                                                        "{} espacio{}",
                                                        espacios_actividad.len(),
                                                        if espacios_actividad.len() == 1 { "" } else { "s" },
                                                    );

                                                    rsx! {
                                                        CategoryCard {
                                                            emoji: Some(get_deporte_info(&actividad).map_or("🏆", |d| d.emoji).to_string()),
                                                            title: actividad.clone(),
                                                            badge_text: badge_text.clone(),
                                                            description: Some(tipos.join(", ")),
                                                            footer: Some(rsx!{
                                                                div { class: "flex flex-wrap gap-2 justify-center",
                                                                    for pav in pavimentos.iter() {
                                                                        Badge { variant: BadgeVariant::Default, "{pav}" }
                                                                    }
                                                                    if tiene_cubierto {
                                                                        Badge { variant: BadgeVariant::Secondary, "🏠 Cubierto" }
                                                                    }
                                                                    if tiene_abierto {
                                                                        Badge { variant: BadgeVariant::Outline, "☀️ Abierto" }
                                                                    }
                                                                    if tiene_nocturno {
                                                                        Badge { variant: BadgeVariant::Secondary, "🌙 Nocturno" }
                                                                    }
                                                                }
                                                            })
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
            }
        },
        None => rsx! {
            BaseLayout {
                title: "Instalación no encontrada".to_string(),
                children: rsx!{
                    div { class: "text-center py-12",
                        p { class: "text-6xl mb-4", "🔍" }
                        h2 { class: "text-2xl font-bold text-zinc-950 dark:text-zinc-50 mb-2",
                            "Instalación no encontrada"
                        }
                        p { class: "text-zinc-600 dark:text-zinc-400", "No se pudo encontrar la instalación solicitada." }
                    }
                }
            }
        },
    }
}

// EspacioDeportivoData wrapper removed: use `get_espacios()` from `crate::data` instead
