use bson::DateTime;
use chrono::Datelike;

pub fn format_date_3m_dsfx_year(d: DateTime) -> String {
    let cdt: chrono::DateTime<chrono::Utc> = d.into();
    let day = cdt.day();
    let suffix = match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        11..=13 => "th",
        _ => "th",
    };
    let f = format!("%b %d{} %Y", suffix);

    cdt.format(&f).to_string()
}
