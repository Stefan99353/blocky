use crate::error;
use crate::instance::Instance;
use crate::minecraft::models::asset_index_data::AssetIndexData;
use crate::utils::download_file_check;
use std::fs;

impl Instance {
    pub fn save_asset_index(&self) -> error::Result<()> {
        let version_data = self.read_version_data()?;

        if let Some(asset_index) = &version_data.asset_index {
            let mut indexes_path = self.asset_index_path();

            // Create folder
            fs::create_dir_all(&indexes_path).map_err(error::Error::IO)?;

            // Download file
            indexes_path.push(format!("{}.json", &version_data.assets));
            let sha = hex::decode(&asset_index.sha1).map_err(error::Error::Sha1Decode)?;
            download_file_check(&asset_index.url, indexes_path, Some(sha))?;
        }

        Ok(())
    }

    pub fn read_asset_index(&self) -> error::Result<AssetIndexData> {
        let version_data = self.read_version_data()?;

        let mut asset_index_path = self.asset_index_path();
        asset_index_path.push(format!("{}.json", &version_data.assets));

        let asset_index = fs::read_to_string(&asset_index_path).map_err(error::Error::IO)?;
        let asset_index =
            serde_json::from_str::<AssetIndexData>(&asset_index).map_err(error::Error::Serde)?;

        Ok(asset_index)
    }
}
