// Centralizar inclusiones de JSON para evitar repetición en múltiples archivos
pub const EVENTOS_JSON: &str =
    include_str!("../data/agenda-de-eventos-deportivos-en-tenerife.json");
pub const INSTALACIONES_JSON: &str = include_str!("../data/instalaciones-deportivas.geojson");
pub const ESPACIOS_JSON: &str = include_str!("../data/espacios-deportivos.json");

use std::sync::OnceLock;

use crate::models::{EspacioDeportivo, EventoData, InstalacionesGeoJSON};

// Cache deserializado de eventos
static EVENTOS_CACHE: OnceLock<EventoData> = OnceLock::new();
pub fn get_eventos() -> &'static EventoData {
    EVENTOS_CACHE.get_or_init(|| {
        serde_json::from_str(EVENTOS_JSON).unwrap_or(EventoData { eventos: vec![] })
    })
}

// Cache deserializado de instalaciones (GeoJSON)
static INSTALACIONES_CACHE: OnceLock<InstalacionesGeoJSON> = OnceLock::new();
pub fn get_instalaciones() -> &'static InstalacionesGeoJSON {
    INSTALACIONES_CACHE.get_or_init(|| {
        serde_json::from_str(INSTALACIONES_JSON)
            .unwrap_or(InstalacionesGeoJSON { features: vec![] })
    })
}

// Espacios deportivos: el JSON tiene una envoltura { "espacios_deportivos": [...] }
#[derive(serde::Deserialize)]
struct EspaciosWrapper {
    espacios_deportivos: Vec<EspacioDeportivo>,
}

static ESPACIOS_CACHE: OnceLock<Vec<EspacioDeportivo>> = OnceLock::new();
pub fn get_espacios() -> &'static Vec<EspacioDeportivo> {
    ESPACIOS_CACHE.get_or_init(|| {
        serde_json::from_str::<EspaciosWrapper>(ESPACIOS_JSON)
            .map(|w| w.espacios_deportivos)
            .unwrap_or_else(|_| vec![])
    })
}
