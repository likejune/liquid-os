//! Settings: sidebar sections and the "Оформление" page model.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use liquid_design::{Accent, Theme};

/// Sidebar sections, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// Wi-Fi.
    WiFi,
    /// Bluetooth.
    Bluetooth,
    /// Сеть.
    Network,
    /// Экран.
    Display,
    /// Звук.
    Sound,
    /// Клавиатура.
    Keyboard,
    /// Оформление.
    Appearance,
    /// Конфиденциальность.
    Privacy,
    /// Пользователи.
    Users,
    /// Обновление ПО.
    Updates,
    /// О системе.
    About,
}

impl Section {
    /// All sections in sidebar order.
    pub const ALL: [Section; 11] = [
        Section::WiFi,
        Section::Bluetooth,
        Section::Network,
        Section::Display,
        Section::Sound,
        Section::Keyboard,
        Section::Appearance,
        Section::Privacy,
        Section::Users,
        Section::Updates,
        Section::About,
    ];

    /// Russian label.
    pub const fn label(self) -> &'static str {
        match self {
            Section::WiFi => "Wi-Fi",
            Section::Bluetooth => "Bluetooth",
            Section::Network => "Сеть",
            Section::Display => "Экран",
            Section::Sound => "Звук",
            Section::Keyboard => "Клавиатура",
            Section::Appearance => "Оформление",
            Section::Privacy => "Конфиденциальность",
            Section::Users => "Пользователи",
            Section::Updates => "Обновление ПО",
            Section::About => "О системе",
        }
    }

    /// Sections whose label contains the query (sidebar search "Поиск").
    pub fn search(query: &str) -> Vec<Section> {
        let q = query.trim().to_lowercase();
        Self::ALL
            .into_iter()
            .filter(|s| s.label().to_lowercase().contains(&q))
            .collect()
    }
}

/// "Оформление" page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Appearance {
    /// Светлый / Тёмный.
    pub theme: Theme,
    /// Акцентный цвет.
    pub accent: Accent,
    /// Wallpaper index.
    pub wallpaper: usize,
    /// "Прозрачность и размытие" (off = solid surfaces for weak GPUs).
    pub transparency: bool,
    /// "Уменьшить движение".
    pub reduce_motion: bool,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            accent: Accent::Blue,
            wallpaper: 0,
            transparency: true,
            reduce_motion: false,
        }
    }
}

impl Appearance {
    /// Serialize to the key=value file read by the compositor and shell
    /// (`~/.config/liquid/appearance.conf`).
    pub fn to_conf(&self) -> String {
        format!(
            "theme={}\naccent={}\nwallpaper={}\ntransparency={}\nreduce_motion={}\n",
            match self.theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            },
            Accent::ALL
                .iter()
                .position(|a| *a == self.accent)
                .unwrap_or(0),
            self.wallpaper,
            self.transparency,
            self.reduce_motion
        )
    }

    /// Parse the file; unknown keys are ignored, missing keys keep defaults.
    pub fn from_conf(text: &str) -> Self {
        let mut a = Self::default();
        for (k, v) in text.lines().filter_map(|l| l.split_once('=')) {
            match k.trim() {
                "theme" => {
                    a.theme = if v.trim() == "dark" {
                        Theme::Dark
                    } else {
                        Theme::Light
                    }
                }
                "accent" => {
                    a.accent = v
                        .trim()
                        .parse::<usize>()
                        .ok()
                        .and_then(|i| Accent::ALL.get(i).copied())
                        .unwrap_or(a.accent)
                }
                "wallpaper" => a.wallpaper = v.trim().parse().unwrap_or(a.wallpaper),
                "transparency" => a.transparency = v.trim() == "true",
                "reduce_motion" => a.reduce_motion = v.trim() == "true",
                _ => {}
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven_sections() {
        assert_eq!(Section::ALL.len(), 11);
        assert_eq!(Section::search("обнов"), vec![Section::Updates]);
    }

    #[test]
    fn appearance_roundtrip() {
        let a = Appearance {
            theme: Theme::Dark,
            accent: Accent::Pink,
            wallpaper: 3,
            transparency: false,
            reduce_motion: true,
        };
        assert_eq!(Appearance::from_conf(&a.to_conf()), a);
    }
}
