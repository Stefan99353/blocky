use super::HelperError;
use crate::Instance;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

type InstanceStorage = HashMap<Uuid, Instance>;

pub fn load_instances(path: impl AsRef<Path>) -> Result<Vec<Instance>, HelperError> {
    debug!("Reading instances from disk");
    let instances = read_file(&path)?;

    let instances = instances
        .into_iter()
        .map(|(_, instance)| instance)
        .collect::<Vec<Instance>>();

    Ok(instances)
}

pub fn find_instance(uuid: Uuid, path: impl AsRef<Path>) -> Result<Option<Instance>, HelperError> {
    let instances = read_file(&path)?;
    let instance = instances.get(&uuid).cloned();
    Ok(instance)
}

pub fn save_instance(instance: Instance, path: impl AsRef<Path>) -> Result<(), HelperError> {
    debug!("Saving a new instance to disk or updating existing one");
    let mut instances = read_file(&path)?;

    let _old = instances.insert(instance.uuid, instance);

    write_file(instances, path)
}

pub fn remove_instance(uuid: Uuid, path: impl AsRef<Path>) -> Result<(), HelperError> {
    debug!("Removing an instance from disk");
    let mut instances = read_file(&path)?;

    let _old = instances.remove(&uuid);

    write_file(instances, path)
}

fn read_file(path: impl AsRef<Path>) -> Result<InstanceStorage, HelperError> {
    let mut instances = HashMap::new();

    if path.as_ref().is_file() {
        let instances_string = fs::read_to_string(&path)?;
        instances = serde_json::from_str::<InstanceStorage>(&instances_string)?;
    }

    Ok(instances)
}

fn write_file(instances: InstanceStorage, path: impl AsRef<Path>) -> Result<(), HelperError> {
    let mut file = fs::File::create(&path)?;
    let content = serde_json::to_vec(&instances)?;
    file.write_all(&content)?;
    file.flush()?;

    Ok(())
}
