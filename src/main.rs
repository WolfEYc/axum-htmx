use axum::{routing::{get, post}, Router, Server, handler::HandlerWithoutStateExt};
use std::{net::SocketAddr, error::Error};
use tower_http::services::ServeDir;

mod auth;
mod app_env;
mod strings;
mod page;
mod pages;
mod components;
mod queries;

use components::*;
use pages::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let state = app_env::create_appstate().await?;
    sqlx::migrate!().run(&state.db).await?;

    let router = Router::new()
        .route("/", get(index::index))
        .route("/hello", post(hello::hello))
        .route("/login", get(pages::login::index).post(pages::login::login))
        .nest("/signup", signup::signup_routes())
        .with_state(state)
        .fallback_service(ServeDir::new("./static")
            .not_found_service(notfound::not_found.into_service()));
        

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
        .await?)
}
