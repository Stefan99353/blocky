use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum VersionType {
    #[serde(alias = "release")]
    Release,
    #[serde(alias = "snapshot")]
    Snapshot,
    #[serde(alias = "old_alpha")]
    OldAlpha,
    #[serde(alias = "old_beta")]
    OldBeta,
}

impl Display for VersionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionType::Release => write!(f, "release"),
            VersionType::Snapshot => write!(f, "snapshot"),
            VersionType::OldAlpha => write!(f, "old_alpha"),
            VersionType::OldBeta => write!(f, "old_beta"),
        }
    }
}
