//! Keybinds for Titanium

titaniumutils::allow!(
    target_os = "windows",
    "This module is only available on Windows"
);

#[doc(hidden)]
mod common;
#[doc(hidden)]
mod public;
pub use crate::public::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use crate::windows::*;