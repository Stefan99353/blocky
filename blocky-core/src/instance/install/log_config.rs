use crate::error;
use crate::instance::download::download_file_check;
use crate::instance::install::error::InstallationError;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::Instance;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

impl Instance {
    pub fn install_log_config(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        debug!("Installing log config");

        let version_data = self.read_version_data()?;
        if let Some(logging_info) = &version_data.logging {
            let _ = sender.send(Ok(Some(ResourceInstallationUpdate {
                resource_type: ResourceType::LogConfig,
                url: logging_info.client.file.url.to_string(),
                total: 1,
                n: 1,
                size: Some(logging_info.client.file.size),
            })));
            // Check cancel
            if cancel.load(Ordering::Relaxed) {
                return Err(InstallationError::Cancelled.into());
            }

            // Log config is stored in "log_configs" dir in natives dir
            let mut config_path = self.log_configs_path();

            // Create folder
            fs::create_dir_all(&config_path).map_err(error::Error::Filesystem)?;

            // Download file
            config_path.push(&logging_info.client.file.id);
            let sha = hex::decode(&logging_info.client.file.sha1)?;
            download_file_check(&logging_info.client.file.url, &config_path, Some(sha))?;
        }

        Ok(())
    }
}