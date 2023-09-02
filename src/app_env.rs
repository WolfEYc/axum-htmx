use std::env;
use sqlx::{PgPool, postgres::PgPoolOptions};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::sync::OnceCell;

use crate::Boxres;

static STATE: OnceCell<AppState> = OnceCell::const_new();
pub fn state() -> &'static AppState {
    STATE.get().unwrap()
}

#[derive(Debug)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_key: Hmac<Sha256>
}

async fn create_pool() -> Boxres<PgPool>  {
    Ok(PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL")?)
        .await?)
}

pub async fn create_appstate() -> Boxres<()> {
    Ok(STATE.set(AppState {
        db: create_pool().await?,
        jwt_key: Hmac::new_from_slice(env::var("JWT_SECRET")?.as_bytes())?
    })?)
}
