pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl From<libsql::Row> for User {
    fn from(row: libsql::Row) -> Self {
        Self {
            firstname: row.get_value(0).unwrap().as_text().unwrap().to_string(),
            lastname: row.get_value(1).unwrap().as_text().unwrap().to_string(),
            email: row.get_value(2).unwrap().as_text().unwrap().to_string(),
        }
    }
}