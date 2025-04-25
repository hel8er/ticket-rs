use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    // Explicitly create the database file if it doesn't exist
    let pool = SqlitePool::connect("sqlite://database.db?mode=rwc")
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database connection failed: {}", e);
            panic!("Cannot connect to database");
        });

    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS items (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL
    )
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
