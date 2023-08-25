use sqlx::{FromRow, PgPool, Error, postgres::PgQueryResult};

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct CreateCompanyReq {
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct UpdateCompanyReq {
    pub id: i32,
    pub name: Option<String>,
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

pub async fn read_one(id: i32, pool: &PgPool) -> Result<Option<Company>, Error> {
    sqlx::query_as!(Company,"
        SELECT * 
        FROM company
        WHERE id = $1",
    id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update(company: &UpdateCompanyReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        UPDATE company
        SET name = $2, description = $3
        WHERE id = $1",
    company.id,
    company.name,
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
