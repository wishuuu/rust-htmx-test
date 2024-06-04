use libsql::Builder;

pub struct TursoDbHandler {
    db: libsql::Database,
    pub conn: libsql::Connection,
}

impl TursoDbHandler {
    pub async fn new_main() -> Self {
        let url = dotenv::var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
        let token = dotenv::var("TURSO_AUTH_TOKEN").unwrap_or_default();
        let db = Builder::new_remote(url, token).build().await.unwrap();
        let conn = db.connect().unwrap();
        TursoDbHandler { db, conn }
    }

    pub async fn new_from_params(url: String, token: String) -> Self {
        let db = Builder::new_remote(url, token).build().await.unwrap();
        let conn = db.connect().unwrap();
        TursoDbHandler { db, conn }
    }

    pub async fn get_for_db(&self, db_name: String) -> TursoDbHandler {
        let mut rows = self
            .conn
            .query(
                "SELECT url, token FROM databases WHERE name = ?1",
                libsql::params![db_name],
            )
            .await
            .unwrap();

        let row = rows.next().await.unwrap().unwrap();
        let url = row.get_value(0).unwrap().as_text().unwrap().to_string();
        let token = row.get_value(1).unwrap().as_text().unwrap().to_string();
        TursoDbHandler::new_from_params(url, token).await
    }

    pub async fn create_for_db(&self, db_name: String, created_url: String, created_token: String) {
        let url = dotenv::var("TURSO_DATABASE_URL").expect("LIBSQL_URL must be set");
        let token = dotenv::var("TURSO_AUTH_TOKEN").unwrap_or_default();
        let db = Builder::new_remote(url.clone(), token.clone())
            .build()
            .await
            .unwrap();
        let conn = db.connect().unwrap();
        conn.execute(
            "INSERT INTO databases (name, url, token) VALUES (?1, ?2, ?3)",
            libsql::params![db_name.clone(), created_url, created_token],
        )
        .await
        .unwrap();

        self.get_for_db(db_name).await.perform_migration().await;
    }

    async fn perform_migration(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS users (FIRSTNAME TEXT, LASTNAME TEXT, EMAIL TEXT)",
                (),
            )
            .await
            .unwrap();
    }
}
