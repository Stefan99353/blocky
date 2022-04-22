use super::error::InstallationError;
use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::models::VersionData;
use crate::instance::utils::version_manifest;
use crate::Instance;
use std::fs;
use std::path::PathBuf;

impl Instance {
    pub fn install_version_data(&self) -> Result<(), Error> {
        debug!("Installing version data");

        // Get version manifest
        let manifest = version_manifest().map_err(Error::Request)?;

        // Figure out version
        let version_summary = match &self.version {
            None => manifest
                .versions
                .get(&manifest.latest.release)
                .ok_or_else(|| InstallationError::Version(manifest.latest.release.clone()))?,
            Some(version) => manifest
                .versions
                .get(version)
                .ok_or_else(|| InstallationError::Version(version.clone()))?,
        };
        debug!(
            "Downloading version data for version '{}'",
            &version_summary.id
        );

        // Create instance folder
        let mut instance_path = PathBuf::from(&self.instance_path);
        fs::create_dir_all(&instance_path).map_err(Error::Filesystem)?;

        // Save version data
        let mut version_data_path = instance_path;
        version_data_path.push("version.json");
        download_file_check(&version_summary.url, &version_data_path, None)?;

        Ok(())
    }

    pub fn read_version_data(&self) -> Result<VersionData, Error> {
        let mut version_data_path = PathBuf::from(&self.instance_path);
        version_data_path.push("version.json");
        let version_data = fs::read_to_string(&version_data_path).map_err(Error::Filesystem)?;
        let version_data = serde_json::from_str::<VersionData>(&version_data)
            .map_err(InstallationError::ParseVersionData)?;

        Ok(version_data)
    }
}
