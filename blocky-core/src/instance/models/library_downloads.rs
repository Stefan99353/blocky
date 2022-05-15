use super::Artifact;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    // TODO: propery deserialize
    pub classifiers: Option<serde_json::Value>,
}
