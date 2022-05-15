use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub sha1: String,
    pub size: usize,
    pub url: String,
}
