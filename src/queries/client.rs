use bcrypt::{hash,DEFAULT_COST};
use sqlx::{FromRow, PgPool, postgres::PgQueryResult, Error};

#[derive(Debug, FromRow)]
pub struct Client {
    pub id: i64,
    pub email: String,
    pub access: Access
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Access {
    Client = 0,
    Admin = 1,
}

fn hash_pwd(password: String) -> Result<String, Error> {
    match hash(password, DEFAULT_COST) {
        Ok(hashword) => Ok(hashword),
        Err(_) => Err(Error::RowNotFound),
    }
}

/// Creates temp client, still requires email verification
pub async fn create(email: String, password: String, access: Access, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO temp_client VALUES
        (DEFAULT, DEFAULT, $1, $2, $3)",
    access as i16,
    email,
    hash_pwd(password)?
    )
    .execute(pool)
    .await
}