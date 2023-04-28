//! <p align="center"><img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/></p><h1 align="center"></h1><p align="center"style="">A rust library for writing video game "utilities"</p>
//! 
//! <h1><b>Examples</b>:</h1>
//! 

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

// titaniumutils::warn!(
//     all(
//         feature = "desktop",
//         feature = "web",
//     ),
//     "Both desktop and web features are enabled. This is not recommended."
// );

/// Web-related utilities
/// 
/// # Example
/// 
/// ```rust
/// use titanium::web;
/// use web::prelude::*;
/// 
/// #[titanium::main]
/// async fn main() -> web::Result<()> {
///     let mut app = web::new_server();
///     app.at("/")
///         .with(web::WebSocket::new(|_, mut stream| async move {
///             while let Some(Ok(web::Message::Text(input))) = stream.next().await {
///                 println!("data: {}", &input);
///                 stream
///                     .send_string(input)
///                     .await?;
///             }
///             Ok(())
///         }))
///         .get(web::root_page_endpoint);
///     web::serve_internals(&mut app);
///     web::serve_client_code(&mut app, include_str!("./client.js"));
///     app.listen("127.0.0.1:8080").await?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "web")]
pub mod web {
    pub use titaniumweb::*;
}

/// Module for desktop utilities
#[cfg(feature = "desktop")]
pub mod desktop {
    pub use titaniumdesktop::*;
}

#[cfg(feature = "macros")]
pub use titaniummacros::*;

/// Internals mostly used by macros
#[doc(hidden)]
pub mod __internals__ {
    /// Runs the main function
    #[doc(hidden)]
    #[cfg(feature = "macros")]
    pub use async_std::task::block_on as run_main;
}

/// A basic macro for printing "Hello Titanium world!"
#[macro_export]
macro_rules! hello_world {
    () => {
        println!("Hello Titanium world!");
    };
}
