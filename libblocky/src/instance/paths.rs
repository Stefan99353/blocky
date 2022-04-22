use crate::Instance;
use std::path::PathBuf;

impl Instance {
    pub fn instance_path(&self) -> PathBuf {
        PathBuf::from(&self.instance_path)
    }

    pub fn libraries_path(&self) -> PathBuf {
        match &self.game.libraries_path {
            None => {
                let mut path = PathBuf::from(&self.instance_path);
                path.push(".minecraft/libraries");
                path
            }
            Some(path) => PathBuf::from(path),
        }
    }

    pub fn assets_path(&self) -> PathBuf {
        match &self.game.assets_path {
            None => {
                let mut path = PathBuf::from(&self.instance_path);
                path.push(".minecraft/assets");
                path
            }
            Some(path) => PathBuf::from(path),
        }
    }
}
