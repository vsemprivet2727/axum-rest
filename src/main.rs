mod items;
use axum::Router;
use sqlx::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = create_pool(5, "sqlite:database.db").await?;
    let app = create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn create_router(pool: SqlitePool) -> Router {
    items::routes(pool)
}

async fn create_pool(max_conn: u32, connect_str: &str) -> Result<SqlitePool, sqlx::Error> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(&connect_str)
        .await
}

#[tokio::test]
async fn test_query() -> Result<(), sqlx::Error> {
    let pool = create_pool(1, "sqlite::memory:").await?;
    query(
        "CREATE TABLE items_test ( \
        id INTEGER PRIMARY KEY AUTOINCREMENT,\
        name TEXT,\
        price INTEGER);",
    )
    .execute(&pool)
    .await?;
    Ok(())
}
