//! sRGB color type used by all tokens.

/// A color with components in 0.0..=1.0 (sRGB, straight alpha).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}

impl Rgba {
    /// Opaque color from 8-bit channels.
    pub const fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::rgba8(r, g, b, 255)
    }

    /// Color from 8-bit channels including alpha.
    pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Same color with a new alpha (0.0..=1.0).
    pub const fn with_alpha(self, a: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }

    /// Parse `#RRGGBB` or `#RRGGBBAA`.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let h = hex.strip_prefix('#')?;
        let byte = |i: usize| u8::from_str_radix(h.get(i..i + 2)?, 16).ok();
        match h.len() {
            6 => Some(Self::rgb8(byte(0)?, byte(2)?, byte(4)?)),
            8 => Some(Self::rgba8(byte(0)?, byte(2)?, byte(4)?, byte(6)?)),
            _ => None,
        }
    }

    /// Format as `#RRGGBB` (opaque) or `#RRGGBBAA`.
    pub fn to_hex(self) -> String {
        let c = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        if c(self.a) == 255 {
            format!("#{:02X}{:02X}{:02X}", c(self.r), c(self.g), c(self.b))
        } else {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                c(self.r),
                c(self.g),
                c(self.b),
                c(self.a)
            )
        }
    }

    /// Linear interpolation between two colors, `t` in 0.0..=1.0.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let m = |a: f32, b: f32| a + (b - a) * t;
        Self {
            r: m(self.r, other.r),
            g: m(self.g, other.g),
            b: m(self.b, other.b),
            a: m(self.a, other.a),
        }
    }
}
