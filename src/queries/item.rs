use serde::Deserialize;
use sqlx::{FromRow, PgPool, Error, postgres::PgQueryResult};

#[derive(Debug, FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub price: Option<f32>,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CreateItemReq {
    pub name: String,
    pub price: Option<f32>,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemReq {
    pub id: i64,
    pub price: Option<f32>,
    pub description: Option<String>
}

pub async fn create(item: CreateItemReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO item VALUES
        (DEFAULT, $1, $2, $3)",
    item.name,
    item.price,
    item.description
    )
    .execute(pool)
    .await
}

pub async fn read_all(ids: Vec<i64>, pool: &PgPool, ) -> Result<Vec<Item>, Error> {
    sqlx::query_as!(Item,"
        SELECT * 
        FROM item
        WHERE id = ANY($1)",
    &ids[..]
    )
    .fetch_all(pool)
    .await
}

pub async fn read_one(id: i64, pool: &PgPool) -> Result<Option<Item>, Error> {
    sqlx::query_as!(Item,"
        SELECT * 
        FROM item
        WHERE id = $1",
    id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update(item: UpdateItemReq, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        UPDATE item
        SET price = $2, description = $3
        WHERE id = $1",
    item.id,
    item.price,
    item.description
    )
    .execute(pool)
    .await
}

pub async fn delete(id: i64, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        DELETE FROM item
        WHERE id = $1",
    id
    )
    .execute(pool)
    .await
}
