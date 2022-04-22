use crate::error::Error;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::path::Path;

pub fn download_file(url: &str, dest: impl AsRef<Path>) -> crate::error::Result<()> {
    debug!("Downloading file: {}", url);

    let mut response = reqwest::blocking::get(url)
        .map_err(Error::Request)?
        .error_for_status()
        .map_err(Error::Request)?;

    let mut file = File::create(&dest).map_err(Error::Filesystem)?;

    let _total_size = std::io::copy(&mut response, &mut file).map_err(Error::Filesystem)?;

    Ok(())
}

pub fn download_file_check(
    url: &str,
    dest: impl AsRef<Path>,
    sha1: Option<&[u8]>,
) -> crate::error::Result<()> {
    debug!("Download file if newer or not exists: {}", url);

    if dest.as_ref().exists() {
        // Check SHA1
        if let Some(sha1) = sha1 {
            let hash = get_sha1(&dest)?;
            if hash == sha1 {
                trace!("File exists in newest form: {}", url);
                return Ok(());
            } else {
                trace!("File exists but corrupt/outdated: {}", url);
                download_file(url, &dest)?;
            }
        } else {
            trace!("File exists: {}", url);
        }
    } else {
        // Download
        download_file(url, &dest)?;
    }

    if let Some(sha1) = sha1 {
        let hash = get_sha1(dest)?;
        if hash != sha1 {
            // Download is not correct
            return Err(Error::Sha1Mismatch(url.to_string()));
        }
    }

    Ok(())
}

fn get_sha1(file: impl AsRef<Path>) -> crate::error::Result<Vec<u8>> {
    debug!(
        "Generathing SHA1 for file {}",
        file.as_ref().to_string_lossy()
    );

    let mut file = File::open(&file).map_err(Error::Filesystem)?;
    let mut hasher: Sha1 = Sha1::new();
    std::io::copy(&mut file, &mut hasher).map_err(Error::Filesystem)?;
    let hash = hasher.finalize();

    Ok(hash.to_vec())
}
