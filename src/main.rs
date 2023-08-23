use std::path::PathBuf;
use axum::{routing::{get, post}, Router, handler::HandlerWithoutStateExt};

mod strings;
mod page;
mod pages;
mod components;

use components::*;
use pages::*;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {

    let router = Router::new()
    .route("/", get(index::index))
    .route("/hello", post(hello::hello))
    .fallback_service(ServeDir::new(static_folder).not_found_service(notfound::not_found.into_service()));

    Ok(router.into())
}
