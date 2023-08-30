use std::{time::{SystemTime, UNIX_EPOCH}, error::Error};
use totp_rs::{Rfc6238, TOTP, Secret};
use crate::strings::TOTP_ISSUER;

pub fn create_totp(username: String) -> Result<TOTP, Box<dyn Error>> {
    let secret = Secret::generate_secret().to_bytes().map_err(|e|e.to_string())?;

    let mut rfc = Rfc6238::with_defaults(secret)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.clone());
    
    Ok(TOTP::from_rfc6238(rfc)?)
}

pub fn recreate_totp(b32: String, username: Option<String>) -> Result<TOTP, Box<dyn Error>> {
    let bytes = Secret::Encoded(b32).to_bytes().map_err(|e|e.to_string())?;
    let mut rfc = Rfc6238::with_defaults(bytes)?;

    rfc.issuer(TOTP_ISSUER.to_string());
    rfc.account_name(username.unwrap_or_default());
    
    Ok(TOTP::from_rfc6238(rfc)?)
}

pub fn verify_6digit_b32(b32: String, six_digit: String) -> Result<(), Box<dyn Error>> {
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


