use crate::Instance;

mod asset_index;
mod assets;
mod client;
pub mod error;
mod libraries;
mod log_config;
mod version_data;

impl Instance {
    pub fn full_install(&self) -> crate::error::Result<()> {
        info!("Installing instance {}", &self.uuid);

        self.install_version_data()?;
        self.install_libraries()?;
        self.install_asset_index()?;
        self.install_assets()?;
        self.install_log_config()?;
        self.install_client()?;

        info!("Finished installing instance");
        Ok(())
    }
}
