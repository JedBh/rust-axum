#![allow(unused)] // for beginning only

use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}\n", addr);
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()
}
