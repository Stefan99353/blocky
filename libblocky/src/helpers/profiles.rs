use super::HelperError;
use crate::BlockyProfile;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, BlockyProfile>;

pub async fn load_profiles(path: impl AsRef<Path>) -> Result<Vec<BlockyProfile>, HelperError> {
    debug!("Reading profiles from disk");
    let profiles = read_file(&path).await?;

    let profiles = profiles
        .into_iter()
        .map(|(_, profile)| profile)
        .collect::<Vec<BlockyProfile>>();

    Ok(profiles)
}

pub async fn find_profile(
    uuid: Uuid,
    path: impl AsRef<Path>,
) -> Result<Option<BlockyProfile>, HelperError> {
    let profiles = read_file(&path).await?;
    let profile = profiles.get(&uuid).cloned();
    Ok(profile)
}

pub async fn save_profile(
    profile: BlockyProfile,
    path: impl AsRef<Path>,
) -> Result<(), HelperError> {
    debug!("Saving a new profile to disk or updating existing one");
    let mut profiles = read_file(&path).await?;

    let _old = profiles.insert(profile.uuid, profile);

    let profiles = serde_json::to_string(&profiles)?;
    fs::write(&path, profiles.as_bytes()).await?;

    Ok(())
}

async fn read_file(path: impl AsRef<Path>) -> Result<ProfileStorage, HelperError> {
    let mut profiles = HashMap::new();

    if path.as_ref().is_file() {
        debug!("Trying to read existing profiles");
        let profiles_string = fs::read_to_string(&path).await?;
        profiles = serde_json::from_str::<ProfileStorage>(&profiles_string)?;
    }

    Ok(profiles)
}
