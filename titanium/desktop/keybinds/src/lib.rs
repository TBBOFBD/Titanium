//! Keybinds for Titanium

#[doc(hidden)]
mod common;
#[doc(hidden)]
mod public;
pub use crate::public::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use crate::windows::*;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use crate::linux::*;