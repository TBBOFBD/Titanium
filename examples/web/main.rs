use titanium::web;
use web::prelude::*;

#[titanium::main]
async fn main() -> web::Result<()> {
    let mut app = web::new_server();
    app.at("/")
        .with(web::WebSocket::new(|_, mut stream| async move {
            while let Some(Ok(web::Message::Text(input))) = stream.next().await {
                println!("data: {}", &input);

                stream
                    .send_string(input)
                    .await?;
            }

            Ok(())
        }))
        .get(web::root_page_endpoint);
    web::serve_internals(&mut app);
    web::serve_client_code(&mut app, include_str!("./client.js"));
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
