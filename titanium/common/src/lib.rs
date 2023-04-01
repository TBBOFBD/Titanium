//! Common utilities for the Titanium project.

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

pub mod math;

#[doc(hidden)]
pub mod __internals__ {
    /// Default App Name
    pub const DEFAULT_APP_NAME: &str = concat!(
        "Titanium v",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
}