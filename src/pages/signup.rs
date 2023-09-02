use axum::{http::{Request, StatusCode}, body::Body, Form, Router, routing::{get, post}, response::{Response, IntoResponse}};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jwt::SignWithKey;
use maud::{Markup, html};
use serde::Deserialize;
use serde_json::json;
use crate::{page::{self, hx_redirect}, components::{error::error_html, copyblock::copyblock, six_digit_entry::six_digit_entry}, queries::client::{self, CreateClientReq}, auth::{create_totp, verify_6digit_b32}, strings::{TARGET_ERROR, qr}, app_env::state};

///On success, will set jwt access token cookie and redirect to console
async fn index(req: Request<Body>) -> Markup {
    let host = req.uri().to_string();
    let title = "axum-htmx-signup";
    let desc = "Create an account";
    
    let content = html! {
        form hx-ext="response-targets" hx-post="/signup/validate-username" hx-swap="outerHTML" .{(TARGET_ERROR)} data-loading-states {
            article {
                header {
                    h1 align="center" { "Create Account" }
                    label for="username" { "Username" }
                    input type="text" name="username" placeholder="Username" required;
                    small { "Unique & Permanent" }
                }
                body {
                    button data-loading-aria-busy { "Check Username" }
                    #error {}
                }
            }
        }
        a hx-boost="true" href="/login" role="button" class="secondary" { "Login Instead" }
    };

    page::page(&host, title, desc, content)
}

#[derive(Debug, Deserialize)]
pub struct ValidateUsernameReq {
    pub username: String
}

async fn validate_username(form: Form<ValidateUsernameReq>) -> (StatusCode, Markup) {
    match client::is_valid(&form.0, &state().db).await {
        Err(err) => (StatusCode::BAD_REQUEST, error_html(err)),
        Ok(false) => (StatusCode::FORBIDDEN, error_html("Username already taken")),
        Ok(true) => new_otp_form(form.0.username)
    }
}

fn otp_form(username: String, qr_b64: String, secret_b32: String) -> Markup {
    let hxvals = json!({
        "username": username,
        "otp_b32": secret_b32
    });

    html! {
        form hx-post="/signup/validate-otp" hx-swap="outerHTML" hx-target="#error" hx-vals=(hxvals.to_string()) data-loading-states {
            article {
                header {
                    label class="center" for="qr" { "Scan me with your auth app" }
                    img id="qr" class="center qr" src=(qr(qr_b64));
                    br;
                    (copyblock(secret_b32))
                }
                body {
                    (six_digit_entry())
                    button class="center" data-loading-aria-busy style="width:auto" { "Create Account" }
                }
            }
            #error {}
        }
    }
}

fn new_otp_form(username: String) -> (StatusCode, Markup) {
    let totp = create_totp(username.clone());
    let Ok(totp) = totp else {
        return (StatusCode::BAD_REQUEST, error_html(totp.unwrap_err()));
    };

    let qr = totp.get_qr();
    let Ok(qr) = qr else {
        return (StatusCode::BAD_REQUEST, error_html(qr.unwrap_err()));
    };

    let secret = totp.get_secret_base32();
    (StatusCode::OK, otp_form(username, qr, secret))
}

#[derive(Deserialize)]
pub struct SignupSubmissionReq {
    pub username: String,
    pub otp_b32: String,
    pub six_digits: String
}

impl From<SignupSubmissionReq> for CreateClientReq {
    fn from(value: SignupSubmissionReq) -> Self {
        CreateClientReq{ username: value.username, otp_b32: value.otp_b32 }
    }
}

///On Success will set the JWT Cookie
async fn validate_otp(jar: CookieJar, form: Form<SignupSubmissionReq>) -> Response {
    if let Err(verification) = verify_6digit_b32(form.otp_b32.clone(), form.six_digits.clone()) {
        return (StatusCode::BAD_REQUEST, error_html(verification)).into_response();
    }
    match client::create(form.0.into(), &state().db).await {
        Ok(claims) => {
            let token = claims.sign_with_key(&state().jwt_key);
            let Ok(token) = token else {
                return (StatusCode::BAD_REQUEST, error_html(token.unwrap_err())).into_response();
            };
            let jar = jar.add(Cookie::new("access_token", token));
            (jar, hx_redirect("/")).into_response()
        },
        Err(_) => todo!(),
    }
}

pub fn signup_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/validate-username", post(validate_username))
        .route("/validate-otp", post(validate_otp))
}

