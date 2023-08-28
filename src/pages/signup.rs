use std::string::FromUtf8Error;

use axum::{http::Request, body::Body, extract::State, Form};
use maud::{Markup, html};
use serde::Deserialize;
use totp_rs::{Secret, Rfc6238, TOTP};
use crate::{page, components::{six_digit_entry::{six_digit_entry, self}, error::error_html, copyblock::copyblock}, queries::client::{self, ValidateUsernameReq}, AppState, strings::TOTP_ISSUER};

pub async fn username_form(req: Request<Body>) -> Markup {
    let host = req.uri().to_string();
    let title = "axum-htmx-signup";
    let desc = "Create an account";
    
    let content = html! {
        h1 align="center" { "Create Account" }
        
        form hx-post="/validate-username" hx-swap="outerHTML" data-loading-states {
            label for="username" { "Username" }
            input type="text" name="username" placeholder="username" required;
            small { "Unique & Permanent" }
            button data-loading-aria-busy { "Check Username" }
        }
    };

    page::page(&host, title, desc, content)
}

pub async fn validate_username(State(state): State<AppState>, form: Form<ValidateUsernameReq>) -> Markup {
    match client::is_valid(&form.0, &state.db).await {
        Err(err) => html!(div class="error-message" { (err.to_string()) }),
        Ok(false) => html!(div class="error-message" { "Username taken" }),
        Ok(true) => otp_form(form.0.username)
    }
}

fn otp_form(username: String) -> Markup {
    
    let secret = Secret::generate_secret().to_bytes();
    let Ok(secret) = secret else {
        return error_html(secret.unwrap_err());
    };

    let rfc = Rfc6238::with_defaults(secret);
    let Ok(mut rfc) = rfc else {
        return error_html(rfc.unwrap_err());
    };

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.clone());
    
    let totp = TOTP::from_rfc6238(rfc);
    let Ok(totp) = totp else {
        return error_html(totp.unwrap_err());
    };

    let qr = totp.get_qr();
    let Ok(qr) = qr else {
        return error_html(qr.unwrap_err());
    };

    let secret = totp.get_secret_base32();

    html! {
        form hx-post="/client" hx-swap="outerHTML" {
            label for="qr" { "Scan me with your auth app" }
            small { "Preferably one that is backed up (not Google Authenticator)" }
            img src=(qr);
            (copyblock("otc_b32".to_string(), secret))
            small { "Or manually add code" }
            (six_digit_entry())
            button data-loading-aria-busy { "Create Account" }
        }
    }
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

pub async fn signup_submission(State(state): State<AppState>, form: Form<SignupSubmissionReq>) -> Markup {
    
    let sixdigits = form.get_six_digits();
    if let Err(sixdigits) = sixdigits {
        
    }

    if let Err(verify_6digit_b32(form))
    

}


