//! <p align="center"><img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/></p><h1 align="center"></h1><p align="center"style="">A rust library for writing video game "utilities"</p>
//! 
//! <h1><b>Example</b>:</h1>
//! 
//! ```rust,no_run
//! fn main() {
//!    println!("Hello, world!");
//! }
//! ```

titaniumutils::declare_module!(
    ///Memory utilities
    "memory" > memory <= titaniummemory
);
titaniumutils::declare_module!(
    /// GUI utilities
    "gui" > gui <= titaniumgui
);
titaniumutils::declare_module!(
    /// Module for discord utilities
    /// # Example
    /// ```rust,no_run
    /// 
    "discord" > discord <= titaniumdiscord
);
titaniumutils::declare_module!(
    /// Module for Keybinds
    "keybinds" > keybinds <= titaniumkeybinds
);
