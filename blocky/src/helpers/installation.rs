use crate::helpers::find_instance;
use anyhow::anyhow;
use blocky_core::minecraft::installation_update::InstallationUpdate;
use crossbeam_channel::{Receiver, Sender};
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

pub fn install_threaded(
    instance_uuid: Uuid,
    instances_path: String,
    cancel: Arc<AtomicBool>,
) -> Receiver<InstallationUpdate> {
    let (tx, rx) = crossbeam_channel::unbounded();
    thread::spawn(move || install(instance_uuid, instances_path, tx, cancel));
    rx
}

fn install(
    instance_uuid: Uuid,
    instances_path: impl AsRef<Path>,
    sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let instance = find_instance(instance_uuid, instances_path)?
        .ok_or(anyhow!("Instance not found: {}", instance_uuid))?;

    instance.full_install(sender, cancel);

    Ok(())
}

pub fn check_install_state(instance_uuid: Uuid, instances_path: String) -> anyhow::Result<bool> {
    let instance = find_instance(instance_uuid, instances_path)?
        .ok_or(anyhow!("Instance not found: {}", instance_uuid))?;
    let installed = instance.check_installed()?;

    Ok(installed)
}
