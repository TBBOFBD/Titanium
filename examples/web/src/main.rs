use titanium::{
    web::{
        self,
        tide
    },
};

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    web::serve_default_items(&mut app);
    web::serve_client_code(&mut app, include_str!("./client.js"));
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}