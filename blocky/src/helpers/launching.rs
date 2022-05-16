use crate::helpers::{find_instance, find_profile, find_refresh_save, save_profile};
use crate::settings;
use crate::settings::SettingKey;
use anyhow::anyhow;
use blocky_core::minecraft::launch_options::{LaunchOptions, LaunchOptionsBuilder};
use chrono::Utc;
use uuid::Uuid;

pub fn launch_instance(
    instance_uuid: Uuid,
    instances_path: String,
    options: LaunchOptions,
) -> anyhow::Result<()> {
    let instance = find_instance(instance_uuid, instances_path)?
        .ok_or(anyhow!("Instance not found: {}", instance_uuid))?;

    instance.launch(&options)?;

    Ok(())
}

pub fn build_launch_options(
    instance_uuid: Uuid,
    instances_path: String,
    profile_uuid: Uuid,
    profiles_path: String,
) -> anyhow::Result<LaunchOptions> {
    let instance = find_instance(instance_uuid, instances_path)?
        .ok_or(anyhow!("Instance not found: {}", instance_uuid))?;
    find_refresh_save(profile_uuid, &profiles_path)?;
    let profile = find_profile(profile_uuid, profiles_path)?
        .ok_or(anyhow!("Profile not found: {}", profile_uuid))?;
    let minecraft_profile = profile
        .minecraft_profile
        .ok_or(anyhow!("Profile is missing"))?;
    let minecraft_token = profile.minecraft.ok_or(anyhow!("Unauthenticated"))?.token;

    let mut builder = LaunchOptionsBuilder::default();

    builder
        .launcher_name("Blocky".to_string())
        .launcher_version(env!("CARGO_PKG_VERSION").to_string())
        .player_name(minecraft_profile.name)
        .profile_id(minecraft_profile.id)
        .token(minecraft_token);

    builder
        .use_fullscreen(instance.use_fullscreen || settings::get_bool(SettingKey::UseFullscreen));
    if instance.enable_window_size {
        builder
            .enable_window_size(true)
            .window_width(instance.window_width)
            .window_height(instance.window_height);
    } else if settings::get_bool(SettingKey::EnableWindowSize) {
        builder
            .enable_window_size(true)
            .window_width(settings::get_integer(SettingKey::GameWindowWidth) as u32)
            .window_height(settings::get_integer(SettingKey::GameWindowHeight) as u32);
    }

    if instance.enable_memory {
        builder
            .enable_memory(true)
            .min_memory(instance.min_memory)
            .max_memory(instance.max_memory);
    } else if settings::get_bool(SettingKey::EnableMemory) {
        builder
            .enable_memory(true)
            .min_memory(settings::get_integer(SettingKey::MinMemory) as u32)
            .max_memory(settings::get_integer(SettingKey::MaxMemory) as u32);
    }

    if !instance.java_exec.trim().is_empty() {
        builder.java_exec(instance.java_exec);
    } else {
        builder.java_exec(settings::get_string(SettingKey::JavaExec));
    }

    if instance.enable_jvm_args {
        builder.enable_jvm_args(true).jvm_args(instance.jvm_args);
    } else if settings::get_bool(SettingKey::EnableJvmArgs) {
        builder
            .enable_jvm_args(true)
            .jvm_args(settings::get_string(SettingKey::JvmArgs));
    }

    Ok(builder.build().unwrap())
}
