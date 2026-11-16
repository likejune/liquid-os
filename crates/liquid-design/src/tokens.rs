//! Design tokens extracted from the Liquid OS design files.
//!
//! Values marked `approx` were measured from raster mockups and must be
//! confirmed against the source design files when they are available.

use crate::Rgba;

/// Light or dark appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Pastel ice-white theme (all original mockups).
    #[default]
    Light,
    /// Deep navy glass theme (mockup "тёмная тема и пункт управления").
    Dark,
}

/// Semantic colors of one theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Desktop base behind wallpaper.
    pub background: Rgba,
    /// Tinted base for secondary surfaces.
    pub surface_tint: Rgba,
    /// Regular glass fill (panels, launcher, control center).
    pub glass: Rgba,
    /// Strong glass fill (content panes, dialogs).
    pub glass_strong: Rgba,
    /// Selected row / active pill background.
    pub selection: Rgba,
    /// 1px rim on every glass surface.
    pub rim: Rgba,
    /// Primary text and glyphs.
    pub text_primary: Rgba,
    /// Secondary text, timestamps, metadata.
    pub text_secondary: Rgba,
    /// Hairline separators.
    pub separator: Rgba,
    /// Shadow tint.
    pub shadow: Rgba,
}

/// Light palette (approx from 1000073174, 1000073172, 1000073176, 1000073177).
pub const LIGHT: Palette = Palette {
    background: Rgba::rgb8(0xF4, 0xF6, 0xFC),
    surface_tint: Rgba::rgb8(0xE8, 0xED, 0xFB),
    glass: Rgba::rgba8(0xFF, 0xFF, 0xFF, 166),
    glass_strong: Rgba::rgba8(0xFF, 0xFF, 0xFF, 217),
    selection: Rgba::rgb8(0xED, 0xF2, 0xFE),
    rim: Rgba::rgba8(0xFF, 0xFF, 0xFF, 204),
    text_primary: Rgba::rgb8(0x1E, 0x29, 0x3B),
    text_secondary: Rgba::rgb8(0x64, 0x74, 0x8B),
    separator: Rgba::rgba8(0x1E, 0x29, 0x3B, 20),
    shadow: Rgba::rgba8(100, 120, 180, 51),
};

/// Dark palette (from the generated dark theme mockup).
pub const DARK: Palette = Palette {
    background: Rgba::rgb8(0x0F, 0x12, 0x26),
    surface_tint: Rgba::rgb8(0x1A, 0x1F, 0x3A),
    glass: Rgba::rgba8(0x1A, 0x1F, 0x3A, 166),
    glass_strong: Rgba::rgba8(0x1A, 0x1F, 0x3A, 217),
    selection: Rgba::rgba8(0x3B, 0x82, 0xF6, 51),
    rim: Rgba::rgba8(0xFF, 0xFF, 0xFF, 46),
    text_primary: Rgba::rgb8(0xFF, 0xFF, 0xFF),
    text_secondary: Rgba::rgb8(0x94, 0xA3, 0xB8),
    separator: Rgba::rgba8(0xFF, 0xFF, 0xFF, 20),
    shadow: Rgba::rgba8(0, 0, 0, 115),
};

/// Palette for a theme.
pub const fn palette(theme: Theme) -> &'static Palette {
    match theme {
        Theme::Light => &LIGHT,
        Theme::Dark => &DARK,
    }
}

/// The six accent colors of the first-run screen "Выбери акцентный цвет".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Accent {
    /// Default.
    #[default]
    Blue,
    /// Purple.
    Purple,
    /// Pink.
    Pink,
    /// Coral (approx).
    Coral,
    /// Green (approx).
    Green,
    /// Gray (approx).
    Gray,
}

impl Accent {
    /// All accents in the order shown in the design.
    pub const ALL: [Accent; 6] = [
        Accent::Blue,
        Accent::Purple,
        Accent::Pink,
        Accent::Coral,
        Accent::Green,
        Accent::Gray,
    ];

    /// Accent color.
    pub const fn color(self) -> Rgba {
        match self {
            Accent::Blue => Rgba::rgb8(0x3B, 0x82, 0xF6),
            Accent::Purple => Rgba::rgb8(0xA8, 0x55, 0xF7),
            Accent::Pink => Rgba::rgb8(0xEC, 0x48, 0x99),
            Accent::Coral => Rgba::rgb8(0xFF, 0x7A, 0x6B),
            Accent::Green => Rgba::rgb8(0x34, 0xC7, 0x7B),
            Accent::Gray => Rgba::rgb8(0x8E, 0x94, 0xA8),
        }
    }
}

