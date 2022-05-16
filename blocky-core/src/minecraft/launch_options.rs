use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Builder, Clone, Debug)]
pub struct LaunchOptions {
    #[builder(default = "String::from(\"blocky-core\")")]
    pub launcher_name: String,
    #[builder(default = "String::from(env!(\"CARGO_PKG_VERSION\"))")]
    pub launcher_version: String,

    #[builder(default = "String::from(\"Steve\")")]
    pub player_name: String,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub profile_id: Option<String>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub token: Option<String>,

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
    #[builder(default = "512")]
    pub min_memory: u32,
    #[builder(default = "1024")]
    pub max_memory: u32,
    #[builder(default = "String::from(\"java\")")]
    pub java_exec: String,
    #[builder(default)]
    pub enable_jvm_args: bool,
    #[builder(default)]
    pub jvm_args: String,
    #[builder(default)]
    pub environment_variables: HashMap<String, Option<String>>,
}
