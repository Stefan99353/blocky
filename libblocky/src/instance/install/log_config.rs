use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::Instance;
use std::fs;

impl Instance {
    pub fn install_log_config(&self) -> Result<(), Error> {
        debug!("Installing log config");

        let version_data = self.read_version_data()?;
        if let Some(logging_info) = &version_data.logging {
            // Log config is stored in "log_configs" dir in natives dir
            let mut config_path = self.assets_path();
            config_path.push("log_configs");

            // Create folder
            fs::create_dir_all(&config_path).map_err(Error::Filesystem)?;

            // Download file
            config_path.push(&logging_info.client.file.id);
            download_file_check(
                &logging_info.client.file.url,
                &config_path,
                Some(&logging_info.client.file.sha1.as_bytes()),
            )?;
        }

        Ok(())
    }
}
