use axum::{
    Router,
    extract::{Form, State},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateItem {
    name: String,
}

pub async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    // Using sqlx::query instead of sqlx::query! to avoid compile-time validation
    // that requires DATABASE_URL or cargo sqlx prepare
    use sqlx::Row;

    let rows = sqlx::query("SELECT id, name FROM items")
        .fetch_all(&pool)
        .await
        .unwrap();

    let body = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            format!("<li>{}</li>", name)
        })
        .collect::<String>();

    Html(format!(
        include_str!("../templates/index.html"),
        items = body
    ))
}

pub async fn add_item(
    State(pool): State<SqlitePool>,
    Form(input): Form<CreateItem>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO items (id, name) VALUES (?, ?)")
        .bind(id)
        .bind(&input.name)
        .execute(&pool)
        .await
        .unwrap();

    Html(format!("<li>{}</li>", input.name))
}

pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/add", post(add_item))
        .with_state(pool)
}
