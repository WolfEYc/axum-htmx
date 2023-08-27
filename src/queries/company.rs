use serde::Deserialize;
use sqlx::{FromRow, PgPool, Error, postgres::PgQueryResult};

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyReq {
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompanyReq {
    pub id: i32,
    pub description: Option<String>
}

pub async fn create(company: &CreateCompanyReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO company VALUES
        (DEFAULT, $1, $2)",
    company.name,
    company.description
    )
    .execute(pool)
    .await
}

pub async fn read_all(ids: Vec<i32>, pool: &PgPool, ) -> Result<Vec<Company>, Error> {
    sqlx::query_as!(Company,"
        SELECT * 
        FROM company
        WHERE id = ANY($1)",
    &ids[..]
    )
    .fetch_all(pool)
    .await
}

pub async fn read_one(id: i32, pool: &PgPool) -> Result<Company, Error> {
    sqlx::query_as!(Company,"
        SELECT *
        FROM company
        WHERE id = $1
    ",
    id
    )
    .fetch_one(pool)
    .await
}

pub async fn is_verified(id: i32, pool: &PgPool) -> Result<bool, Error> {
    Ok(sqlx::query!("
        SELECT 1 as verified
        FROM verified_company
        WHERE company_id = $1
    ",
    id
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}

pub async fn update(company: &Company, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        UPDATE company
        SET description = $2
        WHERE id = $1",
    company.id,
    company.description
    )
    .execute(pool)
    .await
}

pub async fn delete(id: i32, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        DELETE FROM company
        WHERE id = $1",
    id
    )
    .execute(pool)
    .await
}
