use crate::config::{MS_GRAPH_ID, MS_GRAPH_SECRET};
use crate::helpers::error::error_dialog;
use crate::helpers::{read_file, write_file};
use crate::settings;
use crate::settings::SettingKey;
use blocky_core::profile::Profile;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, Profile>;

pub fn save_profile(profile: Profile) {
    let path = PathBuf::from(settings::get_string(SettingKey::ProfilesFilePath));
    let mut saved_profiles: ProfileStorage = match read_file(&path) {
        Ok(profiles) => profiles,
        Err(err) => {
            error_dialog(err);
            return;
        }
    };

    let _old_profile = saved_profiles.insert(profile.uuid, profile);

    if let Err(err) = write_file(&saved_profiles, &path) {
        error_dialog(err);
    }
}

pub fn remove_profile(uuid: Uuid) {
    let path = PathBuf::from(settings::get_string(SettingKey::ProfilesFilePath));
    let mut saved_profiles: ProfileStorage = match read_file(&path) {
        Ok(profiles) => profiles,
        Err(err) => {
            error_dialog(err);
            return;
        }
    };

    let _old_profile = saved_profiles.remove(&uuid);

    if let Err(err) = write_file(&saved_profiles, &path) {
        error_dialog(err);
    }
}

pub fn load_profiles() -> Vec<Profile> {
    let path = PathBuf::from(settings::get_string(SettingKey::ProfilesFilePath));
    let saved_profiles: ProfileStorage = match read_file(&path) {
        Ok(profiles) => profiles,
        Err(err) => {
            error_dialog(err);
            return vec![];
        }
    };

    saved_profiles.into_iter().map(|(_, p)| p).collect()
}

pub fn find_profile(uuid: Uuid) -> Option<Profile> {
    let path = PathBuf::from(settings::get_string(SettingKey::ProfilesFilePath));
    let saved_profiles: ProfileStorage = match read_file(&path) {
        Ok(profiles) => profiles,
        Err(err) => {
            error_dialog(err);
            return None;
        }
    };

    saved_profiles.get(&uuid).cloned()
}

pub fn refresh_and_save_profile(profile: &mut Profile) -> anyhow::Result<()> {
    if let Some(minecraft_token) = &profile.minecraft {
        if minecraft_token.check_expired().is_err() {
            profile.refresh(MS_GRAPH_ID, MS_GRAPH_SECRET)?;
            save_profile(profile.clone());
        }
    }

    Ok(())
}
