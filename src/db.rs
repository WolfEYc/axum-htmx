use std::{env, error::Error};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, Box<dyn Error>>  {
    Ok(PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL")?)
        .await?)
}
