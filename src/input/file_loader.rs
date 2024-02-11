use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct FileLoader {
    path: PathBuf,
}

impl FileLoader {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { path: path.into() }
    }
}

impl super::Loader for FileLoader {
    fn load(&self, day: u16) -> Result<String> {
        let file_name = format!("day{}.txt", day);
        let full_path = self.path.join(file_name);
        std::fs::read_to_string(full_path).context("could not read input file")
    }
}
