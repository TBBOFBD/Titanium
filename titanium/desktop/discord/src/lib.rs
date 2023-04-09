//! Discord Rich Presence integration for Titanium.

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

pub use discord_presence::{
    Client as DiscordClient,
    Result as DiscordResult,
    Event as DiscordEvent,
    DiscordError
};

/// Discord Rich Presence models.
pub mod models {
    pub use discord_presence::models::*;
}