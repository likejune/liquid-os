//! Top menu bar (1000073174, 1000073176): app menus left, status right.

/// Standard menus of an app, in order.
pub const APP_MENUS: [&str; 6] = ["Файл", "Правка", "Вид", "Переход", "Окно", "Справка"];

const WEEKDAYS: [&str; 7] = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"];
const MONTHS: [&str; 12] = [
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

/// Clock text exactly as in the mockups: "Ср, 27 авг.  22:46".
/// `weekday` 0 = Monday, `month` 1..=12.
pub fn clock_text(weekday: usize, day: u32, month: usize, hour: u32, minute: u32) -> String {
    format!(
        "{}, {} {}  {:02}:{:02}",
        WEEKDAYS[weekday % 7],
        day,
        MONTHS[(month.max(1) - 1) % 12],
        hour,
        minute
    )
}

/// Title shown next to the logo: the focused app name, or "Liquid OS" on the desktop.
pub fn title(focused_app: Option<&str>) -> &str {
    focused_app.unwrap_or("Liquid OS")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_matches_mockup() {
        assert_eq!(clock_text(2, 27, 8, 22, 46), "Ср, 27 авг.  22:46");
        assert_eq!(clock_text(5, 23, 8, 16, 42), "Сб, 23 авг.  16:42");
    }

    #[test]
    fn desktop_title() {
        assert_eq!(title(None), "Liquid OS");
        assert_eq!(title(Some("Заметки")), "Заметки");
    }
}
