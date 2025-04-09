pub struct DbConfig {
    database: String,
}

impl DbConfig {
    pub fn new(database: &str) -> Self {
        Self {
            database: database.to_owned(),
        }
    }

    pub fn get_database(&self) -> &str {
        &self.database
    }
}