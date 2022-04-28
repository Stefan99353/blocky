use super::error::InstallationError;
use crate::consts;
use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::extract::extract_native;
use crate::instance::models::Library;
use crate::instance::resource_update::{ResourceInstallationUpdate, ResourceType};
use crate::Instance;
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

impl Instance {
    pub fn install_libraries(
        &self,
        sender: crossbeam_channel::Sender<crate::error::Result<ResourceInstallationUpdate>>,
    ) -> Result<(), Error> {
        debug!("Installing libraries");

        let version_data = self.read_version_data()?;

        // Filter libraries needed
        let libraries = version_data
            .libraries
            .iter()
            .filter(|l| l.check_use())
            .collect::<Vec<&Library>>();
        let total = libraries.len();

        for (n, library) in libraries.into_iter().enumerate() {
            // Get library information
            let (mut package, name, version): (String, String, String) = library
                .name
                .split(':')
                .map(|x| x.to_string())
                .collect_tuple()
                .ok_or(InstallationError::LibraryNameFormat)?;
            package = package.replace('.', "/");

            let mut library_path = self.libraries_path();
            library_path.push(&package);
            library_path.push(&name);
            library_path.push(&version);

            // Create path
            fs::create_dir_all(&library_path).map_err(Error::Filesystem)?;

            if let Some(artifact) = &library.downloads.artifact {
                // Safe library file
                let jar_name = format!("{}-{}.jar", name, version);
                let mut jar_path = library_path.clone();
                jar_path.push(&jar_name);

                // Send update
                let _ = sender.send(Ok(ResourceInstallationUpdate {
                    resource_type: ResourceType::Library,
                    url: artifact.url.to_string(),
                    total,
                    n,
                    size: Some(artifact.size),
                }));
                let sha = hex::decode(&artifact.sha1)?;
                download_file_check(&artifact.url, &jar_path, Some(sha))?;
            }

            // Native library
            if let Some(native) = &library.get_native() {
                let native_jar_name = format!("{}-{}-{}.jar", name, version, native);
                let mut native_jar_path = library_path.clone();
                native_jar_path.push(&native_jar_name);

                // TODO: Check SHA1
                let native_download_url = format!(
                    "{}/{}/{}/{}/{}",
                    consts::MC_LIBRARIES_BASE_URL,
                    &package,
                    &name,
                    &version,
                    &native_jar_name
                );
                let _ = sender.send(Ok(ResourceInstallationUpdate {
                    resource_type: ResourceType::Library,
                    url: native_download_url.to_string(),
                    total,
                    n,
                    size: None,
                }));
                download_file_check(&native_download_url, &native_jar_path, None)?;

                // Extract
                if let Some(extract) = &library.extract {
                    let mut natives_dir = PathBuf::from(&self.instance_path);
                    natives_dir.push("natives");
                    extract_native(&native_jar_path, natives_dir, &extract.exclude)?;
                }
            }
        }

        Ok(())
    }
}
