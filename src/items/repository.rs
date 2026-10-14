use crate::items::structs::*;
use sqlx::{Error, SqlitePool, query, query_as, sqlite::SqliteQueryResult};

pub async fn select_items(pool: &SqlitePool) -> Result<Vec<Item>, Error> {
    query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool)
        .await
}

pub async fn select_item(pool: &SqlitePool, id: i64) -> Result<Item, Error> {
    query_as("SELECT * FROM items WHERE id=?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::RowNotFound)
}

pub async fn delete_from_items(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, Error> {
    query("DELETE FROM items WHERE id=?")
        .bind(id)
        .execute(pool)
        .await
}

pub async fn insert_into_items(
    pool: &SqlitePool,
    item: &CreateItem,
) -> Result<SqliteQueryResult, Error> {
    query("INSERT INTO items (name, price) VALUES (?, ?)")
        .bind(&item.name)
        .bind(item.price)
        .execute(pool)
        .await
}

pub async fn update_item(
    pool: &SqlitePool,
    id: i64,
    item: &Item,
) -> Result<SqliteQueryResult, Error> {
    query("UPDATE items SET name=?, price=? WHERE id=?")
        .bind(&item.name)
        .bind(item.price)
        .bind(id)
        .execute(pool)
        .await
}
