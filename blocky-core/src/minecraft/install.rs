use crate::consts;
use crate::minecraft::error::MinecraftError;
use crate::minecraft::installation_update::{InstallationUpdate, Progress};
use crate::minecraft::models::asset_index_data::AssetIndexData;
use crate::minecraft::models::version_data::VersionData;
use crate::minecraft::utils::extract_native;
use crate::utils::download_file_check;
use crossbeam_channel::Sender;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn install_libraries(
    version_data: &VersionData,
    libraries_path: impl AsRef<Path>,
    natives_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install libraries");
    trace!("Version: {}", &version_data.id);
    trace!(
        "Libraries Path: {}",
        libraries_path.as_ref().to_string_lossy()
    );
    trace!("Natives Path: {}", natives_path.as_ref().to_string_lossy());

    let needed_libraries = version_data.needed_libraries();
    let mut update_progress_template = Progress {
        total_files: needed_libraries.len(),
        ..Default::default()
    };

    for (n, library) in needed_libraries.into_iter().enumerate() {
        trace!("Library: {}", &library.name);
        let artifact = &library.downloads.artifact;

        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        update_progress_template.current_file = n + 1;
        update_progress_template.current_file_url = artifact.url.clone();
        update_progress_template.current_file_size = Some(artifact.size);
        let _ = update_sender.send(InstallationUpdate::Library(
            update_progress_template.clone(),
        ));

        // Extract library information
        let (mut package, name, version) = library.extract_information()?;
        package = package.replace('.', "/");

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        library_path.push(&package);
        library_path.push(&name);
        library_path.push(&version);

        // Create path
        fs::create_dir_all(&library_path).map_err(MinecraftError::IO)?;

        // Download library file
        let jar_name = format!("{}-{}.jar", name, version);
        let mut jar_path = library_path.clone();
        jar_path.push(&jar_name);

        let jar_sha = hex::decode(&artifact.sha1).map_err(MinecraftError::Sha1Decode)?;
        download_file_check(&artifact.url, &jar_path, Some(jar_sha))
            .map_err(MinecraftError::Download)?;

        // Native
        if let Some(native) = library.get_native() {
            trace!("Native: {}", &library.name);
            let native_jar_name = format!("{}-{}-{}.jar", name, version, native);
            let mut native_jar_path = library_path.clone();
            native_jar_path.push(&native_jar_name);

            let native_download_url = format!(
                "{}/{}/{}/{}/{}",
                consts::MC_LIBRARIES_BASE_URL,
                &package,
                &name,
                &version,
                &native_jar_name
            );

            // Send update
            update_progress_template.current_file_url = native_download_url.clone();
            update_progress_template.current_file_size = None;
            let _ = update_sender.send(InstallationUpdate::Library(
                update_progress_template.clone(),
            ));

            // TODO: Check SHA1
            download_file_check(&artifact.url, &native_jar_path, None)
                .map_err(MinecraftError::Download)?;

            // Extract
            if let Some(extract) = &library.extract {
                extract_native(&native_jar_path, &natives_path, &extract.exclude)?;
            }
        }
    }

    Ok(())
}

pub fn install_assets(
    asset_index: &AssetIndexData,
    assets_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install assets");
    trace!("Assets Path: {}", assets_path.as_ref().to_string_lossy());

    let mut objects_path = PathBuf::from(assets_path.as_ref());
    objects_path.push("objects");

    let mut update_progress_template = Progress {
        total_files: asset_index.objects.len(),
        ..Default::default()
    };

    for (n, asset_info) in asset_index.objects.values().enumerate() {
        trace!("Asset: {}", &asset_info.hash);
        let hash_part: String = asset_info.hash.chars().take(2).collect();
        let asset_url = format!(
            "{}/{}/{}",
            consts::MC_ASSETS_BASE_URL,
            &hash_part,
            &asset_info.hash
        );

        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        update_progress_template.current_file = n + 1;
        update_progress_template.current_file_url = asset_url.clone();
        update_progress_template.current_file_size = Some(asset_info.size);
        let _ = update_sender.send(InstallationUpdate::Asset(update_progress_template.clone()));

        let mut asset_path = objects_path.clone();
        asset_path.push(&hash_part);

        // Create path
        fs::create_dir_all(&asset_path).map_err(MinecraftError::IO)?;

        asset_path.push(&asset_info.hash);

        let asset_sha = hex::decode(&asset_info.hash).map_err(MinecraftError::Sha1Decode)?;
        download_file_check(&asset_url, &asset_path, Some(asset_sha))
            .map_err(MinecraftError::Download)?;
    }

    Ok(())
}

