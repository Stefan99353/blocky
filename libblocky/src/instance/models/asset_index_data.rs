use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndexData {
    #[serde(default)]
    pub map_to_resources: bool,
    pub objects: HashMap<String, AssetInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetInfo {
    pub hash: String,
    pub size: usize,
}
