//! Server-side window decorations: the three window buttons (1000073172).

use liquid_design::{spacing, window_controls as wc};
use liquid_ui::Rect;

/// Window button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowButton {
    /// Red.
    Close,
    /// Yellow.
    Minimize,
    /// Green.
    Maximize,
}

impl WindowButton {
    /// Left-to-right order.
    pub const ORDER: [WindowButton; 3] = [
        WindowButton::Close,
        WindowButton::Minimize,
        WindowButton::Maximize,
    ];
}

/// Height of the title area that holds the buttons, px.
pub const TITLE_HEIGHT: f32 = 44.0;

/// Button rectangle relative to the window's top-left corner.
pub fn button_rect(button: WindowButton) -> Rect {
    let i = WindowButton::ORDER
        .iter()
        .position(|b| *b == button)
        .unwrap_or(0) as f32;
    let x = spacing::L + i * (wc::DIAMETER + wc::GAP);
    let y = (TITLE_HEIGHT - wc::DIAMETER) / 2.0;
    Rect::new(x, y, wc::DIAMETER, wc::DIAMETER)
}

/// Which button is under a point (window-relative). Hit area is padded by 4px
/// so the small dots are easy to click.
pub fn hit_test(px: f32, py: f32) -> Option<WindowButton> {
    WindowButton::ORDER
        .into_iter()
        .find(|b| button_rect(*b).inset(-4.0).contains(px, py))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buttons_left_to_right() {
        assert!(button_rect(WindowButton::Close).x < button_rect(WindowButton::Minimize).x);
        assert!(button_rect(WindowButton::Minimize).x < button_rect(WindowButton::Maximize).x);
    }

    #[test]
    fn hit_close() {
        let (cx, cy) = button_rect(WindowButton::Close).center();
        assert_eq!(hit_test(cx, cy), Some(WindowButton::Close));
        assert_eq!(hit_test(500.0, 300.0), None);
    }
}
