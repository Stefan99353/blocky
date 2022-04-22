use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: i64,
    pub url: String,
}
