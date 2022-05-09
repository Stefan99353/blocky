use crate::instance::download::get_sha1;
use crate::Instance;

impl Instance {
    pub fn check_installed(&self) -> crate::error::Result<bool> {
        debug!("Checking if instance is installed");

        let version_data_path = self.version_data_path();
        if !version_data_path.is_file() {
            return Ok(false);
        }

        let version_data = self.read_version_data()?;
        if let Some(downloads) = &version_data.downloads {
            let mut client_path = self.dot_minecraft_path();
            client_path.push("bin");
            client_path.push(format!("minecraft-{}-client.jar", &version_data.id));

            if !client_path.is_file() {
                return Ok(false);
            }

            let remote_sha = hex::decode(&downloads.client.sha1)?;
            let local_sha = get_sha1(client_path)?;

            return Ok(remote_sha == local_sha);
        }

        Ok(false)
    }
}
