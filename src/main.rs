mod templates;
mod turso_api;
mod turso_db_handler;
mod users;

use dotenv;

use crate::users::User;
use askama_axum::IntoResponse;
use axum::{
    extract,
    extract::Path,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use templates::IndexTemplate;
use turso_api::ensure_created;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/app/:db", get(get_handler))
        .route("/app/:db", post(post_handler));

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

async fn get_handler(Path(db): Path<String>) -> impl IntoResponse {
    let db_name = "micrm-".to_owned() + &db;
    let main_db_handler = turso_db_handler::TursoDbHandler::new_main().await;
    ensure_created(db_name.clone(), &main_db_handler).await;
    let db_handler = main_db_handler.get_for_db(db_name).await;

    let mut rows = db_handler
        .conn
        .query("SELECT FIRSTNAME, LASTNAME, EMAIL FROM users", ())
        .await
        .unwrap();

    let mut users: Vec<User> = vec![];
    while let Some(row) = rows.next().await.unwrap() {
        users.push(User::from(row));
    }

    let template = templates::AppTemplate { name: db, users };
    template.into_response()
}

async fn post_handler(
    Path(db): Path<String>,
    extract::Json(payload): extract::Json<AddUser>,
) -> impl IntoResponse {
    let db_name = "micrm-".to_owned() + &db;
    let main_db_handler = turso_db_handler::TursoDbHandler::new_main().await;
    ensure_created(db_name.clone(), &main_db_handler).await;
    let db_handler = main_db_handler.get_for_db(db_name).await;

    db_handler
        .conn
        .execute(
            "INSERT INTO users (FIRSTNAME, LASTNAME, EMAIL) VALUES (?1, ?2, ?3)",
            libsql::params![payload.firstname, payload.lastname, payload.email],
        )
        .await
        .unwrap();

    let mut rows = db_handler
        .conn
        .query("SELECT FIRSTNAME, LASTNAME, EMAIL FROM users", ())
        .await
        .unwrap();

    let mut users: Vec<User> = vec![];
    while let Some(row) = rows.next().await.unwrap() {
        users.push(User::from(row));
    }

    let template = templates::ListTemplate { users };
    template.into_response()
}

#[derive(Deserialize)]
pub struct AddUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}
