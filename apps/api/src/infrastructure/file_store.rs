#[derive(Debug, Clone)]
pub struct FileStore {
    pub root_dir: String,
}

impl FileStore {
    pub fn new(root_dir: String) -> Self {
        Self { root_dir }
    }
}
