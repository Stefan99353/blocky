use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::install::error::InstallationError;
use crate::instance::models::AssetIndexData;
use crate::Instance;
use std::fs;
use std::path::PathBuf;

impl Instance {
    pub fn install_asset_index(&self) -> Result<(), Error> {
        debug!("Installing asset index");

        let version_data = self.read_version_data()?;
        if let Some(asset_index) = &version_data.asset_index {
            // Index is stored in "indexes" dir in natives dir
            let mut indexes_path = self.assets_path();
            indexes_path.push("indexes");

            // Create folder
            fs::create_dir_all(&indexes_path).map_err(Error::Filesystem)?;

            // Download file
            indexes_path.push(format!("{}.json", &version_data.assets));
            let sha = hex::decode(&asset_index.sha1)?;
            download_file_check(&asset_index.url, indexes_path, Some(sha))?;
        }

        Ok(())
    }

    pub fn read_asset_index(&self) -> Result<AssetIndexData, Error> {
        let version_data = self.read_version_data()?;

        let mut asset_index_path = self.assets_path();
        asset_index_path.push("indexes");
        asset_index_path.push(format!("{}.json", &version_data.assets));

        let asset_index = fs::read_to_string(&asset_index_path).map_err(Error::Filesystem)?;
        let asset_index = serde_json::from_str::<AssetIndexData>(&asset_index)
            .map_err(InstallationError::ParseAssetIndex)?;

        Ok(asset_index)
    }
}
