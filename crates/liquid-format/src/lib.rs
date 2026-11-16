//! Russian UI formatting shared by apps: "2,4 МБ", "3 КБ", "22 авг.".
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Short month names used across the mockups.
pub const MONTHS_SHORT: [&str; 12] = [
    "янв.",
    "февр.",
    "мар.",
    "апр.",
    "мая",
    "июн.",
    "июл.",
    "авг.",
    "сент.",
    "окт.",
    "нояб.",
    "дек.",
];

/// Bytes as "3 КБ", "2,4 МБ", "18,7 МБ", "512 ГБ" (decimal comma, 1024 base).
pub fn size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["Б", "КБ", "МБ", "ГБ", "ТБ"];
    let mut v = bytes as f64;
    let mut u = 0;
    while v >= 1024.0 && u < UNITS.len() - 1 {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 || v >= 100.0 || (v - v.round()).abs() < 0.05 {
        format!("{} {}", v.round() as u64, UNITS[u])
    } else {
        format!("{:.1} {}", v, UNITS[u]).replace('.', ",")
    }
}

/// Day label in lists: "Сегодня", "Вчера" or "22 авг.".
/// Days are counted since the Unix epoch in local time.
pub fn day_label(today: i64, day: i64, day_of_month: u32, month: usize) -> String {
    match today - day {
        0 => "Сегодня".into(),
        1 => "Вчера".into(),
        _ => format!("{} {}", day_of_month, MONTHS_SHORT[(month.max(1) - 1) % 12]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_match_mockup() {
        assert_eq!(size(3 * 1024), "3 КБ");
        assert_eq!(size(2_516_582), "2,4 МБ");
        assert_eq!(size(19_608_371), "18,7 МБ");
        assert_eq!(size(512 * 1024 * 1024 * 1024), "512 ГБ");
    }

    #[test]
    fn days() {
        assert_eq!(day_label(100, 100, 23, 8), "Сегодня");
        assert_eq!(day_label(100, 98, 22, 8), "22 авг.");
    }
}
