use crate::instance::launch::launch_options::GlobalLaunchOptions;
use crate::instance::models::VersionData;
use crate::profile::minecraft::{MinecraftProfile, MinecraftToken};
use crate::Instance;
use std::collections::HashMap;

pub struct ArgumentReplacements {
    pub replacements: HashMap<String, String>,
}

impl ArgumentReplacements {
    pub fn build(
        instance: &Instance,
        profile: &MinecraftProfile,
        minecraft_token: &str,
        options: &GlobalLaunchOptions,
        version_data: &VersionData,
        classpath: String,
    ) -> Self {
        let replacements = vec![
            ("${auth_uuid}", profile.id.clone()),
            ("${auth_player_name}", profile.name.clone()),
            ("${auth_access_token}", minecraft_token.to_string()),
            ("${user_type}", "msa".to_string()),
            (
                "${game_directory}",
                instance.dot_minecraft_path().to_string_lossy().to_string(),
            ),
            ("${assets_index_name}", version_data.assets.clone()),
            ("${assets_root}", instance.game.assets_path.clone()),
            (
                "${natives_directory}",
                instance.natives_path().to_string_lossy().to_string(),
            ),
            ("${version_name}", version_data.id.clone()),
            ("${version_type}", version_data._type.to_string()),
            ("${launcher_name}", options.launcher_name.clone()),
            ("${launcher_version}", options.launcher_version.clone()),
            ("${classpath}", classpath),
        ];

        let mut result = HashMap::new();
        for (repl, value) in replacements {
            result.insert(repl.to_string(), value);
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
