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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub url: String,
}
