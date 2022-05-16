use blocky_core::minecraft::models::version_manifest::VersionManifest;
use blocky_core::minecraft::models::version_summary::VersionSummary;
use std::collections::HashMap;

pub fn get_manifest() -> anyhow::Result<HashMap<String, VersionSummary>> {
    debug!("Getting version manifest from Mojang");
    let manifest = VersionManifest::get()?;

    Ok(manifest.versions)
}
