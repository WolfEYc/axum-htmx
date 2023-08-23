use axum::Form;
use maud::{html, Markup};
use serde::Deserialize;

pub async fn hello(user_input: Form<HelloForm>) -> Markup {
    html! {
        #content {
            p { "Hello " (user_input.name) "! This is HTMX." }
        }
    }
}

#[derive(Deserialize)]
pub struct HelloForm {
    name: String,
}