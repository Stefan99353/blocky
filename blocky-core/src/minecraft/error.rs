use crate::error::DownloadError;

#[derive(Debug, thiserror::Error)]
pub enum MinecraftError {
    #[error("The provided name for a library is invalid")]
    LibraryNameFormat,

    #[error("Failed to fork new process")]
    Forking,

    #[error("{0}")]
    IO(std::io::Error),

    #[error("{0}")]
    Sha1Decode(hex::FromHexError),

    #[error("{0}")]
    Download(DownloadError),

    #[error("{0}")]
    Extract(zip::result::ZipError),
}
