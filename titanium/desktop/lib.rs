//! <p align="center"><img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/></p><h1 align="center"></h1><p align="center"style="">A rust library for writing video game "utilities"</p>
//! 
//! <h1><b>Example</b>:</h1>
//! 
//! ```rust,no_run
//! fn main() {
//!    println!("Hello, world!");
//! }
//! ```

#![deny(unsafe_code)]

pub use titaniumcore::*;

#[cfg(feature = "gui")]
pub mod gui {
    pub use titaniumgui::*;
}
