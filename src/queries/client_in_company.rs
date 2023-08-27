use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult, Error};
use super::{client::Client, SearchFilter, PAGE_SIZE, company::Company};

#[derive(Debug, Deserialize)]
pub struct CreateCompanyReq {
    pub company_id: i32,
    pub client_id: i64
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

async fn read_clients(filter: SearchFilter, pool: &PgPool) -> Result<Vec<Client>, Error> {
    sqlx::query_as!(Client,
    "
        SELECT client.*
        FROM client_in_company
        JOIN client ON client_in_company.client_id = client.id
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
}

async fn read_admins(filter: SearchFilter, pool: &PgPool) -> Result<Vec<Client>, Error> {
    sqlx::query_as!(Client,
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