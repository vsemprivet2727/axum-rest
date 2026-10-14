use sqlx::FromRow;
#[derive(FromRow, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub price: i64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateItem {
    pub name: String,
    pub price: i64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PatchItem {
    pub name: Option<String>,
    pub price: Option<i64>,
}
