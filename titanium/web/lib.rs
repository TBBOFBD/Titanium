pub use tide;

#[doc(hidden)]
use tide_websockets::{Message, WebSocket};

#[doc(hidden)]
use async_std::prelude::*;

pub fn serve_client_code<State: Clone + Send + Sync + 'static>(app: &mut tide::Server<State>, code: &'static str) {
    app.at("/client.js")
        .get(move |_| async move {
            Ok(
                tide::Response::builder(200)
                    .body(code)
                    .content_type(tide::http::mime::JAVASCRIPT)
                    .build()
            )
        });
}

pub fn serve_default_items<State: Clone + Send + Sync + 'static>(app: &mut tide::Server<State>) {
    app.at("/api.js")
        .get(|_| async move {
            Ok(
                tide::Response::builder(200)
                    .body(include_str!("./api/lib.js"))
                    .content_type(tide::http::mime::JAVASCRIPT)
                    .build()
            )
        });
    app.at("/loader.js")
        .get(|_| async move {
            Ok(
                tide::Response::builder(200)
                    .body(include_str!("./loader.js"))
                    .content_type(tide::http::mime::JAVASCRIPT)
                    .build()
            )
        });
    app.at("/")
        .with(WebSocket::new(|_request, mut stream| async move {
            while let Some(Ok(Message::Text(input))) = stream.next().await {
                println!("data: {}", &input);

                stream
                    .send_string(input)
                    .await?;
            }

            Ok(())
        }))
        .get(|_| async move {
            Ok(
                tide::Response::builder(200)
                    .body(ROOT_PAGE)
                    .content_type(tide::http::mime::HTML)
                    .build()
            )
        });
}

#[doc(hidden)]
const ROOT_PAGE: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Titanium</title>
    </head>
    <body>
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