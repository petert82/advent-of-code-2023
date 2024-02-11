use anyhow::Result;

pub mod file_loader;

pub trait Loader {
    fn load(&self, day: u16) -> Result<String>;
}
