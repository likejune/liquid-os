//! Styles of every Liquid UI component, per state, taken from the mockups.

use liquid_design::{
    glass, layout, palette, radius, spacing, typography, Accent, Glass, Rgba, Theme, IRIDESCENT,
};

/// Interaction state of a control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    /// Idle.
    #[default]
    Normal,
    /// Pointer over.
    Hovered,
    /// Pressed.
    Pressed,
    /// Disabled.
    Disabled,
}

/// How a surface is filled.
#[derive(Debug, Clone, PartialEq)]
pub enum Fill {
    /// Solid color.
    Solid(Rgba),
    /// Linear gradient, left to right.
    Gradient(Vec<Rgba>),
    /// Liquid Glass material over a translucent tint.
    Glass {
        /// Tint over the blurred backdrop.
        tint: Rgba,
        /// Material parameters.
        material: Glass,
    },
}

/// Visual description of a box-shaped component.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxStyle {
    /// Background.
    pub fill: Fill,
    /// Corner radius, px (`radius::PILL` = capsule).
    pub radius: f32,
    /// 1px rim color.
    pub rim: Rgba,
    /// Text / glyph color.
    pub foreground: Rgba,
    /// Height, px.
    pub height: f32,
    /// Horizontal padding, px.
    pub padding_x: f32,
    /// Font size, px.
    pub font_size: f32,
    /// Font weight.
    pub font_weight: u16,
}

fn dim(c: Rgba, state: State) -> Rgba {
    match state {
        State::Normal => c,
        State::Hovered => c.with_alpha((c.a * 1.08).min(1.0)),
        State::Pressed => c.with_alpha(c.a * 0.85),
        State::Disabled => c.with_alpha(c.a * 0.4),
    }
}

/// Button kinds from the mockups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// Gradient pill: "Подробнее о системе", "Установить сейчас".
    Primary,
    /// Translucent glass pill: "Отчёт системы", "Позже", "Назад".
    Secondary,
    /// Large convex glass pill of the installer: "Продолжить →".
    Hero,
}

/// Button style.
pub fn button(theme: Theme, accent: Accent, kind: ButtonKind, state: State) -> BoxStyle {
    let p = palette(theme);
    let (fill, fg, height) = match kind {
        ButtonKind::Primary => (
            Fill::Gradient(vec![dim(accent.color(), state), dim(IRIDESCENT[2], state)]),
            Rgba::rgb8(0xFF, 0xFF, 0xFF),
            40.0,
        ),
        ButtonKind::Secondary => (
            Fill::Glass {
                tint: dim(p.glass, state),
                material: glass::PANEL,
            },
            p.text_primary,
            40.0,
        ),
        ButtonKind::Hero => (
            Fill::Glass {
                tint: dim(p.glass_strong, state),
                material: glass::DOCK,
            },
            p.text_primary,
            56.0,
        ),
    };
    BoxStyle {
        fill,
        radius: radius::PILL,
        rim: p.rim,
        foreground: if state == State::Disabled {
            fg.with_alpha(0.5)
        } else {
            fg
        },
        height,
        padding_x: spacing::XL,
        font_size: typography::BODY,
        font_weight: typography::MEDIUM,
    }
}

/// Search field ("Поиск приложений", "Поиск в Finder").
pub fn search_field(theme: Theme, focused: bool) -> BoxStyle {
    let p = palette(theme);
    BoxStyle {
        fill: Fill::Glass {
            tint: p.glass,
            material: glass::PANEL,
        },
        radius: 16.0,
        rim: if focused {
            Accent::Blue.color().with_alpha(0.6)
        } else {
            p.rim
        },
        foreground: p.text_secondary,
        height: 36.0,
        padding_x: spacing::M,
        font_size: typography::BODY,
        font_weight: typography::REGULAR,
    }
}

/// Sidebar / list row (Finder, Notes, Settings sidebars).
pub fn list_row(theme: Theme, selected: bool, state: State) -> BoxStyle {
    let p = palette(theme);
    let tint = if selected {
        p.selection
    } else {
        Rgba::rgba8(0, 0, 0, 0)
    };
    BoxStyle {
        fill: Fill::Solid(dim(tint, state)),
        radius: radius::CHIP,
        rim: Rgba::rgba8(0, 0, 0, 0),
        foreground: p.text_primary,
        height: 32.0,
        padding_x: spacing::S,
        font_size: typography::BODY,
        font_weight: if selected {
            typography::MEDIUM
        } else {
            typography::REGULAR
        },
    }
}

/// Card inside a panel (control center tiles, settings groups, installer options).
pub fn card(theme: Theme, selected: bool) -> BoxStyle {
    let p = palette(theme);
    BoxStyle {
        fill: Fill::Glass {
            tint: p.glass_strong,
            material: glass::PANEL,
        },
        radius: radius::CARD,
        rim: if selected {
            Accent::Blue.color()
        } else {
            p.rim
        },
        foreground: p.text_primary,
        height: 0.0,
        padding_x: spacing::L,
        font_size: typography::BODY,
        font_weight: typography::REGULAR,
    }
}

