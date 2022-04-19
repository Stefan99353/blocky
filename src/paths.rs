use crate::config;
use crate::settings;
use crate::settings::SettingKey;
use lazy_static::lazy_static;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref DATA: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path
    };
    pub static ref CONFIG: PathBuf = {
        let mut path = glib::user_config_dir();
        path.push(config::PKG_NAME);
        path
    };
    pub static ref CACHE: PathBuf = {
        let mut path = glib::user_cache_dir();
        path.push(config::PKG_NAME);
        path
    };
    static ref DEFAULT_INSTANCES_DIR: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path.push("instances");
        path
    };
    static ref DEFAULT_LIBRARIES_DIR: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path.push("libraries");
        path
    };
    static ref DEFAULT_ASSETS_DIR: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path.push("assets");
        path
    };
    static ref PROFILES_FILE_PATH: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path.push("profiles.json");
        path
    };
    static ref INSTANCES_FILE_PATH: PathBuf = {
        let mut path = glib::user_data_dir();
        path.push(config::PKG_NAME);
        path.push("instances.json");
        path
    };
}

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(DATA.clone())?;
    fs::create_dir_all(CONFIG.clone())?;
    fs::create_dir_all(CACHE.clone())?;

    Ok(())
}

pub fn set_defaults() {
    let default_instances_dir = settings::get_string(SettingKey::DefaultInstancesDir);
    if default_instances_dir == "NULL" {
        settings::set_string(
            SettingKey::DefaultInstancesDir,
            &DEFAULT_INSTANCES_DIR.to_string_lossy(),
        );
    }

    let default_libraries_dir = settings::get_string(SettingKey::DefaultLibrariesDir);
    if default_libraries_dir == "NULL" {
        settings::set_string(
            SettingKey::DefaultLibrariesDir,
            &DEFAULT_LIBRARIES_DIR.to_string_lossy(),
        );
    }

    let default_assets_dir = settings::get_string(SettingKey::DefaultAssetsDir);
    if default_assets_dir == "NULL" {
        settings::set_string(
            SettingKey::DefaultAssetsDir,
            &DEFAULT_ASSETS_DIR.to_string_lossy(),
        );
    }

    let profiles_file_path = settings::get_string(SettingKey::ProfilesFilePath);
    if profiles_file_path == "NULL" {
        settings::set_string(
            SettingKey::ProfilesFilePath,
            &PROFILES_FILE_PATH.to_string_lossy(),
        );
    }

    let instances_file_path = settings::get_string(SettingKey::InstancesFilePath);
    if instances_file_path == "NULL" {
        settings::set_string(
            SettingKey::InstancesFilePath,
            &INSTANCES_FILE_PATH.to_string_lossy(),
        );
    }
}
