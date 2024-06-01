mod templates;
mod turso_api;

use dotenv;

use askama_axum::IntoResponse;
use axum::{extract::Path, routing::get, Router};
use templates::IndexTemplate;
use turso_api::ensure_created;

use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let app = Router::new()
        .route("/", get(root))
        .route("/:db", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> impl IntoResponse {
    let key = "NAME";
    let name = match dotenv::var(key) {
        Ok(val) => val,
        Err(_) => "strager".to_string(),
    };
    IndexTemplate { name }.into_response()
}

async fn handler(Path(db): Path<String>) -> impl IntoResponse {
    let db_name = "micrm_".to_owned() + &db;
    println!("Database name: {}", db_name.clone());
    println!("Response: {}", ensure_created(db_name.clone()).await);
}

