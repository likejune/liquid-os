//! Screen order and validation, exactly as in the design matrix.

use liquid_design::{Accent, Theme};

/// Every screen of installation and first run, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// 1. "Liquid OS / Быстрее. Легче. Красивее." + "Начать установку".
    Welcome,
    /// 2. "Выбор языка".
    Language,
    /// 3. "Раскладка клавиатуры".
    Keyboard,
    /// 4. "Лицензионное соглашение".
    License,
    /// 5. "Выбор диска".
    Disk,
    /// 6. "Компоненты установки".
    Components,
    /// 7. "Установка Liquid OS" with progress.
    Installing,
    /// 8. "Установка завершена".
    Installed,
    /// 9. "Добро пожаловать в Liquid OS".
    Hello,
    /// 10. "Как тебя зовут?".
    Name,
    /// 11. "Выбери акцентный цвет".
    AccentColor,
    /// 12. "Выбери обои".
    Wallpaper,
    /// 13. "Оформление".
    Appearance,
    /// 14. "Параметры входа".
    SignIn,
    /// 15. "Всё готово!".
    Done,
}

const ORDER: [Screen; 15] = [
    Screen::Welcome,
    Screen::Language,
    Screen::Keyboard,
    Screen::License,
    Screen::Disk,
    Screen::Components,
    Screen::Installing,
    Screen::Installed,
    Screen::Hello,
    Screen::Name,
    Screen::AccentColor,
    Screen::Wallpaper,
    Screen::Appearance,
    Screen::SignIn,
    Screen::Done,
];

/// Languages offered on screen 2.
pub const LANGUAGES: [(&str, &str); 4] = [
    ("ru_RU", "Русский"),
    ("en_US", "English"),
    ("zh_CN", "中文 (简体)"),
    ("es_ES", "Español"),
];

/// Keyboard layouts offered on screen 3.
pub const LAYOUTS: [(&str, &str); 4] = [
    ("ru", "Русская"),
    ("us", "Английская (США)"),
    ("gb", "Английская (Великобритания)"),
    ("ua", "Украинская"),
];

/// Optional components on screen 6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Components {
    /// "Офисные приложения".
    pub office: bool,
    /// "Дополнительные пакеты".
    pub extras: bool,
    /// "Разработчик".
    pub developer: bool,
}

/// Sign-in method on screen 14.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignIn {
    /// "Пароль".
    Password,
    /// "Пин-код".
    Pin,
    /// "Автоматический вход".
    Automatic,
}

/// A disk the user can pick on screen 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disk {
    /// Device path, e.g. /dev/nvme0n1.
    pub device: String,
    /// Label shown, e.g. "Liquid SSD".
    pub label: String,
    /// Size, bytes.
    pub size: u64,
    /// Free space, bytes.
    pub free: u64,
}

/// Minimum disk size for Liquid OS (2 system slots + data), bytes.
pub const MIN_DISK: u64 = 32 * 1024 * 1024 * 1024;

/// Everything the user chose.
#[derive(Debug, Clone, PartialEq)]
pub struct Choices {
    /// Locale id.
    pub language: &'static str,
    /// XKB layout id.
    pub layout: &'static str,
    /// License accepted (checkbox on screen 4).
    pub license_accepted: bool,
    /// Target disk.
    pub disk: Option<Disk>,
    /// Optional components.
    pub components: Components,
    /// User display name.
    pub name: String,
    /// Accent color.
    pub accent: Accent,
    /// Wallpaper index 0..6 or custom path.
    pub wallpaper: usize,
    /// Theme.
    pub theme: Theme,
    /// Sign-in method.
    pub sign_in: SignIn,
}

impl Default for Choices {
    fn default() -> Self {
        Self {
            language: "ru_RU",
            layout: "ru",
            license_accepted: false,
            disk: None,
            components: Components {
                office: true,
                extras: false,
                developer: false,
            },
            name: String::new(),
            accent: Accent::Blue,
            wallpaper: 0,
            theme: Theme::Light,
            sign_in: SignIn::Password,
        }
    }
}

/// Why "Продолжить" is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocked {
    /// License checkbox not ticked.
    LicenseNotAccepted,
    /// No disk selected.
    NoDisk,
    /// Disk smaller than `MIN_DISK`.
    DiskTooSmall,
    /// Name is empty.
    EmptyName,
    /// Installation still running.
    StillInstalling,
}

