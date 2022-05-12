use crate::error::Error;
use crate::Instance;
use std::fs;

impl Instance {
    pub fn remove(&self) -> crate::error::Result<()> {
        debug!("Removing instance");
        let instance_path = self.instance_path();
        fs::remove_dir_all(instance_path).map_err(Error::Filesystem)
    }
}
