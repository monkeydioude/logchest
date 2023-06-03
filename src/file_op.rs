use std::{
    fs::{create_dir_all, File, read_to_string},
    io::{Error, Write},
    path::Path,
};

#[derive(Debug)]
pub struct FileM8kr {
    pub path: String,
    with_dirs: bool,
    file: Option<File>,
}

impl FileM8kr {
    pub fn with_directories(&mut self) -> &mut Self {
        self.with_dirs = true;
        self
    }

    pub fn go_my_dude(&mut self) -> Result<(), Error> {
        if let Some(parent) = Path::new(&self.path).parent() {
            create_dir_all(parent)?;
        }
        let f = File::options()
            .append(true)
            .create(true)
            .open(&self.path)?;
        self.file = Some(f);
        Ok(())
    }

    pub fn write_line(&mut self, line: &str) -> Result<(), Error> {
        let f = self.file.as_mut().unwrap();
        f.write_all((line.to_string() + "\n").as_bytes())?;
        Ok(())
    }
}

pub fn create_file(path: &str) -> FileM8kr {
    FileM8kr {
        with_dirs: false,
        path: path.to_string(),
        file: None,
    }
}

pub fn read_lines<T>(path: &str, transformer: fn((usize, &str)) -> T) -> Vec<T> {
    read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(transformer)
        .collect()
}