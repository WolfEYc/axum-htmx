use std::{env::{self}, error::Error};
use axum::extract::FromRef;
use sqlx::{PgPool, postgres::PgPoolOptions};
use axum_extra::extract::cookie::Key;
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cookie_key: Key,
    pub jwt_key: Hmac<Sha256>
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

async fn create_pool() -> Result<PgPool, Box<dyn Error>>  {
    Ok(PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL")?)
        .await?)
}

pub async fn create_appstate() -> Result<AppState, Box<dyn Error>> {
    Ok(AppState {
        db: create_pool().await?,
        cookie_key: Key::try_from(env::var("COOKIE_SECRET")?.as_bytes())?,
        jwt_key: Hmac::new_from_slice(env::var("JWT_SECRET")?.as_bytes())?
    })
}

