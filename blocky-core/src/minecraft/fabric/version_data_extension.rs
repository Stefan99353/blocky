use crate::minecraft::error::MinecraftError;
use crate::minecraft::models::arguments::Arguments;
use crate::minecraft::models::version_type::VersionType;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FabricVersionDataExtension {
    pub id: String,
    #[serde(alias = "inheritsFrom")]
    pub inherits_from: String,
    #[serde(alias = "mainClass")]
    pub main_class: String,
    pub arguments: Option<Arguments>,
    pub libraries: Vec<FabricLibrary>,
    #[serde(alias = "releaseTime")]
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub _type: VersionType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FabricLibrary {
    pub name: String,
    pub url: String,
}

impl FabricLibrary {
    pub fn extract_information(&self) -> Result<(String, String, String), MinecraftError> {
        self.name
            .split(':')
            .map(|x| x.to_string())
            .collect_tuple()
            .ok_or(MinecraftError::LibraryNameFormat)
    }
}
