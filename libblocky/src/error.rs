use crate::instance::install::error::InstallationError;
use crate::instance::launch::error::LaunchError;
use crate::profile::AuthenticationError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while interacting with filesystem: {0}")]
    Filesystem(std::io::Error),

    #[error("Error while getting resource from web: {0}")]
    Request(reqwest::Error),

    #[error("Error while serializing/deserializing struct: {0}")]
    Serde(serde_json::Error),

    #[error("AuthenticationError: {0}")]
    Authentication(AuthenticationError),

    #[error("InstallationError: {0}")]
    Installation(InstallationError),

    #[error("LaunchError: {0}")]
    Launch(LaunchError),

    #[error("The checksum does not match hash of file: {0}")]
    Sha1Mismatch(String),

    #[error("Provided SHA could not be decoded: {0}")]
    Sha1Decode(hex::FromHexError),

    #[error("Instance UUID '{0}' was not found on disk")]
    InstanceNotFound(uuid::Uuid),

    #[error("Profile UUID '{0}' was not found on disk")]
    ProfileNotFound(uuid::Uuid),
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

impl From<LaunchError> for Error {
    fn from(err: LaunchError) -> Self {
        Self::Launch(err)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Self::Sha1Decode(err)
    }
}
