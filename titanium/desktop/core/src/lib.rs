#[cfg(target_os = "windows")]
pub mod memory;
pub mod math;


#[doc(hidden)]
pub mod __internals__ {
    /// Default App Name
    pub const DEFAULT_APP_NAME: &str = concat!(
        "Titanium v",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
}
