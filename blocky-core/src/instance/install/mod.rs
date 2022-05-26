mod asset_index;
mod assets;
mod check;
mod client;
mod libraries;
mod log_config;
mod version_data;

use crate::error;
#[cfg(feature = "fabric")]
use crate::instance::fabric::FabricInstanceExt;
use crate::instance::Instance;
use crate::minecraft::installation_update::InstallationUpdate;
use crossbeam_channel::Sender;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

impl Instance {
    pub fn full_install(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        // Prepare needed files
        fs::create_dir_all(self.instance_path()).map_err(error::Error::IO)?;
        self.save_version_data()?;
        self.save_asset_index()?;

        #[cfg(feature = "fabric")]
        if self.use_fabric {
            self.save_fabric_version_data()?;
        }

        // Install resources
        self.install_libraries(update_sender.clone(), cancel.clone())?;
        self.install_assets(update_sender.clone(), cancel.clone())?;
        self.install_log_config(update_sender.clone(), cancel.clone())?;
        self.install_client(update_sender.clone(), cancel.clone())?;

        #[cfg(feature = "fabric")]
        if self.use_fabric {
            self.install_fabric_libraries(update_sender.clone(), cancel)?;
        }

        // Done
        let _ = update_sender.send(InstallationUpdate::Success);

        Ok(())
    }
}
