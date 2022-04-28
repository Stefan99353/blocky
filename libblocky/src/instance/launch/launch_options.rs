use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Builder, Clone, Debug)]
pub struct GlobalLaunchOptions {
    #[builder(default = "String::from(\"libblocky\")")]
    pub launcher_name: String,
    #[builder(default = "String::from(env!(\"CARGO_PKG_VERSION\"))")]
    pub launcher_version: String,
    #[builder(default)]
    pub use_fullscreen: bool,
    #[builder(default)]
    pub use_custom_resolution: bool,
    #[builder(default = "1280")]
    pub custom_width: u32,
    #[builder(default = "720")]
    pub custom_height: u32,
    #[builder(default = "String::from(\"java\")")]
    pub java_executable: String,
    #[builder(default)]
    pub use_custom_memory: bool,
    #[builder(default = "512")]
    pub jvm_min_memory: u32,
    #[builder(default = "1024")]
    pub jvm_max_memory: u32,
    #[builder(default)]
    pub use_custom_jvm_arguments: bool,
    #[builder(default)]
    pub jvm_arguments: String,
    #[builder(default)]
    pub environment_variables: HashMap<String, Option<String>>,
}
