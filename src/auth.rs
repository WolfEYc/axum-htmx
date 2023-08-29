use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}, error::Error, str::Utf8Error};
use jwt::{SignWithKey, VerifyWithStore, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::PgPool;
use totp_rs::{Rfc6238, TOTP, Secret};
use crate::{strings::TOTP_ISSUER, queries::client::{self, JWTClaims}, app_env};

pub fn create_totp(username: String) -> Result<TOTP, Box<dyn Error>> {
    let secret = Secret::generate_secret().to_bytes()?;

    let rfc = Rfc6238::with_defaults(secret)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.clone());
    
    TOTP::from_rfc6238(rfc)
}

pub fn recreate_totp(b32: &String, username: Option<String>) -> Result<TOTP, Box<dyn Error>> {
    let bytes = Secret::Encoded(*b32).to_bytes().map_err(|e|e.to_string())?;
    let mut rfc = Rfc6238::with_defaults(bytes)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.unwrap_or_default());
    
    Ok(TOTP::from_rfc6238(rfc)?)
}

pub fn verify_6digit_b32(b32: &String, six_digit: String) -> Result<(), Box<dyn Error>> {
    let totp = recreate_totp(b32, None)?;

    match totp.check_current(&six_digit)? {
        true => Ok(()),
        false => Err("Invalid Code!".into()),
    }
}

///Returns session token if success
pub async fn login(id: i64, six_digit: String, pool: &PgPool, key: &Hmac<Sha256>) -> Result<String, Box<dyn Error>> {
    let client = client::read(id, pool).await?;
    verify_6digit_b32(&client.otp_b32, six_digit)?;
    Ok(JWTClaims::from(client).sign_with_key(key)?)
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

pub fn gen_key() -> Result<Hmac<Sha256>, Box<dyn Error>> {
    Ok(Hmac::new_from_slice(app_env::get_jwt_secret()?.as_bytes())?)
}

pub fn verify_jwt(jwt: String, key: &Hmac<Sha256>) -> Result<JWTClaims, Error> {
    jwt.verify_with_key(key)
}


