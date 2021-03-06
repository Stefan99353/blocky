use crate::minecraft::error::MinecraftError;
use std::fs;
use std::path::{Path, PathBuf};

pub fn extract_native(
    file: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    excludes: &[String],
) -> Result<(), MinecraftError> {
    debug!("Extracting natives of {}", file.as_ref().to_string_lossy());

    // Create destination folder
    fs::create_dir_all(&destination).map_err(MinecraftError::IO)?;

    // Open file
    let zip_file = fs::File::open(&file).map_err(MinecraftError::IO)?;
    let mut archive = zip::ZipArchive::new(zip_file).map_err(MinecraftError::Extract)?;

    'files: for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let zip_path = match file.enclosed_name() {
            None => continue 'files,
            Some(path) => path.to_owned(),
        };

        // Check excludes
        for exclude in excludes {
            if zip_path.starts_with(exclude) {
                continue 'files;
            }
        }

        let mut out_path = PathBuf::from(destination.as_ref());
        out_path.push(zip_path);

        if out_path.is_file() && file.is_file() {
            // File exists => skip
            continue 'files;
        }

        if file.is_file() {
            // File
            trace!("Extract file to: {}", out_path.to_string_lossy());
            let mut out_file = fs::File::create(&out_path).map_err(MinecraftError::IO)?;
            std::io::copy(&mut file, &mut out_file).map_err(MinecraftError::IO)?;
        } else {
            // Dir
            fs::create_dir_all(&out_path).map_err(MinecraftError::IO)?;
        }
    }

    trace!("Finished extracting");

    Ok(())
}
