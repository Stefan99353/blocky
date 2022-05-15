use super::{Extract, LibraryDownloads, Natives, Rule};
use crate::os::Arch;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub natives: Option<Natives>,
    #[serde(default)]
    pub rules: Vec<Rule>,
    pub extract: Option<Extract>,
}

impl Library {
    pub fn check_use(&self) -> bool {
        for rule in &self.rules {
            if !rule.allows() {
                return false;
            }
        }

        true
    }

    pub fn get_native(&self) -> Option<String> {
        let arch = Arch::current();

        if let Some(natives) = &self.natives {
            return natives
                .get_for_current_platform()
                .map(|n| n.replace("${arch}", &arch.get_bits().to_string()));
        }

        None
    }
}
