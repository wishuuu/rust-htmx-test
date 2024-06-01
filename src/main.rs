mod templates;

use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use templates::IndexTemplate;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> impl IntoResponse {
    IndexTemplate{ name: "Stranger".to_string() }.into_response()
}
