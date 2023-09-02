use auth::req_admin;
use axum::{routing::{get, post}, Router, Server, handler::HandlerWithoutStateExt, middleware::from_fn};
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

type Boxres<T> = Result<T,Box<dyn Error>>;

#[tokio::main]
async fn main() -> Boxres<()> {
    dotenvy::dotenv()?;
    app_env::create_appstate().await?;

    let router = Router::new()
        .route("/console", get(console::index))
        .layer(from_fn(req_admin))
        .route("/", get(index::index))
        .route("/hello", post(hello::hello))
        .route("/login", get(pages::login::index).post(pages::login::login))
        .nest("/signup", signup::signup_routes())
        .fallback_service(ServeDir::new("./static")
            .not_found_service(notfound::not_found.into_service()));
        

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
        .await?)
}
