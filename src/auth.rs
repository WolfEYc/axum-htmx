use std::{time::{SystemTime, UNIX_EPOCH}, io};
use axum::{http::{Request, StatusCode}, response::{Response, IntoResponse}, middleware::Next};
use axum_extra::extract::CookieJar;
use hmac::Hmac;
use jwt::VerifyWithKey;
use io::{Error, ErrorKind};
use sha2::Sha256;
use totp_rs::{Rfc6238, TOTP, Secret};
use crate::{strings::TOTP_ISSUER, queries::client::{Role, JWTClaims}, Boxres, components::error::error_html, app_env::state};

pub fn create_totp(username: String) -> Boxres<TOTP> {
    let secret = Secret::generate_secret().to_bytes().map_err(|e|e.to_string())?;

    let mut rfc = Rfc6238::with_defaults(secret)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.clone());
    
    Ok(TOTP::from_rfc6238(rfc)?)
}

pub fn recreate_totp(b32: String, username: Option<String>) -> Boxres<TOTP> {
    let bytes = Secret::Encoded(b32).to_bytes().map_err(|e|e.to_string())?;
    let mut rfc = Rfc6238::with_defaults(bytes)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.unwrap_or_default());
    
    Ok(TOTP::from_rfc6238(rfc)?)
}

pub fn verify_6digit_b32(b32: String, six_digit: String) -> Boxres<()> {
    let totp = recreate_totp(b32, None)?;

    match totp.check_current(&six_digit)? {
        true => Ok(()),
        false => Err("Invalid Code!".into()),
    }
}

pub fn unix_sex() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn gen_exp() -> u64 {
    unix_sex() + 3600
}

fn get_claims(key: &Hmac<Sha256>, jar: CookieJar) -> Boxres<JWTClaims> {
    Ok(jar.get("access_token")
    .ok_or_else(|| Error::new(ErrorKind::NotFound, "Missing Token"))?
    .to_string()
    .verify_with_key(key)?)
}

fn req_auth(role_req: Role, jar: CookieJar) -> Result<JWTClaims, Response> {
    let claims_res = get_claims(&state().jwt_key, jar);
    
    let Ok(claims) = claims_res else {
        return Err((StatusCode::UNAUTHORIZED, error_html(claims_res.unwrap_err())).into_response());
    };

    if claims.role < role_req {
        return Err((StatusCode::UNAUTHORIZED, error_html(format!("Must be {:?} to access", role_req))).into_response());
    };

    Ok(claims)
}

pub async fn req_admin<B>(jar: CookieJar, mut req: Request<B>, next: Next<B>) -> Response { 
    match req_auth(Role::Admin, jar) {
        Err(err) => return err,
        Ok(claims) => req.extensions_mut().insert(claims)
    };

    next.run(req).await
}

pub async fn req_role<B>(jar: CookieJar, mut req: Request<B>, next: Next<B>) -> Response { 
    match req_auth(Role::Normal, jar) {
        Err(err) => return err,
        Ok(claims) => req.extensions_mut().insert(claims)
    };

    next.run(req).await
}