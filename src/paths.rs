use crate::config;
use adw::glib;
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
}

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(DATA.clone())?;
    fs::create_dir_all(CONFIG.clone())?;
    fs::create_dir_all(CACHE.clone())?;

    Ok(())
}
