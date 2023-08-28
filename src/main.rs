use axum::{routing::{get, post}, Router, Server, handler::HandlerWithoutStateExt};
use sqlx::PgPool;
use std::{net::SocketAddr, error::Error};
use tower_http::services::ServeDir;

mod auth;
mod db;
mod strings;
mod page;
mod pages;
mod components;
mod queries;

use db::*;
use components::*;
use pages::*;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let pool = create_pool().await?;
    sqlx::migrate!().run(&pool).await?;

    let router = Router::new()
        .route("/", get(index::index))
        .route("/hello", post(hello::hello))
        .route("/signup", 
            get(signup::username_form)
            .post(signup::validate_username))
        .route("/client", post(signup::signup_submission))
        .with_state(AppState{ db: pool })
        .fallback_service(ServeDir::new("./static")
            .not_found_service(notfound::not_found.into_service()));
        

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
        .await?)
}
