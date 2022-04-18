use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Failed to create TCP listener for oauth redirect url")]
    TcpListener(std::io::Error),

    #[error("Failed to parse OAuth response")]
    OAuthResponse,
    #[error("Failed to exchange authorization code for token")]
    CodeExchange,

    #[error("{0:?} token expired")]
    ExpiredToken(TokenKind),
    #[error("{0:?} not authenticated")]
    NotAuthenticated(TokenKind),

    #[error("No user hash was provided during authentication")]
    UserHash,

    #[error("Entitlements missing")]
    MissingEntitlements,
    #[error("Account does not own the game")]
    NotEntitled,

    #[error("Failed to send/receive/parse request: {0}")]
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for AuthenticationError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Microsoft,
    XboxLive,
    XboxLiveSecret,
    Minecraft,
}
