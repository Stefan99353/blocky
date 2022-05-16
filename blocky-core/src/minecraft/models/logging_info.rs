use crate::minecraft::models::file::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoggingInfo {
    pub client: Client,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Client {
    pub argument: String,
    pub file: File,
    #[serde(rename = "type")]
    pub _type: String,
}