pub fn install_resources(
    asset_index: &AssetIndexData,
    resources_path: impl AsRef<Path>,
    minecraft_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install resources");
    trace!(
        "Resources Path: {}",
        resources_path.as_ref().to_string_lossy()
    );
    trace!(
        "Minecraft Path: {}",
        minecraft_path.as_ref().to_string_lossy()
    );

    let mut update_progress_template = Progress {
        total_files: asset_index.objects.len(),
        ..Default::default()
    };

    for (n, (key, asset_info)) in asset_index.objects.iter().enumerate() {
        trace!("Resource: {}", key);
        let hash_part: String = asset_info.hash.chars().take(2).collect();
        let asset_url = format!(
            "{}/{}/{}",
            consts::MC_ASSETS_BASE_URL,
            &hash_part,
            &asset_info.hash
        );

        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        update_progress_template.current_file = n + 1;
        update_progress_template.current_file_url = asset_url.clone();
        update_progress_template.current_file_size = Some(asset_info.size);
        let _ = update_sender.send(InstallationUpdate::Asset(update_progress_template.clone()));

        // Create path
        let mut resource_path = PathBuf::from(resources_path.as_ref());
        resource_path.push(key);
        resource_path.pop();
        fs::create_dir_all(&resource_path).map_err(MinecraftError::IO)?;

        let mut resource_path = PathBuf::from(resources_path.as_ref());
        resource_path.push(key);

        let resource_sha = hex::decode(&asset_info.hash).map_err(MinecraftError::Sha1Decode)?;
        download_file_check(&asset_url, &resource_path, Some(resource_sha))
            .map_err(MinecraftError::Download)?;
    }

    // Create symlink
    let mut minecraft_resources_path = PathBuf::from(minecraft_path.as_ref());
    minecraft_resources_path.push("resources");
    if !minecraft_resources_path.exists() {
        trace!("Creating symlink for resources");
        symlink::symlink_dir(&resources_path, minecraft_resources_path)
            .map_err(MinecraftError::IO)?;
    }

    Ok(())
}

pub fn install_log_config(
    version_data: &VersionData,
    log_configs_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install log config");
    trace!("Version: {}", &version_data.id);
    trace!(
        "Log Configs Path: {}",
        log_configs_path.as_ref().to_string_lossy()
    );

    if let Some(logging_info) = &version_data.logging {
        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        let update_progress = Progress {
            total_files: 1,
            current_file: 1,
            current_file_url: logging_info.client.file.url.clone(),
            current_file_size: Some(logging_info.client.file.size),
        };
        let _ = update_sender.send(InstallationUpdate::LogConfig(update_progress));

        // Create path
        fs::create_dir_all(&log_configs_path).map_err(MinecraftError::IO)?;

        let mut config_path = PathBuf::from(log_configs_path.as_ref());
        config_path.push(
            &logging_info
                .client
                .file
                .id
                .as_ref()
                .expect("Logging info has no ID"),
        );

        let config_sha =
            hex::decode(&logging_info.client.file.sha1).map_err(MinecraftError::Sha1Decode)?;
        download_file_check(
            &logging_info.client.file.url,
            &config_path,
            Some(config_sha),
        )
        .map_err(MinecraftError::Download)?;
    }

    Ok(())
}

pub fn install_client(
    version_data: &VersionData,
    minecraft_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install client");
    trace!("Version: {}", &version_data.id);
    trace!(
        "Minecraft Path: {}",
        minecraft_path.as_ref().to_string_lossy()
    );

    if let Some(downloads) = &version_data.downloads {
        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        let update_progress = Progress {
            total_files: 1,
            current_file: 1,
            current_file_url: downloads.client.url.clone(),
            current_file_size: Some(downloads.client.size),
        };
        let _ = update_sender.send(InstallationUpdate::Client(update_progress));

        let mut minecraft_path = PathBuf::from(minecraft_path.as_ref());
        minecraft_path.push("bin");

        // Create path
        fs::create_dir_all(&minecraft_path).map_err(MinecraftError::IO)?;

        minecraft_path.push(format!("minecraft-{}-client.jar", &version_data.id));

        let client_sha = hex::decode(&downloads.client.sha1).map_err(MinecraftError::Sha1Decode)?;
        download_file_check(&downloads.client.url, &minecraft_path, Some(client_sha))
            .map_err(MinecraftError::Download)?;
    }

    Ok(())
}
