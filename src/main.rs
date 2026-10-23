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

pub fn create_router(pool: SqlitePool) -> Router {
    items::routes(pool)
}

pub async fn create_pool(max_conn: u32, connect_str: &str) -> Result<SqlitePool, sqlx::Error> {
    sqlite::SqlitePoolOptions::new()
        .max_connections(max_conn)
        .connect(&connect_str)
        .await
}
