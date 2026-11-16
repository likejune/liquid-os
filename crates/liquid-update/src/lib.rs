//! Update center: state machine and texts of the "Обновление ПО" window.
//! The work is done by systemd-sysupdate (A/B) and flatpak.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::process::Command;

/// Update center state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// Nothing to do.
    UpToDate {
        /// Installed version.
        version: String,
    },
    /// Checking for updates.
    Checking,
    /// "Доступно обновление".
    Available {
        /// New version.
        version: String,
        /// Download size, bytes (if known).
        size: Option<u64>,
    },
    /// Downloading into the inactive slot.
    Installing {
        /// New version.
        version: String,
    },
    /// Written to slot B, restart needed.
    ReadyToRestart {
        /// New version.
        version: String,
    },
    /// Something failed; the running system is untouched.
    Failed(String),
}

impl State {
    /// Title line of the window.
    pub fn title(&self) -> String {
        match self {
            State::UpToDate { .. } => "Система обновлена".into(),
            State::Checking => "Поиск обновлений…".into(),
            State::Available { .. } => "Доступно обновление".into(),
            State::Installing { .. } => "Установка обновления…".into(),
            State::ReadyToRestart { .. } => "Перезагрузите компьютер".into(),
            State::Failed(_) => "Не удалось обновить".into(),
        }
    }

    /// Subtitle: "Liquid OS 1.0.1 • 842 МБ".
    pub fn subtitle(&self) -> String {
        match self {
            State::UpToDate { version } => format!("Liquid OS {version}"),
            State::Available {
                version,
                size: Some(s),
            } => format!("Liquid OS {version} • {}", liquid_format::size(*s)),
            State::Available {
                version,
                size: None,
            }
            | State::Installing { version }
            | State::ReadyToRestart { version } => format!("Liquid OS {version}"),
            State::Checking => String::new(),
            State::Failed(e) => e.clone(),
        }
    }
}

/// Rollback note shown under the buttons.
pub const ROLLBACK_NOTE: &str = "При ошибке система вернётся к прошлой версии";

/// Parse the output of `systemd-sysupdate check-new`: it prints the new
/// version on stdout when one exists and nothing otherwise.
pub fn parse_check_new(stdout: &str) -> Option<String> {
    stdout
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(str::to_string)
}

/// Run `systemd-sysupdate check-new`.
pub fn check(current: &str) -> State {
    match Command::new("systemd-sysupdate").arg("check-new").output() {
        Ok(o) if o.status.success() => match parse_check_new(&String::from_utf8_lossy(&o.stdout)) {
            Some(v) if v != current => State::Available {
                version: v,
                size: None,
            },
            _ => State::UpToDate {
                version: current.to_string(),
            },
        },
        Ok(o) => State::Failed(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        Err(e) => State::Failed(e.to_string()),
    }
}

/// Run `systemd-sysupdate update` (writes the inactive slot) and update Flatpak apps.
pub fn install(version: &str) -> State {
    let sys = Command::new("systemd-sysupdate").arg("update").status();
    match sys {
        Ok(s) if s.success() => {
            let _ = Command::new("flatpak")
                .args(["update", "--system", "--noninteractive"])
                .status();
            State::ReadyToRestart {
                version: version.to_string(),
            }
        }
        Ok(s) => State::Failed(format!("systemd-sysupdate: код {}", s.code().unwrap_or(-1))),
        Err(e) => State::Failed(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_match_mockup() {
        let s = State::Available {
            version: "1.0.1".into(),
            size: Some(842 * 1024 * 1024),
        };
        assert_eq!(s.title(), "Доступно обновление");
        assert_eq!(s.subtitle(), "Liquid OS 1.0.1 • 842 МБ");
    }

    #[test]
    fn check_new_parsing() {
        assert_eq!(parse_check_new("\n0.5.0\n").as_deref(), Some("0.5.0"));
        assert_eq!(parse_check_new(""), None);
    }
}
