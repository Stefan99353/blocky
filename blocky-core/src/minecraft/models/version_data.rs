use crate::minecraft::models::arguments::Arguments;
use crate::minecraft::models::asset_index::AssetIndex;
use crate::minecraft::models::downloads::Downloads;
use crate::minecraft::models::library::Library;
use crate::minecraft::models::logging_info::LoggingInfo;
use crate::minecraft::models::version_type::VersionType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionData {
    pub arguments: Option<Arguments>,
    #[serde(alias = "assetIndex")]
    pub asset_index: Option<AssetIndex>,
    pub assets: String,
    #[serde(alias = "complianceLevel")]
    pub compliance_level: i32,
    pub downloads: Option<Downloads>,
    pub id: String,
    // TODO: properly deserialize
    #[serde(alias = "javaVersion")]
    pub java_version: serde_json::Value,
    pub libraries: Vec<Library>,
    pub logging: Option<LoggingInfo>,
    #[serde(alias = "mainClass")]
    pub main_class: String,
    #[serde(alias = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
    #[serde(alias = "minimumLauncherVersion")]
    pub minimum_launcher_version: i32,
    #[serde(alias = "releaseTime")]
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    #[serde(rename = "type")]
    pub _type: VersionType,
}

impl VersionData {
    pub fn needed_libraries(&self) -> Vec<&Library> {
        self.libraries
            .iter()
            .filter(|library| library.check_use())
            .collect()
    }
}
