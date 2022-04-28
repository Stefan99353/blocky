use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod download;
mod extract;
pub(crate) mod install;
pub mod models;
mod paths;
pub mod resource_update;
pub(crate) mod utils;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct Instance {
    #[builder(default = "uuid::Uuid::new_v4()")]
    pub uuid: Uuid,
    #[builder(default)]
    pub name: String,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub description: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub version: String,
    #[builder(default = "String::from(\".\")")]
    pub instance_path: String,
    #[builder(default)]
    pub game: GameProperties,
    #[builder(default)]
    pub process: ProcessProperties,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameProperties {
    #[builder(default = "String::from(\"./libraries\")")]
    pub libraries_path: String,
    #[builder(default = "String::from(\"./assets\")")]
    pub assets_path: String,
    #[builder(default = "1280")]
    pub custom_width: u32,
    #[builder(default = "720")]
    pub custom_height: u32,
    #[builder(default)]
    pub use_custom_resolution: bool,
    #[builder(default)]
    pub use_fullscreen: bool,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProcessProperties {
    #[builder(default)]
    pub use_custom_java_executable: bool,
    #[builder(default)]
    pub java_executable: String,
    #[builder(default)]
    pub use_custom_jvm_arguments: bool,
    #[builder(default)]
    pub jvm_arguments: String,
    #[builder(default)]
    pub use_custom_memory: bool,
    #[builder(default)]
    pub jvm_min_memory: u32,
    #[builder(default)]
    pub jvm_max_memory: u32,
    #[builder(default)]
    pub use_environment_variables: bool,
    #[builder(default)]
    pub environment_variables: Vec<(String, Option<String>)>,
}
