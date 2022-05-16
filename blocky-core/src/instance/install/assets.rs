use crate::error;
use crate::instance::Instance;
use crate::minecraft::install::{install_assets, install_resources};
use crate::minecraft::installation_update::InstallationUpdate;
use crossbeam_channel::Sender;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

impl Instance {
    pub fn install_assets(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        let asset_index = self.read_asset_index()?;

        // Check if installed as resource
        match asset_index.map_to_resources {
            true => install_resources(
                &asset_index,
                self.resources_path(),
                self.dot_minecraft_path(),
                update_sender,
                cancel,
            )?,
            false => install_assets(&asset_index, self.assets_path(), update_sender, cancel)?,
        }

        Ok(())
    }
}
