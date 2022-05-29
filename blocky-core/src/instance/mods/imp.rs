use crate::instance::mods::error::ModError;
use crate::instance::mods::loader_mod::LoaderMod;
use crate::instance::mods::{InstanceModsExt, ModIndex};
use crate::instance::Instance;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use uuid::Uuid;

impl InstanceModsExt for Instance {
    fn mod_index_path(&self) -> PathBuf {
        let mut path = self.available_mods_path();
        path.push("index.json");
        path
    }

    fn active_mods_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("mods");
        path
    }

    fn available_mods_path(&self) -> PathBuf {
        let mut path = self.instance_path();
        path.push("mods");
        path
    }

    fn load_mod_index(&self) -> crate::error::Result<ModIndex> {
        let mut result = ModIndex::default();
        let index_path = self.mod_index_path();

        if index_path.is_file() {
            let file = File::open(&index_path).map_err(ModError::IO)?;
            let reader = BufReader::new(file);
            result = serde_json::from_reader(reader).map_err(ModError::Serde)?;
        }

        Ok(result)
    }

    fn persist_mod_index(&self, index: &ModIndex) -> crate::error::Result<()> {
        let index_path = self.mod_index_path();
        let file = File::create(index_path).map_err(ModError::IO)?;
        serde_json::to_writer_pretty(&file, index).map_err(ModError::Serde)?;
        file.sync_all().map_err(ModError::IO)?;

        Ok(())
    }

    fn list_mods(&self) -> crate::error::Result<Vec<LoaderMod>> {
        let index = self.load_mod_index()?;

        let result = index.into_iter().map(|(_, m)| m).collect();

        Ok(result)
    }

    fn add_mod(&self, file: impl AsRef<Path>) -> crate::error::Result<LoaderMod> {
        let loader_mod = LoaderMod::from_file(&file)?;

        let mut available_path = self.available_mods_path();
        available_path.push(&loader_mod.filename);
        let mut source_mod_file = File::open(&file).map_err(ModError::IO)?;
        let mut target_mod_file = File::create(&available_path).map_err(ModError::IO)?;
        std::io::copy(&mut source_mod_file, &mut target_mod_file).map_err(ModError::IO)?;

        let mut index = self.load_mod_index()?;
        index.insert(loader_mod.uuid, loader_mod.clone());
        self.persist_mod_index(&index)?;

        Ok(loader_mod)
    }

    fn remove_mod(&self, uuid: Uuid) -> crate::error::Result<()> {
        self.disable_mod(uuid)?;
        let mut index = self.load_mod_index()?;
        let loader_mod = index.get_mut(&uuid).ok_or(ModError::NotFound(uuid))?;

        let mut available_path = self.available_mods_path();
        available_path.push(&loader_mod.filename);

        if available_path.is_file() {
            std::fs::remove_file(&available_path).map_err(ModError::IO)?;
        }

        index.remove(&uuid);
        self.persist_mod_index(&index)?;

        Ok(())
    }

    fn enable_mod(&self, uuid: Uuid) -> crate::error::Result<()> {
        let mut index = self.load_mod_index()?;
        let loader_mod = index.get_mut(&uuid).ok_or(ModError::NotFound(uuid))?;

        let mut available_path = self.available_mods_path();
        available_path.push(&loader_mod.filename);
        let mut active_path = self.active_mods_path();
        active_path.push(&loader_mod.filename);

        if !active_path.exists() {
            symlink::symlink_file(&available_path, &active_path).map_err(ModError::IO)?;
        }

        loader_mod.enabled = true;

        self.persist_mod_index(&index)?;

        Ok(())
    }

    fn disable_mod(&self, uuid: Uuid) -> crate::error::Result<()> {
        let mut index = self.load_mod_index()?;
        let loader_mod = index.get_mut(&uuid).ok_or(ModError::NotFound(uuid))?;

        if loader_mod.enabled {
            let mut active_path = self.active_mods_path();
            active_path.push(&loader_mod.filename);

            if active_path.is_symlink() {
                symlink::remove_symlink_file(&active_path).map_err(ModError::IO)?;
            }
        }

        loader_mod.enabled = false;

        self.persist_mod_index(&index)?;

        Ok(())
    }
}
