use sqlx::{FromRow, PgPool, Error, postgres::PgQueryResult};

#[derive(Debug, FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub price: Option<f32>,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct CreateItem {
    pub name: String,
    pub price: Option<f32>,
    pub description: Option<String>
}

pub async fn create(item: CreateItem, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO item VALUES
        (DEFAULT, $1, $2, $3)
    ",
    item.name,
    item.price,
    item.description
    )
    .execute(pool)
    .await
}

pub async fn read(ids: Vec<i64>, pool: &PgPool) -> Result<Vec<Item>, Error> {
    sqlx::query_as!(Item,
    "
        SELECT * 
        FROM item
        WHERE id = ANY($1)
    ",
    &ids[..]
    )
    .fetch_all(pool)
    .await
}
