use crate::minecraft::error::MinecraftError;
use crate::minecraft::fabric::version_data_extension::FabricVersionDataExtension;
use crate::minecraft::installation_update::{InstallationUpdate, Progress};
use crate::utils::download_file_check;
use crossbeam_channel::Sender;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn install_fabric_libraries(
    fabric_version_data: &FabricVersionDataExtension,
    libraries_path: impl AsRef<Path>,
    update_sender: Sender<InstallationUpdate>,
    cancel: Arc<AtomicBool>,
) -> Result<(), MinecraftError> {
    debug!("Install fabric libraries");
    trace!("Fabric Version: {}", &fabric_version_data.id);
    trace!(
        "Libraries Path: {}",
        libraries_path.as_ref().to_string_lossy()
    );

    let mut update_progress_template = Progress {
        total_files: fabric_version_data.libraries.len(),
        ..Default::default()
    };

    for (n, library) in fabric_version_data.libraries.iter().enumerate() {
        trace!("Fabric Library: {}", &library.name);

        // Check if canceled
        if cancel.load(Ordering::Relaxed) {
            trace!("Cancel installation");
            let _ = update_sender.send(InstallationUpdate::Cancel);
            return Ok(());
        }

        // Send update
        update_progress_template.current_file = n + 1;
        update_progress_template.current_file_url = library.url.clone();
        update_progress_template.current_file_size = None;
        let _ = update_sender.send(InstallationUpdate::FabricLibrary(
            update_progress_template.clone(),
        ));

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
        let mut jar_path = library_path;
        jar_path.push(&jar_name);

        let url = format!(
            "{}{}/{}/{}/{}",
            &library.url, &package, &name, &version, &jar_name
        );

        download_file_check(&url, &jar_path, None).map_err(MinecraftError::Download)?;
    }

    Ok(())
}
