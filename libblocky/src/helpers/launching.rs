use crate::instance::launch_options::GlobalLaunchOptions;
use chrono::Utc;
use uuid::Uuid;

pub fn launch_instance(
    instance_uuid: Uuid,
    instances_path: String,
    profile_uuid: Uuid,
    profiles_path: String,
    options: GlobalLaunchOptions,
    client_id: &str,
    client_secret: &str,
) -> crate::error::Result<()> {
    let instance = super::find_instance(instance_uuid, instances_path)?
        .ok_or(crate::error::Error::InstanceNotFound(instance_uuid))?;
    let mut profile = super::find_profile(profile_uuid, &profiles_path)?
        .ok_or(crate::error::Error::ProfileNotFound(profile_uuid))?;

    warn!("Checking if token expired");
    if let Some(minecraft_token) = &profile.minecraft {
        if let Some(exp) = &minecraft_token.exp {
            let now = Utc::now();
            warn!("Now: {}", &now);
            warn!("Exp: {}", &exp);
            if exp <= &now {
                profile.refresh(client_id, client_secret)?;
                crate::helpers::save_profile(profile.clone(), &profiles_path)?;
            }
        }
    }

    instance.launch(&profile, &options)
}
