use axum::{routing::{get, post}, Router, Server, handler::HandlerWithoutStateExt};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod strings;
mod page;
mod pages;
mod components;

use components::*;
use pages::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index::index))
        .route("/hello", post(hello::hello))
        .fallback_service(ServeDir::new("/static").not_found_service(notfound::not_found.into_service()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
