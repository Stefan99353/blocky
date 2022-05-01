use crate::instance::install::error::InstallationError;
use crate::instance::resource_update::ResourceInstallationUpdate;
use crate::Instance;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod asset_index;
mod assets;
mod check;
mod client;
pub mod error;
mod libraries;
mod log_config;
mod version_data;

impl Instance {
    pub fn full_install(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) {
        debug!("Installing instance {}", &self.uuid);

        if let Err(err) = self.install_version_data(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        if let Err(err) = self.install_libraries(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        if let Err(err) = self.install_asset_index(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        if let Err(err) = self.install_assets(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        if let Err(err) = self.install_log_config(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        if let Err(err) = self.install_client(sender.clone(), cancel.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        debug!("Finished installing instance");
        let _ = sender.send(Ok(None));
    }
}
