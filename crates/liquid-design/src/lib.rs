//! Liquid OS design tokens.
//!
//! Single source of truth for every visual value used by Liquid Compositor,
//! Liquid UI, Liquid Shell and system apps. Values are taken from the design
//! files (1000073166-1000073177, 1000073167_optimized.png). Never hardcode a
//! color, radius or shadow anywhere else: import it from this crate.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod color;
pub mod tokens;

pub use color::Rgba;
pub use tokens::*;
