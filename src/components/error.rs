use std::fmt::Display;

use axum::{http::StatusCode, response::{Response, IntoResponse}};
use maud::{Markup, html};

pub fn error_html(err: impl Display) -> Markup {
    html! {
        #error class="error-message" { (err) }
    }
}

pub fn error_status_html(status: StatusCode, err: impl Display) -> Response {
    (status, error_html(err)).into_response()
}