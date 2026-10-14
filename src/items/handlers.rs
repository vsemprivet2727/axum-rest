use crate::items::{repository, structs::*};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::*;

pub async fn get_items(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Item>>, axum::http::StatusCode> {
    match repository::select_items(&pool).await {
        Ok(items) => Ok(Json(items)),
        Err(err) => {
            eprintln!("db err {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, StatusCode> {
    repository::select_item(&pool, id)
        .await
        .map_err(|err| match err {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
        .map(Json)
}

pub async fn create_item(
    State(pool): State<SqlitePool>,
    Json(item): Json<CreateItem>,
) -> (StatusCode, Json<String>) {
    if item.name.trim().is_empty()
        || item.name.trim().contains("\t")
        || item.name.trim().contains("  ")
        || item.price < 0
    {
        (StatusCode::BAD_REQUEST, Json("check your input".into()))
    } else {
        match repository::insert_into_items(&pool, &item).await {
            Ok(_) => (StatusCode::CREATED, Json("created".into())),
            Err(err) => {
                eprintln!("db error: {err}");
                (StatusCode::INTERNAL_SERVER_ERROR, Json("db err".into()))
            }
        }
    }
}

pub async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<String>) {
    match repository::delete_from_items(&pool, id).await {
        Ok(result) if result.rows_affected() > 0 => (StatusCode::OK, Json("deleted".into())),
        Ok(_) => (StatusCode::NOT_FOUND, Json("item not found".into())),
        Err(err) => {
            eprintln!("db: error {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("database error".into()),
            )
        }
    }
}

pub async fn patch_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(patch): Json<PatchItem>,
) -> (StatusCode, Json<String>) {
    match repository::select_item(&pool, id).await {
        Ok(mut item) => {
            if let Some(name) = patch.name {
                item.name = name;
            }
            if let Some(price) = patch.price {
                item.price = price;
            }
            match repository::update_item(&pool, id, &item).await {
                Ok(result) if result.rows_affected() > 0 => {
                    (StatusCode::OK, Json("patched".into()))
                }
                Ok(_) => (StatusCode::NOT_FOUND, Json("item not found".into())),
                Err(err) => {
                    eprintln!("db error: {err}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json("database error".into()),
                    )
                }
            }
        }
        Err(err) => {
            eprintln!("db error: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("database error".into()),
            )
        }
    }
}
