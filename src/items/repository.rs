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

#[cfg(test)]
pub mod tests {
    use crate::create_pool;
    use crate::items::repository::*;
    use sqlx::*;
    pub fn create_test_item() -> Item {
        Item {
            id: 1,
            name: String::from("test"),
            price: 200,
        }
    }
    pub async fn create_test_pool() -> Result<SqlitePool, Error> {
        let test_item = create_test_item();
        let pool = create_pool(5, "sqlite::memory:").await?;
        query(
            "CREATE TABLE IF NOT EXISTS items ( \
            id INTEGER PRIMARY KEY AUTOINCREMENT,\
            name TEXT,\
            price INTEGER);",
        )
        .execute(&pool)
        .await?;
        query(" INSERT OR IGNORE INTO items (name,price) VALUES (?,?)")
            .bind(&test_item.name)
            .bind(test_item.price)
            .execute(&pool)
            .await?;
        Ok(pool)
    }
    #[tokio::test]
    async fn test_select_items() -> Result<(), Error> {
        let test_item = create_test_item();
        let pool = create_test_pool().await?;

        let items = select_items(&pool).await?;

        assert_eq!(items.len(), 1);
        assert_eq!(&items[0].name, &test_item.name);
        assert_eq!(items[0].price, test_item.price);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_item() -> Result<(), Error> {
        let test_item = create_test_item();
        let pool = create_test_pool().await?;

        let item = select_item(&pool, test_item.id).await?;

        assert_eq!(item.id, test_item.id);
        assert_eq!(&item.name, &test_item.name);
        assert_eq!(item.price, test_item.price);

        Ok(())
    }
    #[tokio::test]
    async fn test_rownotfound() -> Result<(), Error> {
        let pool = create_test_pool().await?;
        match select_item(&pool, 999).await {
            Ok(_) => Err(Error::PoolClosed),
            Err(_) => Ok(()),
        }
    }
}
