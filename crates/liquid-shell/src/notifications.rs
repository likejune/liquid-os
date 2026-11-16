//! Notification toasts (1000073171): "Готово / Файл успешно сохранён / 2 мин назад".

/// One notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    /// Id from org.freedesktop.Notifications.
    pub id: u32,
    /// App that sent it.
    pub app: String,
    /// Title line.
    pub summary: String,
    /// Body line.
    pub body: String,
    /// Unix time, seconds.
    pub time: u64,
}

/// "только что", "2 мин назад", "3 ч назад", "2 дн назад".
pub fn relative_time(now: u64, then: u64) -> String {
    let s = now.saturating_sub(then);
    match s {
        0..=59 => "только что".into(),
        60..=3599 => format!("{} мин назад", s / 60),
        3600..=86399 => format!("{} ч назад", s / 3600),
        _ => format!("{} дн назад", s / 86400),
    }
}

/// Visible toasts: newest first, at most `max`, none while Do not disturb is on.
pub fn visible(list: &[Notification], do_not_disturb: bool, max: usize) -> Vec<&Notification> {
    if do_not_disturb {
        return Vec::new();
    }
    let mut v: Vec<&Notification> = list.iter().collect();
    v.sort_by_key(|n| std::cmp::Reverse(n.time));
    v.truncate(max);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative() {
        assert_eq!(relative_time(1000, 880), "2 мин назад");
        assert_eq!(relative_time(1000, 990), "только что");
    }

    #[test]
    fn dnd_hides() {
        let n = Notification {
            id: 1,
            app: "Finder".into(),
            summary: "Готово".into(),
            body: "Файл успешно сохранён".into(),
            time: 1,
        };
        assert!(visible(std::slice::from_ref(&n), true, 3).is_empty());
        assert_eq!(visible(std::slice::from_ref(&n), false, 3).len(), 1);
    }
}
