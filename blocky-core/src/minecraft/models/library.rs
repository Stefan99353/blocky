use crate::minecraft::error::MinecraftError;
use crate::minecraft::models::extract::Extract;
use crate::minecraft::models::library_downloads::LibraryDownloads;
use crate::minecraft::models::natives::Natives;
use crate::minecraft::models::rule::Rule;
use crate::os::Architecture;
use itertools::Itertools;
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

    pub fn extract_information(&self) -> Result<(String, String, String), MinecraftError> {
        self.name
            .split(':')
            .map(|x| x.to_string())
            .collect_tuple()
            .ok_or(MinecraftError::LibraryNameFormat)
    }

    pub fn get_native(&self) -> Option<String> {
        let arch = Architecture::current();

        if let Some(natives) = &self.natives {
            return natives
                .get_for_current_platform()
                .map(|n| n.replace("${arch}", &arch.get_bits().to_string()));
        }

        None
    }
}
