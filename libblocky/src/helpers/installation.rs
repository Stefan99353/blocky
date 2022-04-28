use crate::helpers::HelperError;
use crate::instance::resource_update::ResourceInstallationUpdate;
use crate::Instance;
use std::path::Path;
use std::thread;
use uuid::Uuid;

pub fn install_threaded(
    instance_uuid: Uuid,
    instances_path: String,
) -> crossbeam_channel::Receiver<crate::error::Result<ResourceInstallationUpdate>> {
    let (tx, rx) = crossbeam_channel::unbounded();
    thread::spawn(move || install(instance_uuid, instances_path, tx));
    rx
}

fn install(
    uuid: Uuid,
    path: impl AsRef<Path>,
    sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
) {
    let instance = match super::find_instance(uuid, path) {
        Ok(instance) => {
            if instance.is_none() {
                error!("Instance not found");
                return;
            }

            instance.unwrap()
        }
        Err(err) => {
            let _ = sender.send(Err(err.into()));
            return;
        }
    };

    instance.full_install(sender);
}
