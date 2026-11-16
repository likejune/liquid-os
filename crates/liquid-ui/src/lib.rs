//! Liquid UI: component styles and geometry derived from `liquid-design`.
//!
//! This crate does not depend on a UI toolkit. It describes *what* every
//! component looks like in each state (fills, rims, radii, sizes); toolkit
//! adapters only translate these descriptions into draw calls.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod components;
pub mod geometry;

pub use components::*;
pub use geometry::*;
