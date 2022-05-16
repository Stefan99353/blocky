use crate::error;
use crate::instance::Instance;
use crate::minecraft::install::install_libraries;
use crate::minecraft::installation_update::InstallationUpdate;
use crossbeam_channel::Sender;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

impl Instance {
    pub fn install_libraries(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        let version_data = self.read_version_data()?;

        install_libraries(
            &version_data,
            self.libraries_path(),
            self.natives_path(),
            update_sender,
            cancel,
        )?;

        Ok(())
    }
}
