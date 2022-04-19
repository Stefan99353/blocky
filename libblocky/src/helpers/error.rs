#[derive(Debug, thiserror::Error)]
pub enum HelperError {
    #[error("Error while reading/writing file: {0}")]
    IO(std::io::Error),

    #[error("Error while serializing/deserializing data: {0}")]
    Serde(serde_json::Error),
}

impl From<std::io::Error> for HelperError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<serde_json::Error> for HelperError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}
