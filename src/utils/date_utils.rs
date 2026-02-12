use chrono::{Datelike, NaiveDate};

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_fecha_en_espanol_basica() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 12).unwrap();
        let s = fecha_en_espanol(date);
        assert_eq!(s, "jueves 12 de Febrero");
    }

    #[test]
    fn test_day_name_short() {
        assert_eq!(day_name_short_es(chrono::Weekday::Mon), "Lun");
        assert_eq!(day_name_short_es(chrono::Weekday::Sun), "Dom");
    }

    #[test]
    fn test_month_name_edges() {
        assert_eq!(month_name_es(1), "Enero");
        assert_eq!(month_name_es(12), "Diciembre");
        assert_eq!(month_name_es(0), "");
    }

    #[test]
    fn test_parse_naive_datetime_format() {
        let s = "2026-02-12 14:30:00";
        let dt = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2026, 2, 12).unwrap());
    }
}
