use std::string::FromUtf8Error;
use axum::{http::{Request, StatusCode}, body::Body, extract::State, Form, Router, routing::{get, post}};
use maud::{Markup, html};
use serde::Deserialize;
use crate::{page, components::{six_digit_entry::six_digit_entry, error::error_html, copyblock::copyblock}, queries::client::{self, ValidateUsernameReq, CreateClientReq}, AppState, auth::{create_totp, verify_6digit_b32}, strings::TARGET_ERROR};

///On success, will set jwt access token cookie and redirect to console
pub async fn index(req: Request<Body>) -> Markup {
    let host = req.uri().to_string();
    let title = "axum-htmx-signup";
    let desc = "Create an account";
    
    let content = html! {
        h1 align="center" { "Create Account" }
        form hx-ext="response-targets" hx-post="/validate-username" hx-swap="outerHTML" {(TARGET_ERROR)} data-loading-states {
            label for="username" { "Username" }
            input type="text" name="username" placeholder="username" required;
            small { "Unique & Permanent" }
            button data-loading-aria-busy { "Check Username" }
            #error {}
        }
    };

    page::page(&host, title, desc, content)
}

pub async fn validate_username(State(state): State<AppState>, form: Form<ValidateUsernameReq>) -> (StatusCode, Markup) {
    match client::is_valid(&form.0, &state.db).await {
        Err(err) => (StatusCode::BAD_REQUEST, error_html(err)),
        Ok(false) => (StatusCode::FORBIDDEN, error_html("Username already taken")),
        Ok(true) => new_otp_form(form.0.username)
    }
}

fn otp_form(qr: String, secret_b32: String) -> Markup {
    html! {
        form hx-post="/signup-submission" hx-swap="outerHTML" {
            label for="qr" { "Scan me with your auth app" }
            small { "Preferably one that is backed up (not Google Authenticator)" }
            img src=(qr);
            (copyblock("otc_b32".to_string(), secret_b32))
            small { "Or manually add code" }
            (six_digit_entry())
            button data-loading-aria-busy { "Create Account" }
        }
    }
}

fn new_otp_form(username: String) -> (StatusCode, Markup) {
    let totp = create_totp(username);
    let Ok(totp) = totp else {
        return (StatusCode::BAD_REQUEST, error_html(totp.unwrap_err()));
    };

    let qr = totp.get_qr();
    let Ok(qr) = qr else {
        return (StatusCode::BAD_REQUEST, error_html(qr.unwrap_err()));
    };

    let secret = totp.get_secret_base32();
    (StatusCode::OK, otp_form(qr, secret))
}

#[derive(Debug, Deserialize)]
pub struct SignupSubmissionReq {
    pub username: String,
    pub otp_b32: String,
    pub otc_1: u8,
    pub otc_2: u8,
    pub otc_3: u8,
    pub otc_4: u8,
    pub otc_5: u8,
    pub otc_6: u8,
}

impl SignupSubmissionReq {
    fn get_six_digits(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(vec![self.otc_1, self.otc_2, self.otc_3, self.otc_4, self.otc_5, self.otc_6])
    }
}

pub async fn validate_otp(State(state): State<AppState>, form: Form<SignupSubmissionReq>) -> (StatusCode, Markup) {
    let sixdigits = form.get_six_digits();
    let Ok(sixdigits) = sixdigits else {
        return (StatusCode::BAD_REQUEST, error_html(sixdigits.unwrap_err()));
    };

    if let Err(verification) = verify_6digit_b32(&form.otp_b32, sixdigits) {
        return (StatusCode::BAD_REQUEST, error_html(verification));
    }
    
    match client::create(CreateClientReq{ username: form.username, otp_b32: form.otp_b32 }, &state.db).await {
        Ok(client_id) => 
    }
}

pub fn signup_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/validate-username", post(validate_username))
        .route("/validate-otp", post(validate_otp))
}

