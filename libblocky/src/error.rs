use crate::profile::AuthenticationError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error while interacting with filesystem: {0}")]
    Filesystem(std::io::Error),

    #[error("Error while downloading file: {0}")]
    DownloadFile(reqwest::Error),

    #[error("{0}")]
    Authentication(AuthenticationError),
}

impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Self {
        Self::Authentication(err)
    }
}
