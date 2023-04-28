use titanium::desktop::{discord,keybinds};
use discord::DiscordIpc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = discord::titanium()?;
    loop {
        if let Ok(_) = client.connect() { break };
        println!("Failed to connect to Discord, retrying in 10 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    client.set_activity(discord::titanium_activity())?;
    keybinds::exit::set_handler(move || {
        println!();println!("Exiting...");
        client.close().expect("Failed to close client");
        std::process::exit(0);
    })?;

    loop {}
}
