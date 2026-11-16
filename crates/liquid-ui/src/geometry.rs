//! Shape helpers shared by the compositor and UI: rounded rectangles,
//! continuous-curvature squircles, signed distance functions.

/// Axis-aligned rectangle in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Rect {
    /// New rectangle.
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Center point.
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    /// Corner radius clamped so it never exceeds half the shorter side.
    /// Use with `radius::PILL` to get a true capsule.
    pub fn clamp_radius(&self, radius: f32) -> f32 {
        radius.min(self.w.min(self.h) / 2.0).max(0.0)
    }

    /// Rectangle shrunk by `d` on every side.
    pub fn inset(&self, d: f32) -> Self {
        Self::new(
            self.x + d,
            self.y + d,
            (self.w - 2.0 * d).max(0.0),
            (self.h - 2.0 * d).max(0.0),
        )
    }

    /// Whether a point is inside the rectangle.
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }
}

/// Signed distance from point `(px, py)` to a rounded rectangle
/// (negative inside, 0 on the edge). Same formula as in `glass.frag`.
pub fn sdf_rounded_rect(rect: &Rect, radius: f32, px: f32, py: f32) -> f32 {
    let r = rect.clamp_radius(radius);
    let (cx, cy) = rect.center();
    let qx = (px - cx).abs() - (rect.w / 2.0 - r);
    let qy = (py - cy).abs() - (rect.h / 2.0 - r);
    let ox = qx.max(0.0);
    let oy = qy.max(0.0);
    (ox * ox + oy * oy).sqrt() + qx.max(qy).min(0.0) - r
}

/// Points of a continuous-curvature squircle (superellipse, n = 5) used for
/// app icons, `segments` points, centered in a `size` x `size` box.
pub fn squircle_points(size: f32, segments: usize) -> Vec<(f32, f32)> {
    const N: f32 = 5.0;
    let a = size / 2.0;
    (0..segments)
        .map(|i| {
            let t = i as f32 / segments as f32 * std::f32::consts::TAU;
            let (s, c) = t.sin_cos();
            let x = a * c.signum() * c.abs().powf(2.0 / N);
            let y = a * s.signum() * s.abs().powf(2.0 / N);
            (a + x, a + y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdf_signs() {
        let r = Rect::new(0.0, 0.0, 100.0, 50.0);
        assert!(sdf_rounded_rect(&r, 12.0, 50.0, 25.0) < 0.0);
        assert!(sdf_rounded_rect(&r, 12.0, 200.0, 25.0) > 0.0);
        assert!(sdf_rounded_rect(&r, 12.0, 1.0, 1.0) > 0.0, "corner is cut");
    }

    #[test]
    fn pill_radius_clamps() {
        let r = Rect::new(0.0, 0.0, 200.0, 40.0);
        assert_eq!(r.clamp_radius(999.0), 20.0);
    }

    #[test]
    fn squircle_fits_box() {
        for (x, y) in squircle_points(40.0, 64) {
            assert!((0.0..=40.0).contains(&x) && (0.0..=40.0).contains(&y));
        }
    }
}
