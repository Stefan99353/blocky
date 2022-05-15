#[derive(Debug, thiserror::Error)]
pub enum HelperError {
    #[error("Error while reading/writing file: {0}")]
    IO(std::io::Error),

    #[error("Error while serializing/deserializing data: {0}")]
    Serde(serde_json::Error),

    #[error("Error while getting resource from web: {0}")]
    Request(reqwest::Error),
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

impl From<reqwest::Error> for HelperError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}
