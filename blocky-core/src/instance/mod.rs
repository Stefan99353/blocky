use derive_builder::Builder;
pub use launch::launch_options;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod download;
mod extract;
pub(crate) mod install;
pub(crate) mod launch;
pub mod models;
mod paths;
mod remove;
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
    #[builder(default)]
    pub use_fullscreen: bool,
    #[builder(default)]
    pub enable_window_size: bool,
    #[builder(default = "1280")]
    pub window_width: u32,
    #[builder(default = "720")]
    pub window_height: u32,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProcessProperties {
    #[builder(default)]
    pub enable_memory: bool,
    #[builder(default = "1024")]
    pub min_memory: u32,
    #[builder(default = "2048")]
    pub max_memory: u32,
    #[builder(default)]
    pub enable_java_exec: bool,
    #[builder(default)]
    pub java_exec: String,
    #[builder(default)]
    pub enable_jvm_args: bool,
    #[builder(default)]
    pub jvm_args: String,
    #[builder(default)]
    pub enable_environment: bool,
    #[builder(default)]
    pub environment_variables: Vec<(String, Option<String>)>,
}