/// Toggle switch (Тёмный режим, Ночной свет, Не беспокоить).
#[derive(Debug, Clone, PartialEq)]
pub struct ToggleStyle {
    /// Track fill.
    pub track: Fill,
    /// Knob color.
    pub knob: Rgba,
    /// Track width, px.
    pub width: f32,
    /// Track height, px.
    pub height: f32,
}

/// Toggle style.
pub fn toggle(theme: Theme, accent: Accent, on: bool) -> ToggleStyle {
    let p = palette(theme);
    ToggleStyle {
        track: if on {
            Fill::Gradient(vec![accent.color(), IRIDESCENT[2]])
        } else {
            Fill::Glass {
                tint: p.surface_tint.with_alpha(0.8),
                material: glass::PANEL,
            }
        },
        knob: Rgba::rgb8(0xFF, 0xFF, 0xFF),
        width: 44.0,
        height: 26.0,
    }
}

/// Vertical slider of the control center (Яркость, Громкость).
#[derive(Debug, Clone, PartialEq)]
pub struct SliderStyle {
    /// Empty part of the track.
    pub track: Fill,
    /// Filled part, bottom to value.
    pub fill: Fill,
    /// Thumb color.
    pub thumb: Rgba,
    /// Track width, px.
    pub width: f32,
    /// Track height, px.
    pub height: f32,
}

/// Vertical slider style.
pub fn vertical_slider(theme: Theme) -> SliderStyle {
    let p = palette(theme);
    SliderStyle {
        track: Fill::Glass {
            tint: p.glass,
            material: glass::PANEL,
        },
        fill: Fill::Gradient(vec![
            IRIDESCENT[3],
            IRIDESCENT[2],
            IRIDESCENT[1],
            IRIDESCENT[0],
        ]),
        thumb: Rgba::rgb8(0xFF, 0xFF, 0xFF),
        width: 56.0,
        height: 160.0,
    }
}

/// Installer stepper (Выбор языка / Установка / Готово).
pub fn stepper_dot(theme: Theme, accent: Accent, active: bool) -> BoxStyle {
    let p = palette(theme);
    BoxStyle {
        fill: if active {
            Fill::Solid(accent.color())
        } else {
            Fill::Solid(p.glass_strong)
        },
        radius: radius::PILL,
        rim: p.rim,
        foreground: if active {
            Rgba::rgb8(0xFF, 0xFF, 0xFF)
        } else {
            p.text_secondary
        },
        height: 32.0,
        padding_x: 0.0,
        font_size: typography::BODY,
        font_weight: typography::SEMIBOLD,
    }
}

/// Floating surfaces of the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    /// Top menu bar.
    MenuBar,
    /// Dock capsule.
    Dock,
    /// Control center, launcher.
    Panel,
    /// Notification toast.
    Notification,
    /// App window.
    Window,
}

/// Style of a shell surface.
pub fn surface(theme: Theme, kind: Surface) -> BoxStyle {
    let p = palette(theme);
    let (material, r, tint, h) = match kind {
        Surface::MenuBar => (glass::MENU_BAR, 0.0, p.glass, layout::MENU_BAR_HEIGHT),
        Surface::Dock => (glass::DOCK, radius::DOCK, p.glass, layout::DOCK_HEIGHT),
        Surface::Panel => (glass::PANEL, radius::PANEL, p.glass, 0.0),
        Surface::Notification => (glass::PANEL, radius::NOTIFICATION, p.glass_strong, 0.0),
        Surface::Window => (glass::WINDOW, radius::WINDOW, p.glass_strong, 0.0),
    };
    BoxStyle {
        fill: Fill::Glass { tint, material },
        radius: r,
        rim: p.rim,
        foreground: p.text_primary,
        height: h,
        padding_x: spacing::L,
        font_size: typography::MENU,
        font_weight: typography::REGULAR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buttons_are_pills() {
        for k in [ButtonKind::Primary, ButtonKind::Secondary, ButtonKind::Hero] {
            assert_eq!(
                button(Theme::Light, Accent::Blue, k, State::Normal).radius,
                radius::PILL
            );
        }
    }

    #[test]
    fn window_uses_design_radius() {
        assert_eq!(surface(Theme::Light, Surface::Window).radius, 24.0);
        assert_eq!(surface(Theme::Dark, Surface::Dock).radius, radius::DOCK);
    }

    #[test]
    fn toggle_on_is_gradient() {
        assert!(matches!(
            toggle(Theme::Light, Accent::Blue, true).track,
            Fill::Gradient(_)
        ));
        assert!(matches!(
            toggle(Theme::Light, Accent::Blue, false).track,
            Fill::Glass { .. }
        ));
    }
}
