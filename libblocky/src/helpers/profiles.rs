use super::HelperError;
use crate::BlockyProfile;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

type ProfileStorage = HashMap<Uuid, BlockyProfile>;

pub async fn save_profile(
    profile: BlockyProfile,
    path: impl AsRef<Path>,
) -> Result<(), HelperError> {
    debug!("Saving a new profile to disk or updating existing one");

    let mut profiles = HashMap::new();

    if path.as_ref().exists() && path.as_ref().is_file() {
        debug!("Trying to read existing profiles");
        let profiles_string = fs::read_to_string(&path).await?;
        profiles = serde_json::from_str::<ProfileStorage>(&profiles_string)?;
    }

    let _old = profiles.insert(profile.uuid.clone(), profile);

    let mut file = fs::File::create(&path).await?;
    let profiles = serde_json::to_string(&profiles)?;
    fs::write(&path, profiles.as_bytes()).await?;

    Ok(())
}
