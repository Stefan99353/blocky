use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::install::error::InstallationError;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::Instance;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

impl Instance {
    pub fn install_client(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
        cancel: Arc<AtomicBool>,
    ) -> Result<(), Error> {
        debug!("Installing game client");

        let version_data = self.read_version_data()?;
        if let Some(downloads) = &version_data.downloads {
            let _ = sender.send(Ok(Some(ResourceInstallationUpdate {
                resource_type: ResourceType::Client,
                url: downloads.client.url.to_string(),
                total: 1,
                n: 1,
                size: Some(downloads.client.size),
            })));
            // Check cancel
            if cancel.load(Ordering::Relaxed) {
                return Err(InstallationError::Cancelled.into());
            }

            let mut client_path = self.dot_minecraft_path();
            client_path.push("bin");

            // Create folder
            fs::create_dir_all(&client_path).map_err(Error::Filesystem)?;

            // Download file
            client_path.push(format!("minecraft-{}-client.jar", &version_data.id));
            let sha = hex::decode(&downloads.client.sha1)?;
            download_file_check(&downloads.client.url, &client_path, Some(sha))?;
        }

        Ok(())
    }
}
