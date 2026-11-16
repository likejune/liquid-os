//! Installer (screens 1-8) and first-run setup (screens 9-15) as a state
//! machine. The UI renders `Screen`; actions come from buttons.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod flow;
pub mod plan;

pub use flow::*;
