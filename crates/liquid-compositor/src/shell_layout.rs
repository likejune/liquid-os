//! Geometry of shell surfaces on an output, matching the 1920x1080 mockups.

use liquid_design::{icon, layout, spacing};
use liquid_ui::Rect;

/// Logical size of an output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Output {
    /// Width, logical px.
    pub width: f32,
    /// Height, logical px.
    pub height: f32,
}

/// Top menu bar: full width.
pub fn menu_bar(o: &Output) -> Rect {
    Rect::new(0.0, 0.0, o.width, layout::MENU_BAR_HEIGHT)
}

/// Floating dock capsule, centered at the bottom.
pub fn dock(o: &Output, apps: usize) -> Rect {
    let n = apps.max(1) as f32;
    let w = n * icon::DOCK + (n - 1.0) * spacing::M + 2.0 * spacing::L;
    Rect::new(
        (o.width - w) / 2.0,
        o.height - layout::DOCK_MARGIN - layout::DOCK_HEIGHT,
        w,
        layout::DOCK_HEIGHT,
    )
}

/// Icon slot `index` inside the dock.
pub fn dock_icon(o: &Output, apps: usize, index: usize) -> Rect {
    let d = dock(o, apps);
    let x = d.x + spacing::L + index as f32 * (icon::DOCK + spacing::M);
    let y = d.y + (d.h - icon::DOCK) / 2.0;
    Rect::new(x, y, icon::DOCK, icon::DOCK)
}

/// Area for windows: between the menu bar and the top of the dock.
pub fn work_area(o: &Output) -> Rect {
    let top = layout::MENU_BAR_HEIGHT;
    let bottom = dock(o, layout::DOCK_APPS).y - spacing::M;
    Rect::new(0.0, top, o.width, (bottom - top).max(0.0))
}

fn centered_in(area: Rect, w: f32, h: f32) -> Rect {
    let w = w.min(area.w - 2.0 * spacing::XXL).max(0.0);
    let h = h.min(area.h - 2.0 * spacing::XL).max(0.0);
    Rect::new(
        area.x + (area.w - w) / 2.0,
        area.y + (area.h - h) / 2.0,
        w,
        h,
    )
}

/// App launcher ("Меню"): centered floating panel (1000073175).
pub fn launcher(o: &Output) -> Rect {
    centered_in(work_area(o), 720.0, 600.0)
}

/// Control center: centered floating panel under the menu bar (1000073174).
pub fn control_center(o: &Output) -> Rect {
    let area = work_area(o);
    let w = 760.0_f32.min(area.w - 2.0 * spacing::XXL);
    let h = 440.0_f32.min(area.h - spacing::XL);
    Rect::new(area.x + (area.w - w) / 2.0, area.y + spacing::XL, w, h)
}

/// Notification toast `index` (0 = newest), stacked top-right (1000073171).
pub fn notification(o: &Output, index: usize) -> Rect {
    const H: f32 = 72.0;
    let x = o.width - layout::NOTIFICATION_WIDTH - spacing::L;
    let y = layout::MENU_BAR_HEIGHT + spacing::M + index as f32 * (H + spacing::S);
    Rect::new(x, y, layout::NOTIFICATION_WIDTH, H)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FHD: Output = Output {
        width: 1920.0,
        height: 1080.0,
    };

    #[test]
    fn dock_is_centered_and_floating() {
        let d = dock(&FHD, 3);
        assert!((d.x + d.w / 2.0 - 960.0).abs() < 0.01);
        assert!(d.y + d.h < 1080.0, "dock does not touch the edge");
    }

    #[test]
    fn dock_icons_inside_dock() {
        let d = dock(&FHD, 3);
        for i in 0..3 {
            let r = dock_icon(&FHD, 3, i);
            assert!(r.x >= d.x && r.x + r.w <= d.x + d.w);
        }
    }

    #[test]
    fn panels_inside_work_area() {
        let a = work_area(&FHD);
        for r in [launcher(&FHD), control_center(&FHD)] {
            assert!(r.y >= a.y && r.y + r.h <= a.y + a.h);
        }
    }

    #[test]
    fn notifications_stack_down() {
        assert!(notification(&FHD, 1).y > notification(&FHD, 0).y);
        assert!(notification(&FHD, 0).x + notification(&FHD, 0).w <= 1920.0);
    }
}
