use crate::instance::install::error::InstallationError;
use crate::profile::AuthenticationError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while interacting with filesystem: {0}")]
    Filesystem(std::io::Error),

    #[error("Error while getting resource from web: {0}")]
    Request(reqwest::Error),

    #[error("AuthenticationError: {0}")]
    Authentication(AuthenticationError),

    #[error("InstallationError: {0}")]
    Installation(InstallationError),

    #[error("The checksum does not match hash of file: {0}")]
    Sha1Mismatch(String),

    #[error("Provided SHA could not be decoded: {0}")]
    Sha1Decode(hex::FromHexError),
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Self {
        Self::Authentication(err)
    }
}

impl From<InstallationError> for Error {
    fn from(err: InstallationError) -> Self {
        Self::Installation(err)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Self::Sha1Decode(err)
    }
}
