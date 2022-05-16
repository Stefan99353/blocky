use crate::minecraft::launch_options::LaunchOptions;
use crate::minecraft::models::version_data::VersionData;
use std::collections::HashMap;

pub struct ArgumentReplacements {
    pub replacements: HashMap<String, String>,
}

impl ArgumentReplacements {
    pub fn build(
        options: &LaunchOptions,
        version_data: &VersionData,
        classpath: String,
        minecraft_path: String,
        assets_path: String,
        natives_path: String,
    ) -> Self {
        let mut result = HashMap::new();
        result.insert(
            "${auth_player_name}".to_string(),
            options.player_name.clone(),
        );
        result.insert("${game_directory}".to_string(), minecraft_path.clone());
        result.insert(
            "${assets_index_name}".to_string(),
            version_data.assets.clone(),
        );
        result.insert("${assets_root}".to_string(), assets_path);
        result.insert(
            "${game_assets}".to_string(),
            format!("{}/assets", minecraft_path),
        );
        result.insert("${natives_directory}".to_string(), natives_path);
        result.insert("${version_name}".to_string(), version_data.id.clone());
        result.insert(
            "${version_type}".to_string(),
            version_data._type.to_string(),
        );
        result.insert(
            "${launcher_name}".to_string(),
            options.launcher_name.clone(),
        );
        result.insert(
            "${launcher_version}".to_string(),
            options.launcher_version.clone(),
        );
        result.insert("${classpath}".to_string(), classpath);

        if let (Some(profile_id), Some(token)) = (&options.profile_id, &options.token) {
            result.insert("${auth_uuid}".to_string(), profile_id.clone());
            result.insert("${auth_access_token}".to_string(), token.clone());
            result.insert("${auth_session}".to_string(), token.clone());
            result.insert("${user_type}".to_string(), "msa".to_string());
        }

        Self {
            replacements: result,
        }
    }

    pub fn replace(&self, arg: &str) -> String {
        let mut result = arg.to_string();

        for (pattern, value) in &self.replacements {
            result = result.replace(pattern, value);
        }

        result
    }
}
