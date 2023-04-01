//! <p align="center"><img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/></p><h1 align="center"></h1><p align="center"style="">A rust library for writing video game "utilities"</p>
//! 
//! <h1><b>Example</b>:</h1>
//! 
//! ```rust
//! use titanium::web;
//! use web::prelude::*;
//! 
//! #[titanium::main]
//! async fn main() -> web::Result<()> {
//!     let mut app = web::new_server();
//!     app.at("/")
//!         .with(web::WebSocket::new(|_, mut stream| async move {
//!             while let Some(Ok(web::Message::Text(input))) = stream.next().await {
//!                 println!("data: {}", &input);
//!                 stream
//!                     .send_string(input)
//!                     .await?;
//!             }
//!             Ok(())
//!         }))
//!         .get(web::root_page_endpoint);
//!     web::serve_internals(&mut app);
//!     web::serve_client_code(&mut app, include_str!("./client.js"));
//!     app.listen("127.0.0.1:8080").await?;
//!     Ok(())
//! }
//! ```

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

pub use tide::Result;

/// Creates a new server instance.
pub fn new_server() -> tide::Server<()> {
    tide::new()
}

/// Serves a javascript file from a `&'static str` at the specified path.
pub fn serve_js<State: Clone + Send + Sync + 'static>(app: &mut tide::Server<State>, path: &str, js: &'static str) {
    app.at(path)
        .get(move |_| async move {
            Ok(
                tide::Response::builder(200)
                    .body(js)
                    .content_type(tide::http::mime::JAVASCRIPT)
                    .build()
            )
        });
}

pub use tide_websockets::{Message, WebSocket};

/// Imports the prelude from other libraries.
pub mod prelude {
    #[doc(hidden)]
    pub use async_std::prelude::*;
}

/// Serves a javascript file from a `&'static str` at the specified path.
/// This is used to serve the client code.
/// 
/// # Example
/// ```rust
/// use titanium::web;
/// 
/// #[titanium::main]
/// async fn main() -> web::Result<()> {
///     let mut app = web::new_server();
///     web::serve_internals(&mut app);
///     web::serve_client_code(&mut app, include_str!("./client.js"));
///     app.listen("127.0.0.1:8080").await?;
///     Ok(())
/// }
/// ```
pub fn serve_client_code<State: Clone + Send + Sync + 'static>(app: &mut tide::Server<State>, code: &'static str) {
    serve_js(app, "/client.js", code);
}

/// Serves the internal javascript files that are required for the web server to function.
/// 
/// # Example
/// ```rust
/// use titanium::web;
/// 
/// #[titanium::main]
/// async fn main() -> web::Result<()> {
///     let mut app = web::new_server();
///     web::serve_internals(&mut app);
///     web::serve_client_code(&mut app, include_str!("./client.js"));
///     app.listen("127.0.0.1:8080").await?;
///     Ok(())
/// }
/// ```
pub fn serve_internals<State: Clone + Send + Sync + 'static>(app: &mut tide::Server<State>) {
    serve_js(app, "/api.js", include_str!("./api/lib.js"));
    serve_js(app, "/loader.js", include_str!("./loader.js"));
}

/// Serves a basic HTML page that should be served when the root of the server is requested.
pub async fn root_page_endpoint<
    State: Clone + Send + Sync + 'static
>(_: tide::Request<State>) -> tide::Result<tide::Response> {
    Ok(tide::Response::builder(200)
        .body(ROOT_PAGE)
        .content_type(tide::http::mime::HTML)
        .build()
    )
}

/// A basic HTML page that should be served when the root of the server is requested.
pub const ROOT_PAGE: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Titanium</title>
    </head>
    <body style="background-color:#22272e">
        <p align="center">
            <img src="https://i.imgur.com/hnyLrub.png"alt="titanium-logo"style="width:40%;height:5%px;object-fit:cover;object-position:center -10px""/>
        </p>
        <h1 align="center"></h1>
        <p align="center"style="">A rust library for writing video game "utilities"</p>
        <div align="center">
            <a>
                <img src="https://img.shields.io/github/license/AtomicGamer9523/Titanium?label=License&color=blue">
            </a><br>
            <a href="https://www.github.com/AtomicGamer9523">
                <img src="https://img.shields.io/github/followers/atomicgamer9523?label=AtomicGamer9523%20(Me)&style=social"/>
            </a>
        </div><br>
        <h3><b>Obtaining</b>:</h3>
        <samp>Mercurial: </samp><kbd>hg clone linkrbot.com/hg/titanium</kbd><br>
        <samp>Git: </samp><kbd>git clone github.com/AtomicGamer9523/titanium</kbd>
    </body>
</html>"#;
