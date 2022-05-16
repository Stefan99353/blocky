use crate::os::Platform;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Natives {
    pub linux: Option<String>,
    pub windows: Option<String>,
    pub osx: Option<String>,
}

impl Natives {
    pub fn get_for_current_platform(&self) -> Option<String> {
        let current = Platform::current();

        match current {
            Platform::Linux => self.linux.clone(),
            Platform::Windows => self.windows.clone(),
            Platform::MacOs => self.osx.clone(),
            Platform::Other => None,
        }
    }
}
