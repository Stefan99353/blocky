use crate::consts;
use crate::error::DownloadError;
use crate::minecraft::error::MinecraftError;
use crate::minecraft::models::version_summary::VersionSummary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: HashMap<String, VersionSummary>,
}

impl VersionManifest {
    pub fn get() -> Result<Self, MinecraftError> {
        let response = reqwest::blocking::Client::new()
            .get(consts::MC_VERSION_MANIFEST_URL)
            .send()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))?
            .error_for_status()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))?
            .json::<VersionManifestResponse>()
            .map_err(|err| MinecraftError::Download(DownloadError::Reqwest(err)))?;

        Ok(response.into())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct VersionManifestResponse {
    pub latest: LatestVersion,
    pub versions: Vec<VersionSummary>,
}

impl From<VersionManifestResponse> for VersionManifest {
    fn from(manifest: VersionManifestResponse) -> Self {
        let mut versions = HashMap::new();

        for version in manifest.versions {
            versions.insert(version.id.clone(), version);
        }

        Self {
            latest: manifest.latest,
            versions,
        }
    }
}
