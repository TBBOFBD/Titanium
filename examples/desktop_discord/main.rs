use titanium::desktop::{
    discord::*,
    keybinds
};

fn main() {
    let mut drpc = DiscordClient::new(1090677709350391879);

    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    drpc.on_activity_join_request(|ctx| {
        println!("Join request: {:?}", ctx.event);
    });

    drpc.on_activity_join(|ctx| {
        println!("Joined: {:?}", ctx.event);
    });

    drpc.on_activity_spectate(|ctx| {
        println!("Spectate: {:?}", ctx.event);
    });

    let drpc_thread = drpc.start();

    println!("blocking!");

    drpc.block_until_event(DiscordEvent::Ready).unwrap();

    println!("ready!");

    // Set the activity
    drpc.set_activity(|act| act.state("rusting"))
        .expect("Failed to set activity");

    keybinds::exit::set_handler(move || {
        println!("Exiting...");
        drpc.clear_activity().unwrap();
        std::process::exit(0);
    })
    .unwrap();

    drpc_thread.join().unwrap();
}