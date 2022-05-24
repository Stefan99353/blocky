use crate::error::DownloadError;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::path::Path;

pub fn download_file(url: &str, dest: impl AsRef<Path>) -> Result<(), DownloadError> {
    trace!("Downloading file: {}", url);

    let mut response = reqwest::blocking::get(url)
        .map_err(DownloadError::Reqwest)?
        .error_for_status()
        .map_err(DownloadError::Reqwest)?;

    let mut file = File::create(&dest).map_err(DownloadError::IO)?;

    let _total_size = std::io::copy(&mut response, &mut file).map_err(DownloadError::IO)?;
    file.sync_all().map_err(DownloadError::IO)?;

    Ok(())
}

pub fn download_file_check(
    url: &str,
    dest: impl AsRef<Path>,
    remote_sha: Option<Vec<u8>>,
) -> Result<(), DownloadError> {
    trace!("Checked download of file: {}", url);

    if dest.as_ref().exists() {
        trace!("File already exists");

        match &remote_sha {
            None => {
                trace!("Existing file is assumed correct");
                return Ok(());
            }
            Some(remote_sha) => {
                let local_sha = get_sha1(&dest)?;

                if remote_sha == &local_sha {
                    trace!("Existing file is correct");
                    return Ok(());
                } else {
                    trace!("Existing file does not match checksum");
                    download_file(url, &dest)?;
                }
            }
        }
    } else {
        trace!("File does not exist yet");

        download_file(url, &dest)?;
    }

    if let Some(remote_sha) = &remote_sha {
        let local_sha = get_sha1(&dest)?;
        if remote_sha != &local_sha {
            return Err(DownloadError::Sha1Mismatch(url.to_string()));
        }
    }

    Ok(())
}

pub fn get_sha1(file: impl AsRef<Path>) -> Result<Vec<u8>, DownloadError> {
    trace!(
        "Generathing SHA1 for file {}",
        file.as_ref().to_string_lossy()
    );

    let mut file = File::open(&file).map_err(DownloadError::IO)?;
    let mut hasher: Sha1 = Sha1::new();
    std::io::copy(&mut file, &mut hasher).map_err(DownloadError::IO)?;
    let hash = hasher.finalize();

    Ok(hash.to_vec())
}
