use axum::{http::{Request, StatusCode}, Form, response::{Response, IntoResponse}, body::Body};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jwt::SignWithKey;
use maud::{Markup, html};
use serde::Deserialize;

use crate::{page::{self, hx_redirect}, components::{error::error_html, six_digit_entry::six_digit_entry}, queries::client::{self, JWTClaims}, auth::verify_6digit_b32, app_env::state};

///On success, will set jwt access token cookie and redirect to console
pub async fn index(req: Request<Body>) -> Markup {
    let host = req.uri().to_string();
    let title = "axum-htmx-signup";
    let desc = "Create an account";
    
    let content = html! {
        form hx-post="/login" hx-swap="outerHTML" hx-target="#error" data-loading-states {
            article {
                header {
                    label for="username" { "Username" }
                    input type="text" name="username" placeholder="username" required;
                }
                body {
                    (six_digit_entry())
                    button data-loading-aria-busy { "Login" }
                    #error {}
                }
            }
        }
        a hx-boost="true" href="/signup" role="button" class="secondary" { "Signup Instead" }
        
    };

    page::page(&host, title, desc, content)
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub six_digits: String
}

///Returns session token if success
pub async fn login(jar: CookieJar, form: Form<LoginReq>) -> Response {
    let client = client::read_from_username(form.username.clone(), &state().db).await;
    let Ok(client) = client else {
        return (StatusCode::BAD_REQUEST, error_html(client.unwrap_err())).into_response();
    };

    if let Err(verification) = verify_6digit_b32(client.otp_b32.clone(), form.six_digits.clone()) {
        return (StatusCode::BAD_REQUEST, error_html(verification)).into_response();
    }

    let claims = JWTClaims::from(client);

    let token = claims.sign_with_key(&state().jwt_key);
    let Ok(token) = token else {
        return (StatusCode::BAD_REQUEST, error_html(token.unwrap_err())).into_response();
    };

    let jar = jar.add(Cookie::new("access_token", token));
    (jar, hx_redirect("/")).into_response()
}