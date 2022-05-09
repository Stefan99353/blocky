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
    debug!("Data directory: '{}'", DATA.to_string_lossy());
    debug!("Config directory: '{}'", CONFIG.to_string_lossy());
    debug!("Cache directory: '{}'", CACHE.to_string_lossy());

    fs::create_dir_all(DATA.clone())?;
    fs::create_dir_all(CONFIG.clone())?;
    fs::create_dir_all(CACHE.clone())?;

    Ok(())
}

pub fn set_defaults() {
    let default_instances_dir = settings::get_string(SettingKey::InstancesDir);
    if default_instances_dir == "NULL" {
        settings::set_string(
            SettingKey::InstancesDir,
            &DEFAULT_INSTANCES_DIR.to_string_lossy(),
        );
    }

    let default_libraries_dir = settings::get_string(SettingKey::LibrariesDir);
    if default_libraries_dir == "NULL" {
        settings::set_string(
            SettingKey::LibrariesDir,
            &DEFAULT_LIBRARIES_DIR.to_string_lossy(),
        );
    }

    let default_assets_dir = settings::get_string(SettingKey::AssetsDir);
    if default_assets_dir == "NULL" {
        settings::set_string(SettingKey::AssetsDir, &DEFAULT_ASSETS_DIR.to_string_lossy());
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
