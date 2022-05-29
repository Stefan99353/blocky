use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "fabric")]
pub mod fabric;
mod install;
mod launch;
#[cfg(feature = "fabric")]
pub mod mods;
mod paths;
mod remove;

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

    #[cfg(feature = "fabric")]
    #[builder(default)]
    pub use_fabric: bool,
    #[cfg(feature = "fabric")]
    #[builder(default)]
    pub fabric_version: String,
}
