use axum::Form;
use maud::{html, Markup};
use serde::Deserialize;
use tokio::time::sleep;
use std::time::Duration;

pub async fn hello(user_input: Form<HelloForm>) -> Markup {    
    sleep(Duration::from_millis(1000)).await;

    html! {
        #content {
            h1 align="center" { "Hello " (user_input.name) "! This is HTMX." }
        }
    }
}

#[derive(Deserialize)]
pub struct HelloForm {
    name: String,
}