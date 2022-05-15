use crate::instance::models::VersionSummary;
use crate::instance::utils::version_manifest;
use std::collections::HashMap;

pub fn get_manifest() -> crate::error::Result<HashMap<String, VersionSummary>> {
    debug!("Getting version manifest from Mojang");
    let manifest = version_manifest().map_err(crate::error::Error::Request)?;

    Ok(manifest.versions)
}
