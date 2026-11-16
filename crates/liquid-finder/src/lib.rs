//! Finder model: sidebar, tags, view modes, sorting, search, directory listing.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Sidebar "Избранное" entries, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    /// Главная (home).
    Home,
    /// Недавние.
    Recent,
    /// Документы.
    Documents,
    /// Изображения.
    Pictures,
    /// Загрузки.
    Downloads,
    /// Корзина.
    Trash,
}

impl Place {
    /// All places in sidebar order.
    pub const ALL: [Place; 6] = [
        Place::Home,
        Place::Recent,
        Place::Documents,
        Place::Pictures,
        Place::Downloads,
        Place::Trash,
    ];

    /// Russian label.
    pub const fn label(self) -> &'static str {
        match self {
            Place::Home => "Главная",
            Place::Recent => "Недавние",
            Place::Documents => "Документы",
            Place::Pictures => "Изображения",
            Place::Downloads => "Загрузки",
            Place::Trash => "Корзина",
        }
    }

    /// Directory for a place under `home` (XDG defaults). Recent has none.
    pub fn path(self, home: &Path) -> Option<PathBuf> {
        match self {
            Place::Home => Some(home.to_path_buf()),
            Place::Recent => None,
            Place::Documents => Some(home.join("Documents")),
            Place::Pictures => Some(home.join("Pictures")),
            Place::Downloads => Some(home.join("Downloads")),
            Place::Trash => Some(home.join(".local/share/Trash/files")),
        }
    }
}

/// Default tags with their colors (hex) from the mockup.
pub const TAGS: [(&str, &str); 5] = [
    ("Работа", "#3B82F6"),
    ("Личное", "#EC4899"),
    ("Учёба", "#14B8A6"),
    ("Важное", "#F97316"),
    ("Проекты", "#A855F7"),
];

/// View mode switcher in the toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    /// Grid of large icons (default in the mockup).
    #[default]
    Grid,
    /// List.
    List,
    /// Columns.
    Columns,
}

/// "Сортировка" dropdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortBy {
    /// By name, folders first.
    #[default]
    Name,
    /// Newest first.
    Modified,
    /// Largest first.
    Size,
    /// By extension.
    Kind,
}

/// One entry in a folder.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// File name.
    pub name: String,
    /// Directory or file.
    pub is_dir: bool,
    /// Size, bytes (0 for directories).
    pub size: u64,
    /// Last modification.
    pub modified: SystemTime,
}

impl Entry {
    /// Subtitle under a file in grid view ("2,4 МБ"); folders have none.
    pub fn subtitle(&self) -> Option<String> {
        (!self.is_dir).then(|| liquid_format::size(self.size))
    }

    fn ext(&self) -> String {
        Path::new(&self.name)
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default()
    }
}

/// Sort entries in place; folders always come first.
pub fn sort(entries: &mut [Entry], by: SortBy) {
    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| match by {
            SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortBy::Modified => b.modified.cmp(&a.modified),
            SortBy::Size => b.size.cmp(&a.size),
            SortBy::Kind => a.ext().cmp(&b.ext()).then_with(|| a.name.cmp(&b.name)),
        })
    });
}

/// "Поиск в Finder": case-insensitive substring match on names.
pub fn filter<'a>(entries: &'a [Entry], query: &str) -> Vec<&'a Entry> {
    let q = query.trim().to_lowercase();
    entries
        .iter()
        .filter(|e| q.is_empty() || e.name.to_lowercase().contains(&q))
        .collect()
}

/// Read a directory (hidden files skipped).
pub fn list_dir(dir: &Path) -> std::io::Result<Vec<Entry>> {
    let mut out = Vec::new();
    for de in std::fs::read_dir(dir)? {
        let de = de?;
        let name = de.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let md = de.metadata()?;
        out.push(Entry {
            name,
            is_dir: md.is_dir(),
            size: if md.is_dir() { 0 } else { md.len() },
            modified: md.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn e(name: &str, dir: bool, size: u64, t: u64) -> Entry {
        Entry {
            name: name.into(),
            is_dir: dir,
            size,
            modified: SystemTime::UNIX_EPOCH + Duration::from_secs(t),
        }
    }

    #[test]
    fn folders_first_then_name() {
        let mut v = vec![
            e("notes.txt", false, 3072, 1),
            e("Архив", true, 0, 1),
            e("IMG_4827.jpg", false, 2_516_582, 2),
        ];
        sort(&mut v, SortBy::Name);
        assert!(v[0].is_dir);
        assert_eq!(v[1].name, "IMG_4827.jpg");
        assert_eq!(v[1].subtitle().as_deref(), Some("2,4 МБ"));
    }

    #[test]
    fn search() {
        let v = vec![
            e("palette.png", false, 1, 1),
            e("inspiration.psd", false, 1, 1),
        ];
        assert_eq!(filter(&v, "PAL").len(), 1);
    }

    #[test]
    fn sidebar() {
        assert_eq!(Place::ALL.len(), 6);
        assert_eq!(TAGS.len(), 5);
        assert!(Place::Recent.path(Path::new("/home/linki")).is_none());
    }
}
