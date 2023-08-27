use serde::Deserialize;
use sqlx::{FromRow, PgPool, Error, postgres::PgQueryResult, PgConnection};
use super::{SearchFilter, PAGE_SIZE};

#[derive(Debug, FromRow)]
pub struct ItemOwnershipDisplay {
    pub id: i64,
    pub name: String,
    pub amount: i64,
    pub price: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ItemOwnership {
    pub company_id: i32,
    pub item_id: i64,
    pub amount: i64
}

pub async fn create(ownership: &ItemOwnership, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        INSERT INTO company_owns_item VALUES
        ($1, $2, $3)
    ",
    ownership.company_id,
    ownership.item_id,
    ownership.amount
    )
    .execute(pool)
    .await
}

pub async fn read(filter: SearchFilter, pool: &PgPool) -> Result<Vec<ItemOwnershipDisplay>, Error> {
    sqlx::query_as!(ItemOwnershipDisplay,"
        SELECT item.*, company_owns_item.amount
        FROM company_owns_item
        JOIN item ON company_owns_item.item_id = item.id
        WHERE company_owns_item.company_id = $1
        AND item.name ILIKE '%' || $2 || '%'
        ORDER BY company_owns_item.amount DESC
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

pub async fn update_replace(req: ItemOwnership, pool: &PgPool) -> Result<(), Error> {
    let mut txn = pool.begin().await?;

    sqlx::query!("
        UPDATE company_owns_item
        SET amount = $1
        WHERE company_id = $2 AND item_id = $3
    ",
    req.amount,
    req.company_id,
    req.item_id
    )
    .execute(&mut *txn)
    .await?;

    delete_zeroes(&mut *txn).await?;

    txn.commit().await
}

/// Negative numbers are for subtracting
pub async fn update_add(req: ItemOwnership, pool: &PgPool) -> Result<(), Error> {
    let mut txn = pool.begin().await?;

    sqlx::query!("
        UPDATE company_owns_item
        SET amount = amount + $1
        WHERE company_id = $2 AND item_id = $3
    ",
    req.amount,
    req.company_id,
    req.item_id
    )
    .execute(&mut *txn)
    .await?;

    delete_zeroes(&mut *txn).await?;

    txn.commit().await
}

async fn delete_zeroes(txn: &mut PgConnection) -> Result<PgQueryResult, Error> 
{
    sqlx::query!("
        DELETE FROM company_owns_item
        WHERE amount <= 0
    ")
    .execute(txn)
    .await
}

pub async fn delete(company_id: i32, item_id: i64, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!("
        DELETE FROM company_owns_item
        WHERE company_id = $1 AND item_id = $2
    ",
    company_id,
    item_id
    )
    .execute(pool)
    .await
}
