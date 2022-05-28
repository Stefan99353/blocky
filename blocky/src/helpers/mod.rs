use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod error;
pub mod fabric;
pub mod instances;
pub mod profiles;
pub mod version_manifest;

pub fn read_file<T>(path: impl AsRef<Path>) -> anyhow::Result<T>
where
    T: Default + for<'de> Deserialize<'de>,
{
    let mut result = T::default();

    if path.as_ref().is_file() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        result = serde_json::from_reader(reader)?;
    }

    Ok(result)
}

pub fn write_file<T: Serialize>(value: &T, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(&file, value)?;
    file.sync_all()?;

    Ok(())
}
