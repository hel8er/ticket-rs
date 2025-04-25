mod db;
mod routes;

use axum::Router;
// use tower_http::fs::ServeDir;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let app = Router::new().merge(routes::routes(pool.clone()));
    // .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
