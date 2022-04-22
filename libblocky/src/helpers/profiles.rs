use super::HelperError;
use crate::Profile;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, Profile>;

pub fn load_profiles(path: impl AsRef<Path>) -> Result<Vec<Profile>, HelperError> {
    debug!("Reading profiles from disk");
    let profiles = read_file(&path)?;

    let profiles = profiles
        .into_iter()
        .map(|(_, profile)| profile)
        .collect::<Vec<Profile>>();

    Ok(profiles)
}

pub fn find_profile(uuid: Uuid, path: impl AsRef<Path>) -> Result<Option<Profile>, HelperError> {
    let profiles = read_file(&path)?;
    let profile = profiles.get(&uuid).cloned();
    Ok(profile)
}

pub fn save_profile(profile: Profile, path: impl AsRef<Path>) -> Result<(), HelperError> {
    debug!("Saving a new profile to disk or updating existing one");
    let mut profiles = read_file(&path)?;

    let _old = profiles.insert(profile.uuid, profile);

    write_file(profiles, path)
}

pub fn remove_profile(uuid: Uuid, path: impl AsRef<Path>) -> Result<(), HelperError> {
    debug!("Removing a profile from disk");
    let mut profiles = read_file(&path)?;

    let _old = profiles.remove(&uuid);

    write_file(profiles, path)
}

fn read_file(path: impl AsRef<Path>) -> Result<ProfileStorage, HelperError> {
    let mut profiles = HashMap::new();

    if path.as_ref().is_file() {
        let profiles_string = fs::read_to_string(&path)?;
        profiles = serde_json::from_str::<ProfileStorage>(&profiles_string)?;
    }

    Ok(profiles)
}

fn write_file(profiles: ProfileStorage, path: impl AsRef<Path>) -> Result<(), HelperError> {
    let mut file = fs::File::create(&path)?;
    let content = serde_json::to_vec(&profiles)?;
    file.write_all(&content)?;
    file.flush()?;

    Ok(())
}
