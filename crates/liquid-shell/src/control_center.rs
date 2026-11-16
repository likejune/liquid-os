//! Control center (1000073174). AirDrop removed by decision 15.11.2026:
//! three connectivity tiles only.

/// Connectivity tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// Wi-Fi.
    WiFi,
    /// Bluetooth.
    Bluetooth,
    /// Focus / Do not disturb modes.
    Focus,
}

impl Tile {
    /// Tiles in display order.
    pub const ALL: [Tile; 3] = [Tile::WiFi, Tile::Bluetooth, Tile::Focus];

    /// Russian label.
    pub const fn label(self) -> &'static str {
        match self {
            Tile::WiFi => "Wi-Fi",
            Tile::Bluetooth => "Bluetooth",
            Tile::Focus => "Фокусирование",
        }
    }
}

/// Media playing now.
#[derive(Debug, Clone, PartialEq)]
pub struct NowPlaying {
    /// Track title.
    pub title: String,
    /// Artist / app.
    pub subtitle: String,
    /// Position, seconds.
    pub position: u32,
    /// Length, seconds.
    pub length: u32,
    /// Playing or paused.
    pub playing: bool,
}

/// Format seconds as m:ss ("1:42").
pub fn format_time(s: u32) -> String {
    format!("{}:{:02}", s / 60, s % 60)
}

/// Full state of the control center.
#[derive(Debug, Clone, PartialEq)]
pub struct ControlCenter {
    /// Connected network name, None when disconnected.
    pub wifi: Option<String>,
    /// Bluetooth on.
    pub bluetooth: bool,
    /// Do not disturb on.
    pub do_not_disturb: bool,
    /// Dark theme on.
    pub dark_mode: bool,
    /// Night light on.
    pub night_light: bool,
    /// Brightness 0..=100.
    pub brightness: u8,
    /// Volume 0..=100.
    pub volume: u8,
    /// Battery percent, None on desktops.
    pub battery: Option<u8>,
    /// Media player card.
    pub now_playing: Option<NowPlaying>,
}

impl Default for ControlCenter {
    fn default() -> Self {
        Self {
            wifi: None,
            bluetooth: true,
            do_not_disturb: false,
            dark_mode: false,
            night_light: false,
            brightness: 70,
            volume: 50,
            battery: None,
            now_playing: None,
        }
    }
}

impl ControlCenter {
    /// Subtitle under a tile ("Home_5G", "Вкл.", "Не беспокоить").
    pub fn tile_status(&self, t: Tile) -> String {
        match t {
            Tile::WiFi => self.wifi.clone().unwrap_or_else(|| "Выкл.".into()),
            Tile::Bluetooth => on_off(self.bluetooth).into(),
            Tile::Focus => {
                if self.do_not_disturb {
                    "Не беспокоить".into()
                } else {
                    "Выкл.".into()
                }
            }
        }
    }

    /// Set brightness, clamped to 0..=100.
    pub fn set_brightness(&mut self, v: i32) {
        self.brightness = v.clamp(0, 100) as u8;
    }

    /// Set volume, clamped to 0..=100.
    pub fn set_volume(&mut self, v: i32) {
        self.volume = v.clamp(0, 100) as u8;
    }
}

/// "Вкл." / "Выкл.".
pub const fn on_off(v: bool) -> &'static str {
    if v {
        "Вкл."
    } else {
        "Выкл."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_airdrop() {
        assert_eq!(Tile::ALL.len(), 3);
        assert!(Tile::ALL.iter().all(|t| t.label() != "AirDrop"));
    }

    #[test]
    fn statuses() {
        let mut c = ControlCenter {
            wifi: Some("Home_5G".into()),
            ..Default::default()
        };
        assert_eq!(c.tile_status(Tile::WiFi), "Home_5G");
        assert_eq!(c.tile_status(Tile::Bluetooth), "Вкл.");
        c.set_volume(150);
        assert_eq!(c.volume, 100);
        assert_eq!(format_time(102), "1:42");
        assert_eq!(format_time(207), "3:27");
    }
}
