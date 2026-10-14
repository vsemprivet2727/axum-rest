mod items;
use sqlx::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db")
        .await?;

    let app = items::routes(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
