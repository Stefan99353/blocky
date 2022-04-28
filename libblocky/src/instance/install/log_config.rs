use crate::error;
use crate::instance::download::download_file_check;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::Instance;
use std::fs;

impl Instance {
    pub fn install_log_config(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
    ) -> error::Result<()> {
        debug!("Installing log config");

        let version_data = self.read_version_data()?;
        if let Some(logging_info) = &version_data.logging {
            let _ = sender.send(Ok(ResourceInstallationUpdate {
                resource_type: ResourceType::LogConfig,
                url: logging_info.client.file.url.to_string(),
                total: 1,
                n: 1,
                size: Some(logging_info.client.file.size),
            }));

            // Log config is stored in "log_configs" dir in natives dir
            let mut config_path = self.assets_path();
            config_path.push("log_configs");

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
