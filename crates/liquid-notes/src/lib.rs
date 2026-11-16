//! Notes model: folders with counters, favorites, recently deleted, search,
//! checklists and callouts ("Примечание").
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// A block inside a note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// Paragraph.
    Text(String),
    /// Checklist line.
    Check {
        /// Text.
        text: String,
        /// Ticked.
        done: bool,
    },
    /// Callout box "Примечание".
    Callout(String),
}

/// A note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    /// Id.
    pub id: u64,
    /// Title.
    pub title: String,
    /// Content.
    pub blocks: Vec<Block>,
    /// Folder name, None = no folder.
    pub folder: Option<String>,
    /// In "Избранное".
    pub favorite: bool,
    /// In "Недавно удалённые".
    pub deleted: bool,
    /// Last edit, days since epoch.
    pub modified_day: i64,
}

impl Note {
    /// Excerpt shown in the list: first text, up to `max` chars, with "...".
    pub fn excerpt(&self, max: usize) -> String {
        let text: String = self
            .blocks
            .iter()
            .map(|b| match b {
                Block::Text(t) | Block::Callout(t) => t.as_str(),
                Block::Check { text, .. } => text.as_str(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        if text.chars().count() <= max {
            text
        } else {
            let cut: String = text.chars().take(max).collect();
            format!("{}...", cut.trim_end())
        }
    }
}

/// Sidebar selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// "Все заметки".
    All,
    /// "Избранное".
    Favorites,
    /// "Недавно удалённые".
    Deleted,
    /// A folder.
    Folder(String),
}

/// Notes in a view, newest first, filtered by search query.
pub fn notes_in<'a>(notes: &'a [Note], view: &View, query: &str) -> Vec<&'a Note> {
    let q = query.trim().to_lowercase();
    let mut v: Vec<&Note> = notes
        .iter()
        .filter(|n| match view {
            View::Deleted => n.deleted,
            View::All => !n.deleted,
            View::Favorites => !n.deleted && n.favorite,
            View::Folder(f) => !n.deleted && n.folder.as_deref() == Some(f.as_str()),
        })
        .filter(|n| {
            q.is_empty()
                || n.title.to_lowercase().contains(&q)
                || n.excerpt(10_000).to_lowercase().contains(&q)
        })
        .collect();
    v.sort_by_key(|n| std::cmp::Reverse(n.modified_day));
    v
}

/// Counter badge next to a sidebar entry.
pub fn count(notes: &[Note], view: &View) -> usize {
    notes_in(notes, view, "").len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(id: u64, folder: &str, fav: bool, del: bool, day: i64) -> Note {
        Note {
            id,
            title: format!("Заметка {id}"),
            blocks: vec![Block::Text("Интерфейс должен быть лёгким".into())],
            folder: Some(folder.into()),
            favorite: fav,
            deleted: del,
            modified_day: day,
        }
    }

    #[test]
    fn counters() {
        let v = vec![
            n(1, "Работа", true, false, 3),
            n(2, "Работа", false, false, 2),
            n(3, "Идеи", false, true, 1),
        ];
        assert_eq!(count(&v, &View::All), 2);
        assert_eq!(count(&v, &View::Favorites), 1);
        assert_eq!(count(&v, &View::Deleted), 1);
        assert_eq!(count(&v, &View::Folder("Работа".into())), 2);
        assert_eq!(notes_in(&v, &View::All, "")[0].id, 1, "newest first");
    }

    #[test]
    fn excerpt_cut() {
        let x = n(1, "Идеи", false, false, 1).excerpt(9);
        assert!(x.ends_with("..."));
    }

    #[test]
    fn search_body() {
        let v = vec![n(1, "Идеи", false, false, 1)];
        assert_eq!(notes_in(&v, &View::All, "лёгким").len(), 1);
    }
}
