use crate::Profile;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, Profile>;

pub fn load_profiles(path: impl AsRef<Path>) -> crate::error::Result<Vec<Profile>> {
    debug!("Reading profiles from disk");
    let profiles = read_file(&path)?;

    let profiles = profiles
        .into_iter()
        .map(|(_, profile)| profile)
        .collect::<Vec<Profile>>();

    Ok(profiles)
}

pub fn find_profile(uuid: Uuid, path: impl AsRef<Path>) -> crate::error::Result<Option<Profile>> {
    let profiles = read_file(&path)?;
    let profile = profiles.get(&uuid).cloned();
    Ok(profile)
}

pub fn save_profile(profile: Profile, path: impl AsRef<Path>) -> crate::error::Result<()> {
    debug!("Saving a new profile to disk or updating existing one");
    let mut profiles = read_file(&path)?;

    let _old = profiles.insert(profile.uuid, profile);

    write_file(profiles, path)
}

pub fn remove_profile(uuid: Uuid, path: impl AsRef<Path>) -> crate::error::Result<()> {
    debug!("Removing a profile from disk");
    let mut profiles = read_file(&path)?;

    let _old = profiles.remove(&uuid);

    write_file(profiles, path)
}

fn read_file(path: impl AsRef<Path>) -> crate::error::Result<ProfileStorage> {
    let mut profiles = HashMap::new();

    if path.as_ref().is_file() {
        let profiles_string = fs::read_to_string(&path).map_err(crate::error::Error::Filesystem)?;
        profiles = serde_json::from_str::<ProfileStorage>(&profiles_string)
            .map_err(crate::error::Error::Serde)?;
    }

    Ok(profiles)
}

fn write_file(profiles: ProfileStorage, path: impl AsRef<Path>) -> crate::error::Result<()> {
    let mut file = fs::File::create(&path).map_err(crate::error::Error::Filesystem)?;
    let content = serde_json::to_vec(&profiles).map_err(crate::error::Error::Serde)?;
    file.write_all(&content)
        .map_err(crate::error::Error::Filesystem)?;
    file.flush().map_err(crate::error::Error::Filesystem)?;

    Ok(())
}