/// Iridescent gradient used on rims, sliders, primary buttons and the logo.
pub const IRIDESCENT: [Rgba; 4] = [
    Rgba::rgb8(0x48, 0xCA, 0xE4),
    Rgba::rgb8(0x3B, 0x82, 0xF6),
    Rgba::rgb8(0xA8, 0x55, 0xF7),
    Rgba::rgb8(0xEC, 0x48, 0x99),
];

/// Window control buttons (top-left traffic lights).
pub mod window_controls {
    use crate::Rgba;
    /// Close.
    pub const CLOSE: Rgba = Rgba::rgb8(0xFF, 0x5F, 0x57);
    /// Minimize.
    pub const MINIMIZE: Rgba = Rgba::rgb8(0xFE, 0xBC, 0x2E);
    /// Maximize.
    pub const MAXIMIZE: Rgba = Rgba::rgb8(0x28, 0xC8, 0x40);
    /// Button diameter, px.
    pub const DIAMETER: f32 = 12.0;
    /// Gap between buttons, px.
    pub const GAP: f32 = 8.0;
}

/// Corner radii, px.
pub mod radius {
    /// Application windows and dialogs.
    pub const WINDOW: f32 = 24.0;
    /// Large floating panels (control center, launcher).
    pub const PANEL: f32 = 30.0;
    /// Cards inside panels.
    pub const CARD: f32 = 18.0;
    /// Notifications.
    pub const NOTIFICATION: f32 = 16.0;
    /// Inputs, list selection, small buttons.
    pub const CONTROL: f32 = 12.0;
    /// Chips and icon badges.
    pub const CHIP: f32 = 10.0;
    /// Dock capsule.
    pub const DOCK: f32 = 28.0;
    /// Fully rounded pill (buttons, search, toggles, sliders).
    pub const PILL: f32 = 999.0;
    /// App icon squircle corner, as a fraction of icon size.
    pub const ICON_FRACTION: f32 = 0.22;
}

/// Spacing scale, px.
pub mod spacing {
    /// 4px.
    pub const XS: f32 = 4.0;
    /// 8px.
    pub const S: f32 = 8.0;
    /// 12px, gap between rows and cards.
    pub const M: f32 = 12.0;
    /// 16px, inner padding of cards.
    pub const L: f32 = 16.0;
    /// 24px, panel padding.
    pub const XL: f32 = 24.0;
    /// 32px, outer margins.
    pub const XXL: f32 = 32.0;
}

/// Icon system (1000073173).
pub mod icon {
    /// Base grid, px.
    pub const GRID: f32 = 40.0;
    /// Dock icon size, px (approx).
    pub const DOCK: f32 = 56.0;
    /// Launcher grid icon size, px (approx).
    pub const LAUNCHER: f32 = 64.0;
    /// Direction of the single light source: top-left, normalized (x, y).
    pub const LIGHT_DIRECTION: (f32, f32) = (
        -std::f32::consts::FRAC_1_SQRT_2,
        -std::f32::consts::FRAC_1_SQRT_2,
    );
}

/// Parameters of the Liquid Glass material.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glass {
    /// Gaussian blur radius of the backdrop, px.
    pub blur_radius: f32,
    /// Backdrop saturation boost (1.0 = none).
    pub saturation: f32,
    /// Inner rim width, px.
    pub rim_width: f32,
    /// Opacity of the iridescent outer edge.
    pub iridescent_edge: f32,
    /// Strength of refraction distortion near edges, px.
    pub refraction: f32,
    /// Opacity of the top-left specular highlight.
    pub specular: f32,
}

/// Glass presets per surface (approx; tune in compositor against mockups).
pub mod glass {
    use super::Glass;
    /// Top menu bar.
    pub const MENU_BAR: Glass = Glass {
        blur_radius: 30.0,
        saturation: 1.4,
        rim_width: 1.0,
        iridescent_edge: 0.0,
        refraction: 0.0,
        specular: 0.2,
    };
    /// Windows and dialogs.
    pub const WINDOW: Glass = Glass {
        blur_radius: 40.0,
        saturation: 1.5,
        rim_width: 1.0,
        iridescent_edge: 0.25,
        refraction: 2.0,
        specular: 0.35,
    };
    /// Control center, launcher, notifications.
    pub const PANEL: Glass = Glass {
        blur_radius: 50.0,
        saturation: 1.6,
        rim_width: 1.0,
        iridescent_edge: 0.45,
        refraction: 4.0,
        specular: 0.45,
    };
    /// Dock capsule.
    pub const DOCK: Glass = Glass {
        blur_radius: 40.0,
        saturation: 1.6,
        rim_width: 1.0,
        iridescent_edge: 0.5,
        refraction: 6.0,
        specular: 0.5,
    };
}

