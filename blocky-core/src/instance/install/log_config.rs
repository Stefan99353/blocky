use crate::error;
use crate::instance::Instance;
use crate::minecraft::install::install_log_config;
use crate::minecraft::installation_update::InstallationUpdate;
use crossbeam_channel::Sender;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

impl Instance {
    pub fn install_log_config(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        let version_data = self.read_version_data()?;

        install_log_config(
            &version_data,
            self.log_configs_path(),
            update_sender,
            cancel,
        )?;

        Ok(())
    }
}
