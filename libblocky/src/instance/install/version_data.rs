use super::error::InstallationError;
use crate::consts;
use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::models::VersionData;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::instance::utils::version_manifest;
use crate::Instance;
use std::fs;
use std::path::PathBuf;

impl Instance {
    pub fn install_version_data(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
    ) -> Result<(), Error> {
        debug!("Installing version data");

        // Get version manifest
        trace!("Downloading version manifest");
        let _ = sender.send(Ok(ResourceInstallationUpdate {
            resource_type: ResourceType::VersionManifest,
            url: consts::MC_VERSION_MANIFEST_URL.to_string(),
            total: 1,
            n: 1,
            size: None,
        }));
        let manifest = version_manifest().map_err(Error::Request)?;

        // Figure out version
        let version_summary = manifest
            .versions
            .get(&self.version)
            .ok_or_else(|| InstallationError::Version(self.version.clone()))?;
        debug!("Version '{}'", &version_summary.id);

        // Create instance folder
        let mut instance_path = self.instance_path();
        trace!(
            "Creating instance folder: '{}'",
            &instance_path.to_string_lossy()
        );
        fs::create_dir_all(&instance_path).map_err(Error::Filesystem)?;

        // Save version data
        let version_data_path = self.version_data_path();
        trace!(
            "Downloading version data to: '{}'",
            &version_data_path.to_string_lossy()
        );
        let _ = sender.send(Ok(ResourceInstallationUpdate {
            resource_type: ResourceType::VersionData,
            url: version_summary.url.to_string(),
            total: 1,
            n: 1,
            size: None,
        }));
        download_file_check(&version_summary.url, &version_data_path, None)?;

        Ok(())
    }

    pub fn read_version_data(&self) -> Result<VersionData, Error> {
        debug!("Reading version data from disk");

        let version_data_path = self.version_data_path();

        trace!(
            "Reading from disk: '{}'",
            &version_data_path.to_string_lossy()
        );
        let version_data = fs::read_to_string(&version_data_path).map_err(Error::Filesystem)?;
        let version_data = serde_json::from_str::<VersionData>(&version_data)
            .map_err(InstallationError::ParseVersionData)?;

        Ok(version_data)
    }
}
