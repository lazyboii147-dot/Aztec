use axum::{routing::get, Router};
use std::net::SocketAddr;

mod routes;
mod dto;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/glyph/:name", get(routes::glyph))
        .route("/deity/:name", get(routes::deity));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
