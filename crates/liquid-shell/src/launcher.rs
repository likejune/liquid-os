//! App launcher "Меню" (1000073175): search, 3 pinned apps, 4-column grid.

use liquid_design::layout;

/// An installed application (own app or Flatpak).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// Desktop entry id, e.g. "org.mozilla.firefox".
    pub id: String,
    /// Localized name shown in the grid.
    pub name: String,
    /// Extra search keywords.
    pub keywords: Vec<String>,
}

impl App {
    /// New app without keywords.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            keywords: Vec::new(),
        }
    }
}

/// Ids of the three apps pinned in the dock and launcher top row.
pub const PINNED: [&str; 3] = ["os.liquid.Finder", "os.liquid.Menu", "os.liquid.Notes"];

/// Apps matching a query (case-insensitive, by name, id or keyword),
/// pinned apps first, then alphabetical.
pub fn search<'a>(apps: &'a [App], query: &str) -> Vec<&'a App> {
    let q = query.trim().to_lowercase();
    let mut out: Vec<&App> = apps
        .iter()
        .filter(|a| {
            q.is_empty()
                || a.name.to_lowercase().contains(&q)
                || a.id.to_lowercase().contains(&q)
                || a.keywords.iter().any(|k| k.to_lowercase().contains(&q))
        })
        .collect();
    out.sort_by_key(|a| (!PINNED.contains(&a.id.as_str()), a.name.to_lowercase()));
    out
}

/// Grid position (row, column) of the n-th app.
pub fn grid_cell(index: usize) -> (usize, usize) {
    (
        index / layout::LAUNCHER_COLUMNS,
        index % layout::LAUNCHER_COLUMNS,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apps() -> Vec<App> {
        vec![
            App::new("org.mozilla.firefox", "Браузер"),
            App::new("os.liquid.Notes", "Заметки"),
            App::new("os.liquid.Finder", "Finder"),
            App::new("org.gnome.Calendar", "Календарь"),
        ]
    }

    #[test]
    fn pinned_first() {
        let a = apps();
        let r = search(&a, "");
        assert_eq!(r[0].id, "os.liquid.Finder");
        assert_eq!(r[1].id, "os.liquid.Notes");
    }

    #[test]
    fn search_russian() {
        let a = apps();
        let r = search(&a, "кален");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].name, "Календарь");
    }

    #[test]
    fn grid_is_four_columns() {
        assert_eq!(grid_cell(5), (1, 1));
    }
}
