use axum::{http::Request, body::Body};
use maud::{Markup, html};

use crate::page;

pub async fn index(req: Request<Body>) -> Markup {
    let host = format!("{}", req.uri());
    let title = "axum-htmx";
    let desc = "This is a template. There are many like it but this one is mine.";
    // TODO: Add your site or application content here.
    let content = html! {
        #content {
            h1 align="center" { "Hello world! This is Maud." }
        }
        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" data-loading-states {
            input type="text" name="name" value="" placeholder="What's your name?";
            button data-loading-aria-busy { "Submit" }
        }
    };

    page::page(&host, title, desc, content)
}