/// The flow.
#[derive(Debug, Clone, PartialEq)]
pub struct Flow {
    /// Current screen.
    pub screen: Screen,
    /// Choices so far.
    pub choices: Choices,
    /// Install progress 0..=100 (screen 7).
    pub progress: u8,
}

impl Default for Flow {
    fn default() -> Self {
        Self {
            screen: Screen::Welcome,
            choices: Choices::default(),
            progress: 0,
        }
    }
}

impl Flow {
    /// Start first-run setup on an installed system (screens 9-15).
    pub fn first_run() -> Self {
        Self {
            screen: Screen::Hello,
            ..Self::default()
        }
    }

    fn index(&self) -> usize {
        ORDER.iter().position(|s| *s == self.screen).unwrap_or(0)
    }

    /// Can the user go forward from the current screen?
    pub fn can_continue(&self) -> Result<(), Blocked> {
        let c = &self.choices;
        match self.screen {
            Screen::License if !c.license_accepted => Err(Blocked::LicenseNotAccepted),
            Screen::Disk => match &c.disk {
                None => Err(Blocked::NoDisk),
                Some(d) if d.size < MIN_DISK => Err(Blocked::DiskTooSmall),
                Some(_) => Ok(()),
            },
            Screen::Installing if self.progress < 100 => Err(Blocked::StillInstalling),
            Screen::Name if c.name.trim().is_empty() => Err(Blocked::EmptyName),
            _ => Ok(()),
        }
    }

    /// "Продолжить →".
    pub fn forward(&mut self) -> Result<Screen, Blocked> {
        self.can_continue()?;
        let i = (self.index() + 1).min(ORDER.len() - 1);
        self.screen = ORDER[i];
        Ok(self.screen)
    }

    /// "Назад". Not available during or after installation, nor
    /// before the first first-run screen.
    pub fn back(&mut self) -> Screen {
        let blocked = matches!(
            self.screen,
            Screen::Welcome | Screen::Installing | Screen::Installed | Screen::Hello
        );
        if !blocked {
            self.screen = ORDER[self.index() - 1];
        }
        self.screen
    }

    /// Stepper on the first screen of the design: 1 Выбор языка, 2 Установка, 3 Готово.
    pub fn stepper(&self) -> usize {
        match self.screen {
            Screen::Welcome | Screen::Language | Screen::Keyboard => 1,
            Screen::License | Screen::Disk | Screen::Components | Screen::Installing => 2,
            _ => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GB: u64 = 1024 * 1024 * 1024;

    #[test]
    fn full_happy_path() {
        let mut f = Flow::default();
        f.forward().unwrap();
        f.forward().unwrap();
        f.forward().unwrap();
        assert_eq!(f.screen, Screen::License);
        assert_eq!(f.forward(), Err(Blocked::LicenseNotAccepted));
        f.choices.license_accepted = true;
        f.forward().unwrap();
        assert_eq!(f.forward(), Err(Blocked::NoDisk));
        f.choices.disk = Some(Disk {
            device: "/dev/nvme0n1".into(),
            label: "Liquid SSD".into(),
            size: 512 * GB,
            free: 489 * GB,
        });
        f.forward().unwrap();
        f.forward().unwrap();
        assert_eq!(f.screen, Screen::Installing);
        assert_eq!(f.forward(), Err(Blocked::StillInstalling));
        f.progress = 100;
        f.forward().unwrap();
        f.forward().unwrap();
        f.forward().unwrap();
        assert_eq!(f.forward(), Err(Blocked::EmptyName));
        f.choices.name = "Linki".into();
        for _ in 0..5 {
            f.forward().unwrap();
        }
        assert_eq!(f.screen, Screen::Done);
    }

    #[test]
    fn small_disk_rejected() {
        let mut f = Flow {
            screen: Screen::Disk,
            ..Flow::default()
        };
        f.choices.disk = Some(Disk {
            device: "/dev/sda".into(),
            label: "USB".into(),
            size: 16 * GB,
            free: 16 * GB,
        });
        assert_eq!(f.forward(), Err(Blocked::DiskTooSmall));
    }

    #[test]
    fn no_back_during_install() {
        let mut f = Flow {
            screen: Screen::Installing,
            ..Flow::default()
        };
        assert_eq!(f.back(), Screen::Installing);
    }

    #[test]
    fn fifteen_screens() {
        assert_eq!(ORDER.len(), 15);
    }
}
