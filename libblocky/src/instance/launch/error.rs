use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("Minecraft Profile is missing")]
    MissingProfile,

    #[error("Minecraft token is missing")]
    Unauthenticated,
}
