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

pub async fn ensure_created(db_name: String) {
    todo!();
    if !check_if_exists(db_name.clone()).await {
    }
}

async fn get_databases() -> String {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.turso.tech/v1/organizations/{}/databases", dotenv::var("TURSO_ORGANIZATION").unwrap()))
        .header("Authorization", format!("Bearer {}", dotenv::var("TURSO_BEARER").unwrap()))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Request Failed")
        .text()
        .await
        .expect("Parsing Error");
    resp
}