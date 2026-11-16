//! Uniforms for `shaders/glass.frag`, derived from design tokens.

use liquid_design::{Rgba, Theme, IRIDESCENT};
use liquid_ui::{surface, Fill, Surface};

/// Uniform values for one glass surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlassUniforms {
    /// Corner radius, px.
    pub radius: f32,
    /// Tint over the blurred backdrop (rgba, straight alpha).
    pub tint: [f32; 4],
    /// Backdrop saturation.
    pub saturation: f32,
    /// Rim width, px.
    pub rim_width: f32,
    /// Rim color.
    pub rim_color: [f32; 4],
    /// Opacity of the iridescent edge.
    pub iridescent_edge: f32,
    /// Specular highlight strength.
    pub specular: f32,
    /// Refraction distortion, px.
    pub refraction: f32,
    /// Iridescent gradient stops.
    pub iridescent: [[f32; 3]; 4],
    /// Blur radius for the blur passes, px.
    pub blur_radius: f32,
}

fn rgba(c: Rgba) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

/// Number of dual Kawase passes needed for a blur radius.
pub fn blur_passes(blur_radius: f32) -> u32 {
    match blur_radius {
        r if r <= 8.0 => 1,
        r if r <= 20.0 => 2,
        r if r <= 40.0 => 3,
        _ => 4,
    }
}

/// Uniforms for a shell surface in a theme.
pub fn glass_uniforms(theme: Theme, kind: Surface) -> GlassUniforms {
    let style = surface(theme, kind);
    let (tint, m) = match style.fill {
        Fill::Glass { tint, material } => (tint, material),
        _ => unreachable!("shell surfaces are always glass"),
    };
    GlassUniforms {
        radius: style.radius,
        tint: rgba(tint),
        saturation: m.saturation,
        rim_width: m.rim_width,
        rim_color: rgba(style.rim),
        iridescent_edge: m.iridescent_edge,
        specular: m.specular,
        refraction: m.refraction,
        iridescent: IRIDESCENT.map(|c| [c.r, c.g, c.b]),
        blur_radius: m.blur_radius,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_surface_is_glass() {
        for k in [
            Surface::MenuBar,
            Surface::Dock,
            Surface::Panel,
            Surface::Notification,
            Surface::Window,
        ] {
            let u = glass_uniforms(Theme::Light, k);
            assert_eq!(u.rim_width, 1.0);
            assert!(u.blur_radius > 0.0);
        }
    }

    #[test]
    fn passes_grow_with_radius() {
        assert!(blur_passes(50.0) > blur_passes(10.0));
    }
}
