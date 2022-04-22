use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod download;
mod extract;
pub mod install;
mod models;
mod paths;
mod utils;

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
    pub version: Option<String>,
    #[builder(default = "String::from(\".\")")]
    pub instance_path: String,
    #[builder(default)]
    pub game: GameProperties,
    #[builder(default)]
    pub process: ProcessProperties,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct GameProperties {
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub libraries_path: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub assets_path: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub custom_resolution: Option<(u32, u32)>,
    #[builder(default)]
    pub fullscreen: bool,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProcessProperties {
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub java_executable: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub jvm_arguments: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub memory: Option<(u32, u32)>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub environment_variables: Option<Vec<(String, Option<String>)>>,
}
