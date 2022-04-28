use crate::instance::launch::argument_replacements::ArgumentReplacements;
use crate::instance::launch::error::LaunchError;
use crate::instance::launch::launch_options::GlobalLaunchOptions;
use crate::instance::launch::utils::game_arguments;
use crate::profile::AuthenticationError;
use crate::{Instance, Profile};
use glib::types::instance_of;
use std::process::Command;
use utils::{classpath, java_executable, jvm_arguments};

mod argument_replacements;
pub mod error;
mod launch_options;
mod utils;

impl Instance {
    pub fn launch_command(
        &self,
        profile: &Profile,
        options: &GlobalLaunchOptions,
    ) -> crate::error::Result<Command> {
        let minecraft_profile = profile
            .minecraft_profile
            .as_ref()
            .ok_or(LaunchError::MissingProfile)?;
        let minecraft_token = &profile
            .minecraft
            .as_ref()
            .ok_or(LaunchError::Unauthenticated)?
            .token;

        let version_data = self.read_version_data()?;
        let classpath = classpath(self, &version_data)?;
        let arg_replacers = ArgumentReplacements::build(
            self,
            minecraft_profile,
            minecraft_token,
            options,
            &version_data,
            classpath,
        );
        let jvm_arguments = jvm_arguments(self, options, &version_data, &arg_replacers);
        let game_arguments = game_arguments(&version_data, &arg_replacers);

        // Build command
        let mut command = Command::new(java_executable(&self, &options));
        command.current_dir(&self.dot_minecraft_path());

        // Add JVM Arguments
        command.args(jvm_arguments);

        // Add logging config
        if let Some(logging) = &version_data.logging {
            let mut logger_config_file = self.log_configs_path();
            logger_config_file.push(&logging.client.file.id);

            let argument = &logging
                .client
                .argument
                .replace("${path}", &logger_config_file.to_string_lossy());
            command.arg(argument);
        }

        // Add main class
        command.arg(&version_data.main_class);

        // Add minecraft args
        command.args(game_arguments);

        Ok(command)
    }
}
