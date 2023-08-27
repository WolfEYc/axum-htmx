use serde::Deserialize;
use sqlx::{FromRow, PgPool, postgres::PgQueryResult, Error};

#[derive(Debug, FromRow)]
pub struct Client {
    pub id: i64,
    pub otp_b32: String,
    pub username: String
}

#[derive(Debug, Deserialize)]
pub struct CreateClientReq {
    pub otp_b32: String,
    pub username: String
}

pub async fn create(req: CreateClientReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO client VALUES
        (DEFAULT, $1, $2)",
    req.otp_b32,
    req.username
    )
    .execute(pool)
    .await
}

pub async fn read(id: i64, pool: &PgPool) -> Result<Client, Error> {
    sqlx::query_as!(Client,"
        SELECT *
        FROM client
        WHERE id = $1
    ",
    id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(id: i64, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        DELETE FROM client
        WHERE id = $1
    ",
    id
    ).execute(pool)
    .await
}

pub async fn is_verified(id: i64, pool: &PgPool) -> Result<bool, Error> {
    Ok(sqlx::query!("
        SELECT 1 as is_verified
        FROM verified_client
        WHERE client_id = $1 
    ",
    id
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}

pub async fn is_admin(id: i64, pool: &PgPool) -> Result<bool, Error> {
    Ok(sqlx::query!("
        SELECT 1 as is_admin
        FROM admin_client
        WHERE client_id = $1 
    ",
    id
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}