#[derive(Debug)]
pub struct FileM8kr {
    pub path: String,
    with_dirs: bool,
}

impl FileM8kr {
    pub fn with_directories(&mut self) -> &FileM8kr {
        self.with_dirs = true;
        self
    }
}

pub fn create_file(path: &str) -> FileM8kr {
    FileM8kr {
        with_dirs: false,
        path: path.to_string(),
    }
}