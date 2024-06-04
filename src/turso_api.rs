use anyhow::{Result, Error};
use crate::turso_db_handler::TursoDbHandler;

pub async fn check_if_exists(db_name: String) -> bool {
    let resp = get_databases().await;
    let json: serde_json::Value = serde_json::from_str(&resp).unwrap();
    for i in json.get("databases").unwrap().as_array().unwrap() {
        if i.get("Name").unwrap().as_str().unwrap() == db_name.as_str() {
            return true;
        }
    }
    false
}

pub async fn ensure_created(db_name: String, main_db_handler: &TursoDbHandler) {
    if !check_if_exists(db_name.clone()).await {
        let token = create_database(db_name.clone()).await;
        let db_url = get_database_url(db_name.clone()).await.unwrap();
        main_db_handler.create_for_db(db_name, db_url, token).await;
    }
}

async fn get_databases() -> String {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "https://api.turso.tech/v1/organizations/{}/databases",
            dotenv::var("TURSO_ORGANIZATION").unwrap()
        ))
        .header(
            "Authorization",
            format!("Bearer {}", dotenv::var("TURSO_BEARER").unwrap()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Request Failed")
        .text()
        .await
        .expect("Parsing Error");
    resp
}

async fn create_database(db_name: String) -> String {
    let client = reqwest::Client::new();
    let json = serde_json::to_string(&serde_json::json!({
        "name": db_name,
        "group": "default"
    })).unwrap();
    println!("json: {}", json);
    let resp = client
        .post(format!(
            "https://api.turso.tech/v1/organizations/{}/databases",
            dotenv::var("TURSO_ORGANIZATION").unwrap()
        ))
        .header(
            "Authorization",
            format!("Bearer {}", dotenv::var("TURSO_BEARER").unwrap()),
        )
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await
        .expect("Request Failed");

    let json = client
        .post(format!(
            "https://api.turso.tech/v1/organizations/{}/databases/{}/auth/tokens",
            dotenv::var("TURSO_ORGANIZATION").unwrap(),
            db_name
        ))
        .header(
            "Authorization",
            format!("Bearer {}", dotenv::var("TURSO_BEARER").unwrap()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Request Failed")
        .text()
        .await
        .expect("Parsing Error");

    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    json.get("jwt").unwrap().as_str().unwrap().to_string()
}

async fn get_database_url(db_name: String) -> Result<String> {
    let resp = get_databases().await;
    let json: serde_json::Value = serde_json::from_str(&resp).unwrap();
    for i in json.get("databases").unwrap().as_array().unwrap() {
        if i.get("Name").unwrap().as_str().unwrap() == db_name.as_str() {
            return Ok(i.get("Hostname").unwrap().as_str().unwrap().to_string());
        }
    }
    Err(anyhow::anyhow!("Database not found"))
}
