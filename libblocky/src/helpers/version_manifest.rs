use super::HelperError;
use crate::instance::models::VersionSummary;
use crate::instance::utils::version_manifest;
use std::collections::HashMap;

pub fn get_manifest() -> Result<HashMap<String, VersionSummary>, HelperError> {
    debug!("Getting version manifest from Mojang");
    let manifest = version_manifest()?;

    Ok(manifest.versions)
}
