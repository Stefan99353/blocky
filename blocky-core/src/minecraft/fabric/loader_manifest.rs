use crate::consts;
use crate::error::DownloadError;
use crate::minecraft::error::MinecraftError;
use serde::{Deserialize, Serialize};

impl FabricManifestEntry {
    pub fn get_versions(game_version: &str) -> Result<Vec<Self>, MinecraftError> {
        let manifest_url = format!(
            "{}/versions/loader/{}",
            consts::FABRIC_BASE_V2_URL,
            game_version
        );

        reqwest::blocking::Client::new()
            .get(manifest_url)
            .send()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))?
            .error_for_status()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))?
            .json::<Vec<Self>>()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FabricManifestEntry {
    pub loader: FabricLoaderSummary,
    pub intermediary: FabricIntermediarySummary,
    #[serde(alias = "launcherMeta")]
    pub launcher_meta: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FabricLoaderSummary {
    pub separator: String,
    pub build: i32,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

impl FabricLoaderSummary {
    pub fn get(game_version: &str) -> Result<Vec<Self>, MinecraftError> {
        let manifest = FabricManifestEntry::get_versions(game_version)?
            .into_iter()
            .map(|m| m.loader)
            .collect();

        Ok(manifest)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FabricIntermediarySummary {
    pub maven: String,
    pub version: String,
    pub stable: bool,
}
