use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool, postgres::PgQueryResult, Error};

#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    pub id: i64,
    pub exp: u64,
    pub username: String,
    pub role: Role,
}

#[derive(Debug)]
pub struct Client {
    pub id: i64,
    pub username: String,
    pub otp_b32: String,
    pub role: Role
}

#[derive(Debug, FromRow)]
pub struct ClientID {
    pub id: i64
}

#[derive(Debug)]
pub struct CreateClientReq {
    pub username: String,
    pub otp_b32: String
}

#[derive(Debug, Deserialize)]
pub struct ValidateUsernameReq {
    pub username: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Normal,
    Verified,
    Admin
}

impl From<Client> for JWTClaims {
    fn from(value: Client) -> Self {
        JWTClaims { id: value.id, username: value.username, role: value.role, exp:  }
    }
}

pub async fn is_valid(req: &ValidateUsernameReq, pool: &PgPool) -> Result<bool, Error> {
    sqlx::query!("
        SELECT 1 as exists
        FROM client
        WHERE username = $1
    ",
    req.username
    )
    .fetch_optional(pool)
    .await
    .map(|r|r.is_none())
}

///Returns the created client's id (if successful)
pub async fn create(req: CreateClientReq, pool: &PgPool) -> Result<i64, Error> {
    sqlx::query_as!(ClientID,"
        INSERT INTO client VALUES
        (DEFAULT, $1, $2)
        RETURNING id
        ",
    req.username,
    req.otp_b32
    )
    .fetch_one(pool)
    .await
    .map(|r|r.id)
}

pub async fn read(id: i64, pool: &PgPool) -> Result<Client, Error> {
    sqlx::query!("
        SELECT *, (SELECT true FROM admin_client WHERE client_id = $1) as is_admin, (SELECT true FROM verified_client WHERE client_id = $1) as is_verified
        FROM client
        WHERE id = $1
    ",
    id
    )
    .fetch_one(pool)
    .await
    .map(|r| Client{
        id,
        username: r.username,
        otp_b32: r.otp_b32,
        role: if Some(true) == r.is_admin { Role::Admin } else if Some(true) == r.is_verified { Role::Verified } else { Role::Normal }
    })
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
    sqlx::query!("
        SELECT 1 as is_verified
        FROM verified_client
        WHERE client_id = $1 
    ",
    id
    )
    .fetch_optional(pool)
    .await
    .map(|r|r.is_some())
}

pub async fn is_admin(id: i64, pool: &PgPool) -> Result<bool, Error> {
    sqlx::query!("
        SELECT 1 as is_admin
        FROM admin_client
        WHERE client_id = $1 
    ",
    id
    )
    .fetch_optional(pool)
    .await
    .map(|r|r.is_some())
}
