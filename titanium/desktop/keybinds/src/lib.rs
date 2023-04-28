//! Keybinds for Titanium

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

/// Keybinds for Titanium.
/// Mostly used for creating shortcuts.
#[cfg(not(target_os = "macos"))]
pub mod keys {
    pub use inputbot::{
        KeySequence, 
        MouseCursor,
        MouseWheel,
        BlockInput,
        KeybdKey as Keyboard,
        MouseButton as Mouse,
        handle_input_events
    };
}

/// Ctrl+C and Ctrl+Break events.
/// Mostly used for handling termination events.
pub mod exit {
    pub use ctrlc::*;
}