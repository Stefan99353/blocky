use crate::minecraft::models::file::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LibraryDownloads {
    pub artifact: File,
    // TODO: properly deserialize
    pub classifiers: Option<serde_json::Value>,
}
