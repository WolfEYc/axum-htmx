use maud::Markup;
use totp_rs::{Rfc6238, TOTP};

use crate::components::error::error_html;

pub fn verify_6digit_b32(b32: String, six_digit: String) -> Result<(), Markup> {
    let rfc = Rfc6238::with_defaults(b32.into_bytes());
    let Ok(mut rfc) = rfc else {
        return Err(error_html(rfc.unwrap_err()));
    };
    
    let totp = TOTP::from_rfc6238(rfc);
    let Ok(totp) = totp else {
        return Err(error_html(totp.unwrap_err()));
    };

    match totp.check_current(&six_digit) {
        Ok(true) => Ok(()),
        Ok(false) => Err(error_html("Invalid Code!")),
        Err(err) => Err(error_html(err))
    } 
}