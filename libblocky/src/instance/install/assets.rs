use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::{consts, Instance};
use std::fs;

impl Instance {
    pub fn install_assets(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
    ) -> Result<(), Error> {
        debug!("Installing assets");

        let asset_index = self.read_asset_index()?;

        // Assets are stored in "objects" dir in assets dir
        let mut objects_path = self.assets_path();
        objects_path.push("objects");
        fs::create_dir_all(&objects_path).map_err(Error::Filesystem)?;

        let total = asset_index.objects.len();

        for (n, info) in asset_index.objects.values().enumerate() {
            let asset_sub_folder: String = info.hash.chars().take(2).collect();
            let mut asset_path = objects_path.clone();
            asset_path.push(&asset_sub_folder);

            // Create dir
            fs::create_dir_all(&asset_path).map_err(Error::Filesystem)?;

            // Download asset
            asset_path.push(&info.hash);
            let url = format!(
                "{}/{}/{}",
                consts::MC_ASSETS_BASE_URL,
                &asset_sub_folder,
                &info.hash
            );

            let _ = sender.send(Ok(ResourceInstallationUpdate {
                resource_type: ResourceType::Asset,
                url: url.to_string(),
                total,
                n,
                size: Some(info.size),
            }));

            let sha = hex::decode(&info.hash)?;
            download_file_check(&url, &asset_path, Some(sha))?;
        }

        Ok(())
    }
}
