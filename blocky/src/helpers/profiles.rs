use anyhow::anyhow;
use blocky_core::profile::Profile;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, Profile>;

pub fn find_refresh_save(uuid: Uuid, path: impl AsRef<Path>) -> anyhow::Result<Profile> {
    let mut profile = find_profile(uuid, &path)?.ok_or(anyhow!("Profile not found: {}", uuid))?;

    debug!("Checking if token expired");
    if let Some(minecraft_token) = &profile.minecraft {
        if minecraft_token.check_expired().is_err() {
            profile.refresh(crate::config::MS_GRAPH_ID, crate::config::MS_GRAPH_SECRET)?;
            save_profile(profile.clone(), &path)?;
        }
    }

    Ok(profile)
}

pub fn load_profiles(path: impl AsRef<Path>) -> anyhow::Result<Vec<Profile>> {
    debug!("Reading profiles from disk");
    let profiles = read_file(&path)?;

    let profiles = profiles
        .into_iter()
        .map(|(_, profile)| profile)
        .collect::<Vec<Profile>>();

    Ok(profiles)
}

pub fn find_profile(uuid: Uuid, path: impl AsRef<Path>) -> anyhow::Result<Option<Profile>> {
    let profiles = read_file(&path)?;
    let profile = profiles.get(&uuid).cloned();
    Ok(profile)
}

pub fn save_profile(profile: Profile, path: impl AsRef<Path>) -> anyhow::Result<()> {
    debug!("Saving a new profile to disk or updating existing one");
    let mut profiles = read_file(&path)?;

    let _old = profiles.insert(profile.uuid, profile);

    write_file(profiles, path)
}

pub fn remove_profile(uuid: Uuid, path: impl AsRef<Path>) -> anyhow::Result<()> {
    debug!("Removing a profile from disk");
    let mut profiles = read_file(&path)?;

    let _old = profiles.remove(&uuid);

    write_file(profiles, path)
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<ProfileStorage> {
    let mut profiles = HashMap::new();

    if path.as_ref().is_file() {
        let profiles_string = fs::read_to_string(&path)?;
        profiles = serde_json::from_str::<ProfileStorage>(&profiles_string)?;
    }

    Ok(profiles)
}

fn write_file(profiles: ProfileStorage, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut file = fs::File::create(&path)?;
    let content = serde_json::to_vec(&profiles)?;
    file.write_all(&content)?;
    file.flush()?;

    Ok(())
}
