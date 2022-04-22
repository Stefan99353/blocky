use super::VersionSummary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: HashMap<String, VersionSummary>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionManifestResponse {
    pub latest: LatestVersion,
    pub versions: Vec<VersionSummary>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}
