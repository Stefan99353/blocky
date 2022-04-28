use crate::instance::resource_update::ResourceInstallationUpdate;
use crate::Instance;

mod asset_index;
mod assets;
mod client;
pub mod error;
mod libraries;
mod log_config;
mod version_data;

impl Instance {
    pub fn full_install(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
    ) {
        info!("Installing instance {}", &self.uuid);

        if let Err(err) = self.install_version_data(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }
        if let Err(err) = self.install_libraries(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }
        if let Err(err) = self.install_asset_index(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }
        if let Err(err) = self.install_assets(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }
        if let Err(err) = self.install_log_config(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }
        if let Err(err) = self.install_client(sender.clone()) {
            let _ = sender.send(Err(err));
            return;
        }

        info!("Finished installing instance");
    }
}
