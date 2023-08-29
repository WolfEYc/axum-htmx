use std::env::{self, VarError};

pub fn get_db_url() -> Result<String, VarError> {
    env::var("DATABASE_URL")
}

pub fn get_jwt_secret() -> Result<String, VarError> {
    env::var("JWT_SECRET")
}