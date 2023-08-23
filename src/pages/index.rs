use axum::{http::Request, body::Body};
use maud::{Markup, html};
use crate::page;

pub async fn index(req: Request<Body>) -> Markup {
    let host = format!("{}", req.uri());
    let title = "actix-maud-htmx-h5bp";
    let desc = "This is a template. There are many like it but this one is mine.";
    let lang = "en";
    // TODO: Add your site or application content here.
    let content = html! {
        #content {
            p { "Hello world! This is HTML5 Boilerplate." }
        }
        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                input type="text" name="name" value="" {}
            }
            button { "Submit" }
        }
    };

    page::page(&host, title, desc, lang, content)
}