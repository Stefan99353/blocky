use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum InstallationError {
    #[error("Version '{0}' is invalid")]
    Version(String),

    #[error("Error while parsing version data: {0}")]
    ParseVersionData(serde_json::Error),

    #[error("Version data is not installed for instance")]
    NoVersionData,

    #[error("Error while parsing asset index: {0}")]
    ParseAssetIndex(serde_json::Error),

    #[error("The provided name for a library is invalid")]
    LibraryNameFormat,

    #[error("Error while extracting file: {0}")]
    Extract(zip::result::ZipError),
}
