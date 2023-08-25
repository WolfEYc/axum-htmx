use axum::{routing::{get, post}, Router, Server, handler::HandlerWithoutStateExt};
use std::{net::SocketAddr, error::Error};
use tower_http::services::ServeDir;

mod db;
mod strings;
mod page;
mod pages;
mod components;
mod queries;

use db::*;
use components::*;
use pages::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let pool = create_pool().await?;
    sqlx::migrate!().run(&pool).await?;

    let router = Router::new()
        .route("/", get(index::index))
        .route("/hello", post(hello::hello))
        .fallback_service(ServeDir::new("/static")
            .not_found_service(notfound::not_found.into_service()))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
        .await?)
}
