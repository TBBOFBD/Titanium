//! <p align="center"><img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/></p><h1 align="center"></h1><p align="center"style="">A rust library for writing video game "utilities"</p><div align="center"><a><img src="https://img.shields.io/github/license/AtomicGamer9523/Titanium?label=License&color=blue"></a><br><a href="https://www.github.com/AtomicGamer9523"><img src="https://img.shields.io/github/followers/atomicgamer9523?label=AtomicGamer9523%20(Me)&style=social"/></a></div>
//! 
//! <h1><b>Example</b>:</h1>
//! 
//! ```rust,no_run
//! fn main() {
//!    println!("Hello, world!");
//! }
//! ```

pub use titaniumcore::*;

#[cfg(feature = "gui")]
pub mod gui {
    pub use titaniumgui::*;
}
