#[derive(Debug, Clone)]
pub struct SqliteStore {
    pub connection_string: String,
}

impl SqliteStore {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}
