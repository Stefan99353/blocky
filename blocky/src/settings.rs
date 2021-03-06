use crate::config;
use gio::prelude::*;
use glib::IsA;

#[derive(Clone, Debug)]
pub enum SettingKey {
    // Instance settings
    InstancesDir,
    LibrariesDir,
    AssetsDir,
    UseFullscreen,
    EnableWindowSize,
    GameWindowWidth,
    GameWindowHeight,
    EnableMemory,
    MinMemory,
    MaxMemory,
    JavaExec,
    EnableJvmArgs,
    JvmArgs,
    // Storage
    ProfilesFilePath,
    InstancesFilePath,
    // State
    DefaultProfile,
    WindowWidth,
    WindowHeight,
    IsMaximized,
}

impl SettingKey {
    pub fn to_key(&self) -> &'static str {
        match self {
            SettingKey::InstancesDir => "instances-dir",
            SettingKey::LibrariesDir => "libraries-dir",
            SettingKey::AssetsDir => "assets-dir",
            SettingKey::UseFullscreen => "use-fullscreen",
            SettingKey::EnableWindowSize => "enable-window-size",
            SettingKey::GameWindowWidth => "game-window-width",
            SettingKey::GameWindowHeight => "game-window-height",
            SettingKey::EnableMemory => "enable-memory",
            SettingKey::MinMemory => "min-memory",
            SettingKey::MaxMemory => "max-memory",
            SettingKey::JavaExec => "java-exec",
            SettingKey::EnableJvmArgs => "enable-jvm-args",
            SettingKey::JvmArgs => "jvm-args",
            SettingKey::ProfilesFilePath => "profiles-file-path",
            SettingKey::InstancesFilePath => "instances-file-path",
            SettingKey::DefaultProfile => "default-profile",
            SettingKey::WindowWidth => "window-width",
            SettingKey::WindowHeight => "window-height",
            SettingKey::IsMaximized => "is-maximized",
        }
    }
}

// TODO Error logs on unwrap
pub fn get_settings() -> gio::Settings {
    gio::Settings::new(config::APP_ID)
}

pub fn bind_property<P: IsA<glib::Object>>(key: SettingKey, object: &P, property: &str) {
    let settings = get_settings();
    settings
        .bind(key.to_key(), object, property)
        .flags(gio::SettingsBindFlags::DEFAULT)
        .build()
}

pub fn get_string(key: SettingKey) -> String {
    let settings = get_settings();
    settings.string(key.to_key()).to_string()
}

pub fn set_string(key: SettingKey, value: &str) {
    let setting = get_settings();
    setting.set_string(key.to_key(), value).unwrap();
}

pub fn get_bool(key: SettingKey) -> bool {
    let settings = get_settings();
    settings.boolean(key.to_key())
}

pub fn set_bool(key: SettingKey, value: bool) {
    let setting = get_settings();
    setting.set_boolean(key.to_key(), value).unwrap();
}

pub fn get_integer(key: SettingKey) -> i32 {
    let settings = get_settings();
    settings.int(key.to_key())
}

pub fn set_integer(key: SettingKey, value: i32) {
    let setting = get_settings();
    setting.set_int(key.to_key(), value).unwrap();
}
