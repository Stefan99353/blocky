use crate::instance::Instance;
use crate::minecraft::fabric::install::install_fabric_libraries;
use crate::minecraft::fabric::launch::build_fabric_launch_command;
use crate::minecraft::fabric::version_data_extension::FabricVersionDataExtension;
use crate::minecraft::installation_update::InstallationUpdate;
use crate::minecraft::launch::launch;
use crate::minecraft::launch_options::LaunchOptions;
use crate::utils::download_file_check;
use crate::{consts, error};
use crossbeam_channel::Sender;
use std::fs;
use std::process::{Command, Stdio};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub trait FabricInstanceExt {
    fn save_fabric_version_data(&self) -> error::Result<()>;
    fn read_fabric_version_data(&self) -> error::Result<FabricVersionDataExtension>;
    fn install_fabric_libraries(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()>;
    fn fabric_launch_command(&self, options: &LaunchOptions) -> error::Result<Command>;
    fn fabric_launch(&self, options: &LaunchOptions) -> error::Result<()>;
}

impl FabricInstanceExt for Instance {
    fn save_fabric_version_data(&self) -> error::Result<()> {
        if !self.use_fabric {
            warn!("Instance is not using fabric");
            return Ok(());
        }

        let profile_url = format!(
            "{}/versions/loader/{}/{}/profile/json",
            consts::FABRIC_BASE_V2_URL,
            &self.version,
            &self.fabric_version
        );

        let fabric_version_data_path = self.fabric_version_data_path();
        download_file_check(&profile_url, &fabric_version_data_path, None)?;

        Ok(())
    }

    fn read_fabric_version_data(&self) -> error::Result<FabricVersionDataExtension> {
        let fabric_version_data_path = self.fabric_version_data_path();

        let version_data =
            fs::read_to_string(&fabric_version_data_path).map_err(error::Error::IO)?;
        let version_data = serde_json::from_str::<FabricVersionDataExtension>(&version_data)
            .map_err(error::Error::Serde)?;

        Ok(version_data)
    }

    fn install_fabric_libraries(
        &self,
        update_sender: Sender<InstallationUpdate>,
        cancel: Arc<AtomicBool>,
    ) -> error::Result<()> {
        if !self.use_fabric {
            warn!("Instance is not using fabric");
            return Ok(());
        }

        let fabric_version_data = self.read_fabric_version_data()?;

        install_fabric_libraries(
            &fabric_version_data,
            self.libraries_path(),
            update_sender,
            cancel,
        )?;

        Ok(())
    }

    fn fabric_launch_command(&self, options: &LaunchOptions) -> error::Result<Command> {
        let version_data = self.read_version_data()?;
        let fabric_version_data = self.read_fabric_version_data()?;

        let command = build_fabric_launch_command(
            &version_data,
            &fabric_version_data,
            self.dot_minecraft_path(),
            self.libraries_path(),
            self.assets_path(),
            self.natives_path(),
            self.log_configs_path(),
            options,
        )?;

        Ok(command)
    }

    fn fabric_launch(&self, options: &LaunchOptions) -> error::Result<()> {
        let version_data = self.read_version_data()?;
        let fabric_version_data = self.read_fabric_version_data()?;

        let mut command = build_fabric_launch_command(
            &version_data,
            &fabric_version_data,
            self.dot_minecraft_path(),
            self.libraries_path(),
            self.assets_path(),
            self.natives_path(),
            self.log_configs_path(),
            options,
        )?;
        command
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null());

        let _pid = launch(command)?;

        Ok(())
    }
}
