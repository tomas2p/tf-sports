use chrono::{NaiveDate, Datelike};

/// Formatea una fecha en español, ejemplo: "lunes 3 de febrero"
pub fn fecha_en_espanol(date: NaiveDate) -> String {
    let dia_semana = day_name_es(date.weekday());
    let mes = month_name_es(date.month());
    format!("{} {} de {}", dia_semana, date.day(), mes)
}

/// Retorna el nombre completo del día de la semana en español (minúscula)
pub fn day_name_es(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "lunes",
        chrono::Weekday::Tue => "martes",
        chrono::Weekday::Wed => "miércoles",
        chrono::Weekday::Thu => "jueves",
        chrono::Weekday::Fri => "viernes",
        chrono::Weekday::Sat => "sábado",
        chrono::Weekday::Sun => "domingo",
    }
}

/// Retorna el nombre corto del día de la semana en español
pub fn day_name_short_es(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Lun",
        chrono::Weekday::Tue => "Mar",
        chrono::Weekday::Wed => "Mié",
        chrono::Weekday::Thu => "Jue",
        chrono::Weekday::Fri => "Vie",
        chrono::Weekday::Sat => "Sáb",
        chrono::Weekday::Sun => "Dom",
    }
}

/// Retorna el nombre del mes en español (primera letra mayúscula)
pub fn month_name_es(month: u32) -> &'static str {
    match month {
        1 => "Enero",
        2 => "Febrero",
        3 => "Marzo",
        4 => "Abril",
        5 => "Mayo",
        6 => "Junio",
        7 => "Julio",
        8 => "Agosto",
        9 => "Septiembre",
        10 => "Octubre",
        11 => "Noviembre",
        12 => "Diciembre",
        _ => "",
    }
}

/// Retorna el nombre del mes en español (minúscula)
pub fn _month_name_es_lower(month: u32) -> &'static str {
    match month {
        1 => "enero",
        2 => "febrero",
        3 => "marzo",
        4 => "abril",
        5 => "mayo",
        6 => "junio",
        7 => "julio",
        8 => "agosto",
        9 => "septiembre",
        10 => "octubre",
        11 => "noviembre",
        12 => "diciembre",
        _ => "",
    }
}
