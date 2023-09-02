use axum::{http::Request, body::Body, Extension};
use maud::{Markup, html};
use crate::{page, queries::client::JWTClaims};

pub async fn index(Extension(claims): Extension<JWTClaims>, req: Request<Body>) -> Markup {
    let host = format!("{}", req.uri());
    let title = "axum-htmx";
    let desc = "This is a template. There are many like it but this one is mine.";
    // TODO: Add your site or application content here.
    let content = html! {
        #content {
            h1 align="center" { "Hello "(claims.username)" This is Maud." }
        }
        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" data-loading-states {
            input type="text" name="name" value="" placeholder="What's your name?";
            button data-loading-aria-busy { "Submit" }
        }
    };

    page::page(&host, title, desc, content)
}