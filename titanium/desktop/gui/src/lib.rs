//! This module contains the GUI implementation for the Titanium desktop application.
//! Requires the `gui` feature to be enabled.
//! Depends on GTK3

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

/// The default GUI implementation for Titanium.
pub mod default;
mod guiinstance;
mod guilauncher;

pub use guiinstance::GuiInstance;
pub use guilauncher::GuiLauncher;
pub use druid as libdruid;
