use crate::instance::launch_options::GlobalLaunchOptions;
use crate::Profile;
use std::process::Stdio;
use uuid::Uuid;

pub fn launch_instance(
    instance_uuid: Uuid,
    instances_path: String,
    profile_uuid: Uuid,
    profiles_path: String,
    options: GlobalLaunchOptions,
) -> crate::error::Result<()> {
    let instance = super::find_instance(instance_uuid, instances_path)?
        .ok_or_else(|| crate::error::Error::InstanceNotFound(instance_uuid))?;
    let profile = super::find_profile(profile_uuid, profiles_path)?
        .ok_or_else(|| crate::error::Error::ProfileNotFound(profile_uuid))?;

    instance.launch(&profile, &options)
}
