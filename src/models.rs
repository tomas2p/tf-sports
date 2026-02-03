use serde::{Deserialize, Serialize};

/// Estructura que representa un tipo de deporte con su nombre y emoji
#[derive(Debug, Clone, PartialEq)]
pub struct DeporteInfo {
    pub nombre: &'static str,
    pub emoji: &'static str,
    pub keyword: &'static str,
}

/// Lista de todos los deportes disponibles (ordenados por longitud de keyword para matching específico)
pub const DEPORTES: &[DeporteInfo] = &[
    DeporteInfo { nombre: "Campo a Través", emoji: "🏃", keyword: "campo a través" },
    DeporteInfo { nombre: "Marcha Nórdica", emoji: "🥾", keyword: "marcha nórdica" },
    DeporteInfo { nombre: "Natación Artística", emoji: "🏊‍♀️", keyword: "natación artística" },
    DeporteInfo { nombre: "Patinaje Artístico", emoji: "⛸️", keyword: "patinaje artístico" },
    DeporteInfo { nombre: "Carrera de Obstáculos", emoji: "🏃‍♂️", keyword: "carrera de obstáculos" },
    DeporteInfo { nombre: "Tenis de Mesa", emoji: "🏓", keyword: "tenis de mesa" },
    DeporteInfo { nombre: "Bola Canaria", emoji: "🎳", keyword: "bola canaria" },
    DeporteInfo { nombre: "Lucha Canaria", emoji: "🤼", keyword: "lucha canaria" },
    DeporteInfo { nombre: "Paddle Surf", emoji: "🏄", keyword: "paddle surf" },
    DeporteInfo { nombre: "Vóley-Playa", emoji: "🏐", keyword: "vóley-playa" },
    DeporteInfo { nombre: "Senderismo", emoji: "🥾", keyword: "senderismo" },
    DeporteInfo { nombre: "Atletismo", emoji: "🏃", keyword: "atletismo" },
    DeporteInfo { nombre: "Balonmano", emoji: "🤾", keyword: "balonmano" },
    DeporteInfo { nombre: "Baloncesto", emoji: "🏀", keyword: "baloncesto" },
    DeporteInfo { nombre: "Bádminton", emoji: "🏸", keyword: "bádminton" },
    DeporteInfo { nombre: "Ciclismo", emoji: "🚴", keyword: "ciclismo" },
    DeporteInfo { nombre: "Frontenis", emoji: "🎾", keyword: "frontenis" },
    DeporteInfo { nombre: "Natación", emoji: "🏊", keyword: "natación" },
    DeporteInfo { nombre: "Orientación", emoji: "🧭", keyword: "orientación" },
    DeporteInfo { nombre: "Salvamento", emoji: "🏊‍♂️", keyword: "salvamento" },
    DeporteInfo { nombre: "Taekwondo", emoji: "🥋", keyword: "taekwondo" },
    DeporteInfo { nombre: "Triatlón", emoji: "🏊‍♂️", keyword: "triatlón" },
    DeporteInfo { nombre: "Voleibol", emoji: "🏐", keyword: "voleibol" },
    DeporteInfo { nombre: "Waterpolo", emoji: "🤽", keyword: "waterpolo" },
    DeporteInfo { nombre: "Ajedrez", emoji: "♟️", keyword: "ajedrez" },
    DeporteInfo { nombre: "Béisbol", emoji: "⚾", keyword: "béisbol" },
    DeporteInfo { nombre: "Kayak", emoji: "🛶", keyword: "kayak" },
    DeporteInfo { nombre: "Rugby", emoji: "🏉", keyword: "rugby" },
    DeporteInfo { nombre: "Surf", emoji: "🏄", keyword: "surf" },
    DeporteInfo { nombre: "Taichí", emoji: "🧘", keyword: "taichí" },
    DeporteInfo { nombre: "Trail", emoji: "🏃‍♀️", keyword: "trail" },
    DeporteInfo { nombre: "Yoga", emoji: "🧘‍♀️", keyword: "yoga" },
    DeporteInfo { nombre: "Judo", emoji: "🥋", keyword: "judo" },
    DeporteInfo { nombre: "Lucha", emoji: "🤼", keyword: "lucha" },
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventoData {
    pub eventos: Vec<Evento>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evento {
    pub evento_nombre: String,
    pub evento_url: String,
    pub evento_descripcion: String,
    pub evento_lugar: Option<String>,
    pub municipio_nombre: Option<String>,
    pub evento_organizador: String,
    pub evento_fecha_inicio: String,
    pub evento_fecha_fin: String,
}

impl Evento {
    pub fn get_badge_variant(&self) -> &'static str {
        use chrono::NaiveDateTime;
        
        if let Ok(fecha_inicio) = NaiveDateTime::parse_from_str(&self.evento_fecha_inicio, "%Y-%m-%d %H:%M:%S") {
            let now = chrono::Local::now().naive_local();
            
            if let Ok(fecha_fin) = NaiveDateTime::parse_from_str(&self.evento_fecha_fin, "%Y-%m-%d %H:%M:%S") {
                if now >= fecha_inicio && now <= fecha_fin {
                    return "EN VIVO";
                } else if now > fecha_fin {
                    return "FINALIZADO";
                }
            }
        }
        
        "PRÓXIMO"
    }
    
    pub fn format_fecha(&self) -> String {
        use chrono::NaiveDateTime;
        
        if let Ok(fecha) = NaiveDateTime::parse_from_str(&self.evento_fecha_inicio, "%Y-%m-%d %H:%M:%S") {
            fecha.format("%d/%m/%Y • %H:%M hrs").to_string()
        } else {
            self.evento_fecha_inicio.clone()
        }
    }
    
    pub fn get_deporte(&self) -> String {
        let nombre = self.evento_nombre.to_lowercase();
        let desc = self.evento_descripcion.to_lowercase();
        
        // Buscar en nombre primero, luego en descripción
        DEPORTES
            .iter()
            .find(|d| nombre.contains(d.keyword) || desc.contains(d.keyword))
            .map(|d| d.nombre.to_string())
            .unwrap_or_else(|| "Varios".to_string())
    }
    
    pub fn get_deporte_emoji(&self) -> &'static str {
        let nombre = self.evento_nombre.to_lowercase();
        let desc = self.evento_descripcion.to_lowercase();
        
        DEPORTES
            .iter()
            .find(|d| nombre.contains(d.keyword) || desc.contains(d.keyword))
            .map(|d| d.emoji)
            .unwrap_or("🏆")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EspacioDeportivo {
    pub instalacion_codigo: i64,
    pub espacio_codigo: i64,
    pub espacio_nombre: String,
    pub espacio_tipo: Option<String>,
    pub espacio_clase: Option<String>,
    pub espacio_actividad_principal: Option<String>,
    pub pavimento_tipo: Option<String>,
    pub pavimento_conservacion: Option<String>,
    pub espacio_cerramiento: Option<String>,
    pub espacio_estado_uso: Option<String>,
    pub espacio_calefaccion: Option<String>,
    pub espacio_climatizacion: Option<String>,
    pub espacio_iluminacion: Option<String>,
    pub ultima_modificacion: String,
}

// Modelo para instalaciones deportivas (GeoJSON)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalacionesGeoJSON {
    pub features: Vec<InstalacionFeature>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalacionFeature {
    pub properties: InstalacionDeportiva,
    pub geometry: Geometry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalacionDeportiva {
    pub instalacion_codigo: i64,
    pub instalacion_nombre: String,
    pub municipio_nombre: String,
    pub codigo_postal: Option<String>,
    pub email: Option<String>,
    pub telefono_fijo: Option<String>,
    pub web: Option<String>,
    pub fax: Option<String>,
    pub propiedad: Option<String>,
    pub tipo_gestion: Option<String>,
    pub observaciones: Option<String>,
    pub longitud: f64,
    pub latitud: f64,
    pub ultima_modificacion: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geom_type: String,
    pub coordinates: Vec<f64>,
}