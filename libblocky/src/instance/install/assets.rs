use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::install::error::InstallationError;
use crate::instance::models::AssetIndexData;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::{consts, Instance};
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl Instance {
    pub fn install_assets(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) -> Result<(), Error> {
        debug!("Installing assets");

        let asset_index = self.read_asset_index()?;

        // Check if installed as resource
        if asset_index.map_to_resources {
            return self.install_resources(asset_index, sender, cancel);
        }

        // Assets are stored in "objects" dir in assets dir
        // Only in newer versions
        let mut objects_path = self.assets_path();
        objects_path.push("objects");
        fs::create_dir_all(&objects_path).map_err(Error::Filesystem)?;

        let total = asset_index.objects.len();

        for (n, info) in asset_index.objects.values().enumerate() {
            let hash_path: String = info.hash.chars().take(2).collect();
            let mut asset_path = objects_path.clone();
            asset_path.push(&hash_path);

            // Create dir
            fs::create_dir_all(&asset_path).map_err(Error::Filesystem)?;

            // Download asset
            asset_path.push(&info.hash);
            let url = format!(
                "{}/{}/{}",
                consts::MC_ASSETS_BASE_URL,
                &hash_path,
                &info.hash
            );

            let _ = sender.send(Ok(Some(ResourceInstallationUpdate {
                resource_type: ResourceType::Asset,
                url: url.to_string(),
                total,
                n,
                size: Some(info.size),
            })));
            // Check cancel
            if cancel.load(Ordering::Relaxed) {
                return Err(InstallationError::Cancelled.into());
            }

            let sha = hex::decode(&info.hash)?;
            download_file_check(&url, &asset_path, Some(sha))?;
        }

        Ok(())
    }

    fn install_resources(
        &self,
        asset_index: AssetIndexData,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) -> Result<(), Error> {
        debug!("Installing assets as resources");

        // Assets (resources) are stored in "resources" dir in .minecraft
        let resources_path = self.resources_path();
        fs::create_dir_all(&resources_path).map_err(Error::Filesystem)?;

        let total = asset_index.objects.len();

        for (n, (key, info)) in asset_index.objects.iter().enumerate() {
            let hash_path: String = info.hash.chars().take(2).collect();
            // Create dir
            let mut resource_folder = self.resources_path();
            resource_folder.push(key);
            resource_folder.pop();
            fs::create_dir_all(&resource_folder).map_err(Error::Filesystem)?;

            // Download asset
            let mut resource_path = self.resources_path();
            resource_path.push(key);
            let url = format!(
                "{}/{}/{}",
                consts::MC_ASSETS_BASE_URL,
                &hash_path,
                &info.hash
            );

            let _ = sender.send(Ok(Some(ResourceInstallationUpdate {
                resource_type: ResourceType::Asset,
                url: url.to_string(),
                total,
                n,
                size: Some(info.size),
            })));
            // Check cancel
            if cancel.load(Ordering::Relaxed) {
                return Err(InstallationError::Cancelled.into());
            }

            let sha = hex::decode(&info.hash)?;
            download_file_check(&url, &resource_path, Some(sha))?;
        }

        // Create symlink to for .minecraft/resources
        let dot_minecraft_resources_path = self.dot_minecraft_resources_path();
        if !dot_minecraft_resources_path.exists() {
            symlink::symlink_dir(self.resources_path(), dot_minecraft_resources_path)
                .map_err(Error::Filesystem)?;
        }

        Ok(())
    }
}
