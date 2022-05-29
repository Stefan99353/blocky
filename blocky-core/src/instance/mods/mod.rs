use crate::instance::mods::loader_mod::LoaderMod;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub mod error;
mod imp;
pub mod loader_mod;

pub type ModIndex = HashMap<Uuid, LoaderMod>;

pub trait InstanceModsExt {
    fn mod_index_path(&self) -> PathBuf;
    fn active_mods_path(&self) -> PathBuf;
    fn available_mods_path(&self) -> PathBuf;

    // Index
    fn load_mod_index(&self) -> crate::error::Result<ModIndex>;
    fn persist_mod_index(&self, index: &ModIndex) -> crate::error::Result<()>;

    // Management
    fn list_mods(&self) -> crate::error::Result<Vec<LoaderMod>>;
    fn add_mod(&self, file: impl AsRef<Path>) -> crate::error::Result<LoaderMod>;
    fn remove_mod(&self, uuid: Uuid) -> crate::error::Result<()>;
    fn enable_mod(&self, uuid: Uuid) -> crate::error::Result<()>;
    fn disable_mod(&self, uuid: Uuid) -> crate::error::Result<()>;
}
