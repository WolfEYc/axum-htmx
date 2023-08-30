use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult, Error};
use super::{client::Role, SearchFilter, PAGE_SIZE, company::Company};

#[derive(Debug, Deserialize)]
pub struct CreateCompanyReq {
    pub company_id: i32,
    pub client_id: i64
}

#[derive(Debug)]
pub struct PubClientInfo {
    pub id: i64,
    pub username: String,
    pub role: Role
}

async fn create(req: CreateCompanyReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO client_in_company VALUES
        ($1, $2)
    ",
    req.company_id,
    req.client_id
    )
    .execute(pool)
    .await
}

async fn read_clients(filter: SearchFilter, pool: &PgPool) -> Result<Vec<PubClientInfo>, Error> {
    sqlx::query!(
    "
        SELECT client.*, CASE WHEN v.client_id IS NOT NULL THEN true ELSE false END AS is_verified, CASE WHEN a.client_id IS NOT NULL THEN true ELSE false END AS is_admin
        FROM client_in_company ic
        JOIN client ON client.id = ic.client_id
        LEFT JOIN verified_client v ON v.client_id = ic.client_id
        LEFT JOIN admin_in_company a ON a.client_id = ic.client_id
        WHERE ic.company_id = $1
        AND client.username ILIKE '%' || $2 || '%'
        ORDER BY client.username
        OFFSET $3
        LIMIT $4
    ",
    filter.owner_id,
    filter.name,
    filter.calc_offset(),
    PAGE_SIZE
    )
    .fetch_all(pool)
    .await
    .map(|v|
        v.iter().map(|r| PubClientInfo {
            id: r.id,
            username: r.username.clone(),
            role: if Some(true) == r.is_admin { Role::Admin } else if Some(true) == r.is_verified { Role::Verified } else { Role::Normal } 
        }).collect()
    )
}

async fn read_admins(filter: SearchFilter, pool: &PgPool) -> Result<Vec<PubClientInfo>, Error> {
    sqlx::query!(
    "
        SELECT client.*
        FROM client_in_company
        JOIN client ON client_in_company.client_id = client.id
        JOIN admin_in_company ON admin_in_company.client_id = client.id
        WHERE client_in_company.company_id = $1
        AND client.username ILIKE '%' || $2 || '%'
        ORDER BY client.username
        OFFSET $3
        LIMIT $4
    ",
    filter.owner_id,
    filter.name,
    filter.calc_offset(),
    PAGE_SIZE
    )
    .fetch_all(pool)
    .await
    .map(|v|
        v.iter().map(|r| PubClientInfo {
            id: r.id,
            username: r.username.clone(),
            role: Role::Admin
        }).collect()
    )
}

pub async fn read_companies(client_id: i64, pool: &PgPool) -> Result<Vec<Company>, Error> {
    sqlx::query_as!(Company,
    "
        SELECT company.*
        FROM client_in_company
        JOIN company ON client_in_company.company_id = company.id
        WHERE client_in_company.client_id = $1
    ",
    client_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn read_verified_companies(client_id: i64, pool: &PgPool) -> Result<Vec<Company>, Error> {
    sqlx::query_as!(Company,
    "
        SELECT company.*
        FROM client_in_company
        JOIN company ON client_in_company.company_id = company.id
        JOIN verified_company ON verified_company.company_id = company.id
        WHERE client_in_company.client_id = $1
    ",
    client_id,
    )
    .fetch_all(pool)
    .await
}