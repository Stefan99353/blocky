use crate::helpers::error::error_dialog;
use crate::helpers::{read_file, write_file};
use crate::settings::SettingKey;
use crate::{helpers, settings};
use anyhow::anyhow;
use blocky_core::instance::fabric::FabricInstanceExt;
use blocky_core::instance::Instance;
use blocky_core::minecraft::installation_update::InstallationUpdate;
use blocky_core::minecraft::launch_options::{LaunchOptions, LaunchOptionsBuilder};
use gtk_macros::send;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use uuid::Uuid;

type InstanceStorage = HashMap<Uuid, Instance>;

pub fn save_instance(instance: Instance) {
    let path = PathBuf::from(settings::get_string(SettingKey::InstancesFilePath));
    let mut saved_instances: InstanceStorage = match read_file(&path) {
        Ok(instances) => instances,
        Err(err) => {
            error_dialog(err);
            return;
        }
    };

    let _old_instance = saved_instances.insert(instance.uuid, instance);

    if let Err(err) = write_file(&saved_instances, &path) {
        error_dialog(err);
    }
}

pub fn remove_instance(uuid: Uuid) {
    let path = PathBuf::from(settings::get_string(SettingKey::InstancesFilePath));
    let mut saved_instances: InstanceStorage = match read_file(&path) {
        Ok(instances) => instances,
        Err(err) => {
            error_dialog(err);
            return;
        }
    };

    let _old_instance = saved_instances.remove(&uuid);

    if let Err(err) = write_file(&saved_instances, &path) {
        error_dialog(err);
    }
}

pub fn load_instances() -> Vec<Instance> {
    let path = PathBuf::from(settings::get_string(SettingKey::InstancesFilePath));
    let saved_instances: InstanceStorage = match read_file(&path) {
        Ok(instances) => instances,
        Err(err) => {
            error_dialog(err);
            return vec![];
        }
    };

    saved_instances.into_iter().map(|(_, i)| i).collect()
}

pub fn find_instance(uuid: Uuid) -> Option<Instance> {
    let path = PathBuf::from(settings::get_string(SettingKey::InstancesFilePath));
    let saved_instances: InstanceStorage = match read_file(&path) {
        Ok(instances) => instances,
        Err(err) => {
            error_dialog(err);
            return None;
        }
    };

    saved_instances.get(&uuid).cloned()
}

pub fn install(uuid: Uuid, g_sender: glib::Sender<InstallationUpdate>, cancel: Arc<AtomicBool>) {
    let instance = match find_instance(uuid) {
        None => {
            error_dialog(anyhow!("Instance not found '{}'", uuid));
            return;
        }
        Some(instance) => instance,
    };

    let (sender, receiver) = crossbeam_channel::unbounded();
    let _install_handle = thread::spawn(move || {
        if let Err(err) = instance.full_install(sender, cancel) {
            error_dialog(err);
        }
    });

    let _translate_handle = thread::spawn(move || {
        while let Ok(update) = receiver.recv() {
            send!(g_sender, update);
        }
    });
}

pub fn check_installed(uuid: Uuid) -> (glib::Receiver<bool>, JoinHandle<()>) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let handle = thread::spawn(move || match find_instance(uuid) {
        None => {
            error_dialog(anyhow!("Instance not found '{}'", uuid));
            send!(sender, false);
        }
        Some(instance) => match instance.check_installed() {
            Ok(installed) => {
                send!(sender, installed);
            }
            Err(err) => {
                error_dialog(err);
                send!(sender, false);
            }
        },
    });

    (receiver, handle)
}

pub fn launch(uuid: Uuid, options: &LaunchOptions) {
    let instance = match find_instance(uuid) {
        None => {
            error_dialog(anyhow!("Instance not found '{}'", uuid));
            return;
        }
        Some(instance) => instance,
    };

    if instance.use_fabric && instance.fabric_version.is_some() {
        // Fabric launch
        debug!("Launching instance with fabric");
        if let Err(err) = instance.fabric_launch(options) {
            error_dialog(err);
        }
    } else {
        // Vanilla launch
        debug!("Launching instance");
        if let Err(err) = instance.launch(options) {
            error_dialog(err);
        }
    }
}

pub fn build_launch_options(
    instance_uuid: Uuid,
    profile_uuid: Uuid,
) -> anyhow::Result<LaunchOptions> {
    let instance = find_instance(instance_uuid)
        .ok_or_else(|| anyhow!("Instance not found '{}'", instance_uuid))?;
    let mut profile = super::profiles::find_profile(profile_uuid)
        .ok_or_else(|| anyhow!("Profile not found '{}'", profile_uuid))?;

    helpers::profiles::refresh_and_save_profile(&mut profile)?;

    let minecraft_profile = profile
        .minecraft_profile
        .ok_or_else(|| anyhow!("Minecraft profile is missing"))?;
    let access_token = profile
        .minecraft
        .ok_or_else(|| anyhow!("Unauthenticated"))?;

    let mut builder = LaunchOptionsBuilder::default();
    builder
        .launcher_name("Blocky".to_string())
        .launcher_version(env!("CARGO_PKG_VERSION").to_string())
        .player_name(minecraft_profile.name)
        .profile_id(minecraft_profile.id)
        .token(access_token.token)
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

    Ok(builder.build()?)
}
