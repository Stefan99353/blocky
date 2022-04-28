use crate::Instance;
use std::path::PathBuf;

impl Instance {
    pub fn instance_path(&self) -> PathBuf {
        PathBuf::from(&self.instance_path)
    }

    pub fn version_data_path(&self) -> PathBuf {
        let mut path = self.instance_path();
        path.push("version.json");
        path
    }

    pub fn libraries_path(&self) -> PathBuf {
        PathBuf::from(&self.game.libraries_path)
    }

    pub fn natives_path(&self) -> PathBuf {
        let mut path = self.instance_path();
        path.push("natives");
        path
    }

    pub fn assets_path(&self) -> PathBuf {
        PathBuf::from(&self.game.assets_path)
    }

    pub fn asset_index_path(&self) -> PathBuf {
        let mut path = self.assets_path();
        path.push("indexes");
        path
    }

    pub fn log_configs_path(&self) -> PathBuf {
        let mut path = self.assets_path();
        path.push("log_configs");
        path
    }

    pub fn dot_minecraft_path(&self) -> PathBuf {
        let mut path = self.instance_path();
        path.push(".minecraft");
        path
    }
}
