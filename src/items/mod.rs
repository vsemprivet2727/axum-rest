pub mod handlers;
pub mod repository;
pub mod structs;

use axum::{Router, routing::get};
use sqlx::SqlitePool;

pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/items",
            get(handlers::get_items).post(handlers::create_item),
        )
        .route(
            "/items/{id}",
            get(handlers::get_item)
                .delete(handlers::delete_item)
                .patch(handlers::patch_item),
        )
        .with_state(pool)
}
