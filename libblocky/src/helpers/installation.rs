use crate::helpers::HelperError;
use crate::instance::resource_update::ResourceInstallationUpdate;
use crate::Instance;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

pub fn install_threaded(
    instance_uuid: Uuid,
    instances_path: String,
    cancel: Arc<AtomicBool>,
) -> crossbeam_channel::Receiver<crate::error::Result<Option<ResourceInstallationUpdate>>> {
    let (tx, rx) = crossbeam_channel::unbounded();
    thread::spawn(move || install(instance_uuid, instances_path, tx, cancel));
    rx
}

pub fn check_install_state(
    instance_uuid: Uuid,
    instances_path: String,
) -> crate::error::Result<bool> {
    let instance = super::find_instance(instance_uuid, instances_path)?
        .ok_or_else(|| crate::error::Error::InstanceNotFound(instance_uuid))?;

    instance.check_installed()
}

fn install(
    uuid: Uuid,
    path: impl AsRef<Path>,
    sender: crossbeam_channel::Sender<crate::error::Result<Option<ResourceInstallationUpdate>>>,
    cancel: Arc<AtomicBool>,
) {
    let instance = match super::find_instance(uuid, path) {
        Ok(instance) => {
            if instance.is_none() {
                let _ = sender.send(Err(crate::error::Error::InstanceNotFound(uuid)));
                return;
            }

            instance.unwrap()
        }
        Err(err) => {
            let _ = sender.send(Err(err.into()));
            return;
        }
    };

    instance.full_install(sender, cancel);
}
