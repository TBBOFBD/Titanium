//! Discord Rich Presence integration for Titanium.

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

/// The client ID of the Titanium Discord application.
pub const CLIENT_ID: &str = "1090677709350391879";

mod discord_ipc;
mod pack_unpack;
pub use discord_ipc::*;
pub mod models;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

pub use ipc::DiscordRPCClient;

/// Creates a new that is automatically connected to the Titanium Discord application.
#[inline]
#[must_use = "This function returns a new client"]
pub fn titanium() -> Result<DiscordRPCClient, Box<dyn std::error::Error>> {
    DiscordRPCClient::new(CLIENT_ID)
}

/// Creates a new activity for the Titanium Discord application.
#[inline]
#[must_use = "This function returns a new activity"]
pub fn titanium_activity<'a>() -> models::Activity<'a> {
    models::Activity::new()
    .timestamps(models::Timestamps::new()
        .start(time())
    )
    .state("Universal Game Client")
    .assets(models::Assets::new()
        .large_image("titanium")
        .large_text("Titanium Client")
        .small_image("dbug")
        .small_text("Developing")
    )
    .buttons(vec![
        models::Button::new("GitHub","https://github.com/TBBOFBD/Titanium"),
        models::Button::new("Discord","https://discord.gg/mh9VYsCrPT")
    ])
}

fn time() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as i64
}
