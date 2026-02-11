use crate::components::ui::*;
use crate::components::{Breadcrumb, breadcrumb_items};
use crate::Route;
use crate::models::EspacioDeportivo;
use crate::data::{get_instalaciones, get_espacios};
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
                    Separator { class: "my-8" }

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
                                    CardHeader {
                                        div { class: "flex items-center justify-between",
                                            CardTitle { "Espacios Deportivos" }
                                            Badge { variant: BadgeVariant::Outline, "{espacios_list.len()} espacios" }
                                        }
                                    }
                                    CardContent {
                                        div { class: "space-y-8",
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
                                                    let mut grupos: HashMap<String, Vec<EspacioDeportivo>> = HashMap::new();
                                                    for espacio in espacios_actividad.iter() {
                                                        let base = espacio
                                                            .espacio_nombre
                                                            .trim()
                                                            .trim_end_matches(|c: char| c.is_ascii_digit() || c == ' ')
                                                            .to_string();
                                                        grupos.entry(base).or_insert_with(Vec::new).push(espacio.clone());
                                                    }
                                                    let mut nombres_base: Vec<String> = grupos.keys().cloned().collect();
                                                    nombres_base.sort();
                                                    let mut nombres_espacios: Vec<String> = Vec::new();
                                                    for base in nombres_base.iter() {
                                                        let grupo = grupos.get(base).unwrap();
                                                        let mut numeros: Vec<String> = grupo
                                                            .iter()
                                                            .filter_map(|e| {
                                                                let nombre = e.espacio_nombre.trim();
                                                                nombre
                                                                    .strip_prefix(base)
                                                                    .map(|s| s.trim())
                                                                    .filter(|s| !s.is_empty())
                                                                    .map(|n| n.to_string())
                                                            })
                                                            .collect();
                                                        numeros.sort();
                                                        let nombre_mostrar = if !numeros.is_empty() {
                                                            format!("{} {}", base, numeros.join(", "))
                                                        } else {
                                                            base.clone()
                                                        };
                                                        nombres_espacios.push(nombre_mostrar);
                                                    }
                                                    rsx! {
                                                        div { class: "border-l-4 border-zinc-300 dark:border-zinc-700 pl-4 py-3 space-y-3",
                                                            div { class: "flex items-center gap-3",
                                                                h3 { class: "text-lg font-bold text-zinc-950 dark:text-zinc-50", "{actividad}" }
                                                                Badge { variant: BadgeVariant::Secondary,
                                                                    {
                                                                        format!(
                                                                            "{} espacio{}",
                                                                            espacios_actividad.len(),
                                                                            if espacios_actividad.len() == 1 { "" } else { "s" },
                                                                        )
                                                                    }
                                                                }
                                                            }

                                                            div { class: "space-y-2",
                                                                if !tipos.is_empty() {
                                                                    div { class: "flex gap-2 items-center",
                                                                        span { class: "text-zinc-600 dark:text-zinc-400 text-sm min-w-[100px]",
                                                                            "Tipo:"
                                                                        }
                                                                        span { class: "text-zinc-950 dark:text-zinc-50 text-sm", "{tipos.join(\", \")}" }
                                                                    }
                                                                }

                                                                if !pavimentos.is_empty() {
                                                                    div { class: "flex gap-2 items-center",
                                                                        span { class: "text-zinc-600 dark:text-zinc-400 text-sm min-w-[100px]",
                                                                            "Pavimento:"
                                                                        }
                                                                        div { class: "flex flex-wrap gap-1",
                                                                            for pav in pavimentos.iter() {
                                                                                Badge { variant: BadgeVariant::Default, "{pav}" }
                                                                            }
                                                                        }
                                                                    }
                                                                }

                                                                div { class: "flex gap-2 items-center flex-wrap",
                                                                    span { class: "text-zinc-600 dark:text-zinc-400 text-sm min-w-[100px]", "Características:" }
                                                                    div { class: "flex flex-wrap gap-2",
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
                                                                }
                                                            }

                                                            div { class: "pt-2",
                                                                span { class: "text-zinc-600 dark:text-zinc-400 text-sm block mb-1", "Espacios:" }
                                                                p { class: "text-zinc-950 dark:text-zinc-50 text-sm", "{nombres_espacios.join(\" • \")}" }
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
