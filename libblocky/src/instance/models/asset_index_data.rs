use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndexData {
    pub objects: HashMap<String, AssetInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetInfo {
    pub hash: String,
    pub size: usize,
}
