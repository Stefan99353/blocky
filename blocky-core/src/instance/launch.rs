use crate::error;
use crate::instance::Instance;
use crate::minecraft::launch::{build_launch_command, launch};
use crate::minecraft::launch_options::LaunchOptions;
use std::process::{Command, Stdio};

impl Instance {
    pub fn launch_command(&self, options: &LaunchOptions) -> error::Result<Command> {
        let version_data = self.read_version_data()?;

        let command = build_launch_command(
            &version_data,
            self.dot_minecraft_path(),
            self.libraries_path(),
            self.assets_path(),
            self.natives_path(),
            self.log_configs_path(),
            options,
        )?;

        Ok(command)
    }

    pub fn launch(&self, options: &LaunchOptions) -> error::Result<()> {
        let version_data = self.read_version_data()?;

        let mut command = build_launch_command(
            &version_data,
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
