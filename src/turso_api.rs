pub async fn ensure_created(db_name: String) -> String {
    let resp_result = reqwest::get("https://api.turso.tech/v1/organizations/wishuuu/databases").await;
    let resp = match resp_result {
        Ok(r) => r.text().await.expect("Parsing Error"),
        _ => panic!("Network error")
    };
    return resp;
}
