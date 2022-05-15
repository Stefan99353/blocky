use crate::Instance;
use std::fs;

impl Instance {
    pub fn install_paths(&self) -> Result<(), std::io::Error> {
        // Instance
        fs::create_dir_all(self.instance_path())?;
        // Libraries
        fs::create_dir_all(self.libraries_path())?;
        // Assets
        fs::create_dir_all(self.assets_path())?;
        // .minecraft
        fs::create_dir_all(self.dot_minecraft_path())?;

        Ok(())
    }
}
