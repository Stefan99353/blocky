use super::models::{VersionManifest, VersionManifestResponse};
use crate::consts;

// TODO: Cache version manifest
pub fn version_manifest() -> Result<VersionManifest, reqwest::Error> {
    let response = reqwest::blocking::Client::new()
        .get(consts::MC_VERSION_MANIFEST_URL)
        .send()?
        .error_for_status()?
        .json::<VersionManifestResponse>()?;

    Ok(response.into())
}
