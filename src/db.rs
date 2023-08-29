use std::error::Error;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::app_env;

pub async fn create_pool() -> Result<PgPool, Box<dyn Error>>  {
    Ok(PgPoolOptions::new()
        .connect(&app_env::get_db_url()?)
        .await?)
}
