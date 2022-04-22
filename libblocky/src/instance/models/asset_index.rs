use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    #[serde(alias = "totalSize")]
    pub total_size: i64,
    pub url: String,
}
