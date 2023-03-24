// Used to launch without a command line interface
#![cfg_attr(all(not(debug_assertions),target_os="windows"),windows_subsystem="windows")]

use titanium::desktop::{
    // memory::MemoryManager,
    gui::{
        GuiInstance,
        GuiLauncher
    }
};

fn main() -> std::io::Result<()> {
    // let manager = MemoryManager::new("game.exe")?;
    let main_window = GuiInstance::default();
    GuiLauncher::launch(main_window)
        .expect("Failed to launch application");
    Ok(())
}