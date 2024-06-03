mod templates;
mod turso_api;
mod turso_db_handler;

use dotenv;

use askama_axum::IntoResponse;
use axum::{extract::Path, routing::get, Router};
use templates::IndexTemplate;
use turso_api::ensure_created;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/app/:db", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> impl IntoResponse {
    let key = "NAME";
    let name = dotenv::var(key).unwrap_or_else(|_| "stranger".to_string());
    IndexTemplate { name }.into_response()
}

async fn handler(Path(db): Path<String>) -> impl IntoResponse {
    let db_name = "micrm_".to_owned() + &db;
    ensure_created(db_name).await
}

