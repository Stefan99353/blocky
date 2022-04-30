use crate::instance::launch_options::GlobalLaunchOptions;
use crate::Profile;
use std::process::Stdio;
use uuid::Uuid;

pub fn launch_instance(
    instance_uuid: Uuid,
    instances_path: String,
    profile: &Profile,
    options: &GlobalLaunchOptions,
) {
    let instance = match super::find_instance(instance_uuid, instances_path) {
        Ok(instance) => {
            if instance.is_none() {
                error!("Instance not found");
                return;
            }

            instance.unwrap()
        }
        Err(err) => {
            error!("Error reading instances");
            return;
        }
    };

    // TODO: Unwrap & fork
    let mut command = instance.launch_command(profile, options).unwrap();
    debug!("{:?}", &command);
    command.stdout(Stdio::null());
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}
