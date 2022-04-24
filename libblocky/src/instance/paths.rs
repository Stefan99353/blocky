use crate::Instance;
use std::path::PathBuf;

impl Instance {
    pub fn instance_path(&self) -> PathBuf {
        PathBuf::from(&self.instance_path)
    }

    pub fn libraries_path(&self) -> PathBuf {
        PathBuf::from(&self.game.libraries_path)
    }

    pub fn assets_path(&self) -> PathBuf {
        PathBuf::from(&self.game.assets_path)
    }
}