/// One layer of a drop shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    /// Vertical offset, px.
    pub offset_y: f32,
    /// Blur radius, px.
    pub blur: f32,
    /// Opacity multiplier applied to `Palette::shadow`.
    pub opacity: f32,
}

/// Layered shadows: soft, diffuse, multi-layer.
pub mod shadow {
    use super::Shadow;
    /// Floating windows and dialogs.
    pub const WINDOW: [Shadow; 2] = [
        Shadow {
            offset_y: 4.0,
            blur: 12.0,
            opacity: 0.5,
        },
        Shadow {
            offset_y: 24.0,
            blur: 64.0,
            opacity: 1.0,
        },
    ];
    /// Panels, dock, notifications.
    pub const PANEL: [Shadow; 2] = [
        Shadow {
            offset_y: 2.0,
            blur: 8.0,
            opacity: 0.4,
        },
        Shadow {
            offset_y: 16.0,
            blur: 48.0,
            opacity: 0.8,
        },
    ];
}

/// Typography (Inter replaces SF Pro, decision 15.11.2026).
pub mod typography {
    /// Font family.
    pub const FAMILY: &str = "Inter";
    /// Hero title ("Liquid OS" on setup and About screens).
    pub const DISPLAY: f32 = 40.0;
    /// Window and screen titles.
    pub const TITLE: f32 = 24.0;
    /// Section headers.
    pub const HEADLINE: f32 = 17.0;
    /// Body text and list rows.
    pub const BODY: f32 = 14.0;
    /// Menu bar items.
    pub const MENU: f32 = 13.0;
    /// Captions, timestamps, badges.
    pub const CAPTION: f32 = 12.0;
    /// Weights.
    pub const REGULAR: u16 = 400;
    /// Medium.
    pub const MEDIUM: u16 = 500;
    /// Semibold.
    pub const SEMIBOLD: u16 = 600;
    /// Bold.
    pub const BOLD: u16 = 700;
}

/// Motion. The mockups are static, so durations are initial values to tune.
pub mod motion {
    /// Hover, press, toggle, ms.
    pub const FAST: u32 = 120;
    /// Panels opening, list changes, ms.
    pub const NORMAL: u32 = 220;
    /// Window open/minimize to dock, ms.
    pub const SLOW: u32 = 360;
    /// Spring stiffness for liquid animations.
    pub const SPRING_STIFFNESS: f32 = 300.0;
    /// Spring damping ratio.
    pub const SPRING_DAMPING: f32 = 0.82;
}

/// Layout sizes of shell elements, px (approx from 1920x1080 mockups).
pub mod layout {
    /// Top menu bar height.
    pub const MENU_BAR_HEIGHT: f32 = 32.0;
    /// Dock height.
    pub const DOCK_HEIGHT: f32 = 76.0;
    /// Gap between dock and screen bottom.
    pub const DOCK_MARGIN: f32 = 12.0;
    /// Number of apps pinned in the dock.
    pub const DOCK_APPS: usize = 3;
    /// Launcher grid columns.
    pub const LAUNCHER_COLUMNS: usize = 4;
    /// Notification card width.
    pub const NOTIFICATION_WIDTH: f32 = 360.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        assert_eq!(LIGHT.text_primary.to_hex(), "#1E293B");
        assert_eq!(Rgba::from_hex("#3B82F6"), Some(Accent::Blue.color()));
        assert_eq!(Rgba::from_hex("#3B82F6").unwrap().to_hex(), "#3B82F6");
        assert!(Rgba::from_hex("3B82F6").is_none());
    }

    #[test]
    fn six_accents() {
        assert_eq!(Accent::ALL.len(), 6);
        assert_eq!(Accent::default(), Accent::Blue);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn radii_follow_design_hierarchy() {
        assert!(radius::PANEL > radius::WINDOW);
        assert!(radius::WINDOW > radius::CARD);
        assert!(radius::CARD > radius::CONTROL);
    }

    #[test]
    fn dock_has_three_apps() {
        assert_eq!(layout::DOCK_APPS, 3);
    }

    #[test]
    fn themes_resolve() {
        assert_eq!(palette(Theme::Light).background.to_hex(), "#F4F6FC");
        assert_eq!(palette(Theme::Dark).text_primary.to_hex(), "#FFFFFF");
    }
}
