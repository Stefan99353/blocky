use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::install::error::InstallationError;
use crate::instance::models::AssetIndexData;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::Instance;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl Instance {
    pub fn install_asset_index(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) -> Result<(), Error> {
        debug!("Installing asset index");

        let version_data = self.read_version_data()?;
        if let Some(asset_index) = &version_data.asset_index {
            let _ = sender.send(Ok(Some(ResourceInstallationUpdate {
                resource_type: ResourceType::AssetIndex,
                url: asset_index.url.to_string(),
                total: 1,
                n: 1,
                size: Some(asset_index.size),
            })));
            // Check cancel
            if cancel.load(Ordering::Relaxed) {
                return Err(InstallationError::Cancelled.into());
            }

            // Index is stored in "indexes" dir in natives dir
            let mut indexes_path = self.asset_index_path();

            // Create folder
            fs::create_dir_all(&indexes_path).map_err(Error::Filesystem)?;

            // Download file
            indexes_path.push(format!("{}.json", &version_data.assets));
            let sha = hex::decode(&asset_index.sha1)?;
            download_file_check(&asset_index.url, indexes_path, Some(sha))?;
        }

        Ok(())
    }

    pub fn read_asset_index(&self) -> Result<AssetIndexData, Error> {
        let version_data = self.read_version_data()?;

        let mut asset_index_path = self.asset_index_path();
        asset_index_path.push(format!("{}.json", &version_data.assets));

        let asset_index = fs::read_to_string(&asset_index_path).map_err(Error::Filesystem)?;
        let asset_index = serde_json::from_str::<AssetIndexData>(&asset_index)
            .map_err(InstallationError::ParseAssetIndex)?;

        Ok(asset_index)
    }
}
