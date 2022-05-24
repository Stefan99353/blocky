use crate::minecraft::error::MinecraftError;
use crate::profile::error::AuthenticationError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Authentication(AuthenticationError),

    #[error("{0}")]
    Minecraft(MinecraftError),

    #[error("Version '{0}' is invalid")]
    Version(String),

    #[error("{0}")]
    IO(std::io::Error),

    #[error("{0}")]
    Serde(serde_json::Error),

    #[error("{0}")]
    Sha1Decode(hex::FromHexError),

    #[error("{0}")]
    Download(DownloadError),
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("The checksum does not match hash of file '{0}'")]
    Sha1Mismatch(String),

    #[error("{0}")]
    Reqwest(reqwest::Error),

    #[error("{0}")]
    IO(std::io::Error),
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Self {
        Self::Authentication(err)
    }
}

impl From<MinecraftError> for Error {
    fn from(err: MinecraftError) -> Self {
        Self::Minecraft(err)
    }
}

impl From<DownloadError> for Error {
    fn from(err: DownloadError) -> Self {
        Self::Download(err)
    }
}

// impl From<InstallationError> for Error {
//     fn from(err: InstallationError) -> Self {
//         Self::Installation(err)
//     }
// }
//
// impl From<LaunchError> for Error {
//     fn from(err: LaunchError) -> Self {
//         Self::Launch(err)
//     }
// }
//
// impl From<hex::FromHexError> for Error {
//     fn from(err: hex::FromHexError) -> Self {
//         Self::Sha1Decode(err)
//     }
// }
