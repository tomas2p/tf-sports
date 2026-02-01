use serde::{Deserialize, Serialize};

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
        
        if nombre.contains("ajedrez") { "Ajedrez".to_string() }
        else if nombre.contains("ciclismo") { "Ciclismo".to_string() }
        else if nombre.contains("natación") { "Natación".to_string() }
        else if nombre.contains("voleibol") { "Voleibol".to_string() }
        else if nombre.contains("taekwondo") { "Taekwondo".to_string() }
        else if nombre.contains("balonmano") { "Balonmano".to_string() }
        else if nombre.contains("baloncesto") { "Baloncesto".to_string() }
        else if nombre.contains("lucha") { "Lucha Canaria".to_string() }
        else if nombre.contains("bola canaria") { "Bola Canaria".to_string() }
        else if nombre.contains("salvamento") { "Salvamento".to_string() }
        else if nombre.contains("orientación") { "Orientación".to_string() }
        else if nombre.contains("tenis de mesa") { "Tenis de Mesa".to_string() }
        else if nombre.contains("waterpolo") { "Waterpolo".to_string() }
        else if nombre.contains("bádminton") { "Bádminton".to_string() }
        else { "Varios".to_string() }
    }
}