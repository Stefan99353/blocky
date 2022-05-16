use crate::error;
use crate::instance::Instance;
use crate::minecraft::models::version_data::VersionData;
use crate::minecraft::models::version_manifest::VersionManifest;
use crate::utils::download_file_check;
use std::fs;

impl Instance {
    pub fn save_version_data(&self) -> error::Result<()> {
        let manifest = VersionManifest::get()?;

        // Figure out version
        let version_summary = manifest
            .versions
            .get(&self.version)
            .ok_or_else(|| error::Error::Version(self.version.clone()))?;

        // Save version data
        let version_data_path = self.version_data_path();
        download_file_check(&version_summary.url, &version_data_path, None)?;

        Ok(())
    }

    pub fn read_version_data(&self) -> error::Result<VersionData> {
        let version_data_path = self.version_data_path();

        let version_data = fs::read_to_string(&version_data_path).map_err(error::Error::IO)?;
        let version_data =
            serde_json::from_str::<VersionData>(&version_data).map_err(error::Error::Serde)?;

        Ok(version_data)
    }
}
