//! Liquid Compositor core: everything that does not depend on the Wayland
//! backend. The backend (Smithay) consumes these modules.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod decorations;
pub mod effects;
pub mod shell_layout;

/// GLSL sources of the Liquid Glass render pass.
pub mod shaders {
    /// Dual Kawase blur, downsample pass.
    pub const BLUR_DOWN: &str = include_str!("../shaders/blur_down.frag");
    /// Dual Kawase blur, upsample pass.
    pub const BLUR_UP: &str = include_str!("../shaders/blur_up.frag");
    /// Liquid Glass composite: tint, saturation, rim, iridescent edge, specular, refraction.
    pub const GLASS: &str = include_str!("../shaders/glass.frag");
}
