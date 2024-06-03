use libsql::Builder;

struct TursoDbHandler {
    db: libsql::Database,
    conn: libsql::Connection
}

impl TursoDbHandler {
    async fn new_main() -> Self {
        let url = dotenv::var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
        let token = dotenv::var("TURSO_AUTH_TOKEN").unwrap_or_default();
        let db = Builder::new_remote(url, token).build().await.unwrap();
        let conn = db.connect().unwrap();
        TursoDbHandler { db, conn }
    }

    async fn new_from_params(url: String, token: String) -> Self {
        let db = Builder::new_remote(url, token).build().await.unwrap();
        let conn = db.connect().unwrap();
        TursoDbHandler { db, conn }
    }

    async fn get_for_db(&self, db_name: String) -> TursoDbHandler {
        let mut rows = self.conn.query("SELECT url, token FROM databases WHERE name = '(:name)'", libsql::named_params! { ":name": db_name }).await.unwrap();

        let row = rows.next().await.unwrap().unwrap();
        let url = row.get_value(0).unwrap().as_text().unwrap().to_string();
        let token = row.get_value(1).unwrap().as_text().unwrap().to_string();
        TursoDbHandler::new_from_params(url, token).await
    }
    
    async fn create_for_db(&self, db_name: String) {
        let url = dotenv::var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
        let token = dotenv::var("TURSO_AUTH_TOKEN").unwrap_or_default();
        let db = Builder::new_remote(url, token).build().await.unwrap();
        let conn = db.connect().unwrap();
        conn.execute("INSERT INTO databases (name, url, token) VALUES (:name, :url, :token)", libsql::named_params! { ":name": db_name, ":url": url, ":token": token }).await.unwrap();
    }
}