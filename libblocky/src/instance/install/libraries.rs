use super::error::InstallationError;
use crate::consts;
use crate::error::Error;
use crate::instance::download::download_file_check;
use crate::instance::extract::extract_native;
use crate::Instance;
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

impl Instance {
    pub fn install_libraries(&self) -> Result<(), Error> {
        debug!("Installing libraries");

        let version_data = self.read_version_data()?;

        // Filter libraries needed
        for library in version_data.libraries.iter().filter(|l| l.check_use()) {
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

            // Safe library file
            let jar_name = format!("{}-{}.jar", name, version);
            let mut jar_path = library_path.clone();
            jar_path.push(&jar_name);
            download_file_check(
                &library.downloads.artifact.url,
                &jar_path,
                Some(library.downloads.artifact.sha1.as_bytes()),
            )?;

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
