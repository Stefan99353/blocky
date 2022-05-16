use crate::error;
use crate::instance::Instance;

impl Instance {
    // TODO: Properly check installed state
    pub fn check_installed(&self) -> error::Result<bool> {
        let version_data_path = self.version_data_path();
        if !version_data_path.is_file() {
            return Ok(false);
        }

        let version_data = self.read_version_data()?;

        if version_data.downloads.is_some() {
            let mut client_path = self.dot_minecraft_path();
            client_path.push("bin");
            client_path.push(format!("minecraft-{}-client.jar", &version_data.id));

            if !client_path.is_file() {
                return Ok(false);
            }
        }

        Ok(false)
    }
}
