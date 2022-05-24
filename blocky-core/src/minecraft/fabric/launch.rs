use crate::minecraft::argument_replacements::ArgumentReplacements;
use crate::minecraft::error::MinecraftError;
use crate::minecraft::fabric::version_data_extension::FabricVersionDataExtension;
use crate::minecraft::launch::{
    build_classpath, build_game_args, build_jvm_args, get_java_exec, launch_command,
};
use crate::minecraft::launch_options::LaunchOptions;
use crate::minecraft::models::version_data::VersionData;
use crate::os::Platform;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(clippy::too_many_arguments)]
pub fn build_fabric_launch_command(
    version_data: &VersionData,
    fabric_version_data_extension: &FabricVersionDataExtension,
    minecraft_path: impl AsRef<Path>,
    libraries_path: impl AsRef<Path>,
    assets_path: impl AsRef<Path>,
    natives_path: impl AsRef<Path>,
    log_configs_path: impl AsRef<Path>,
    launch_options: &LaunchOptions,
) -> Result<Command, MinecraftError> {
    debug!("Building fabric launch command");
    trace!("Version: {}", &version_data.id);
    trace!("Fabric Version: {}", &fabric_version_data_extension.id);
    trace!(
        "Minecraft Path: {}",
        minecraft_path.as_ref().to_string_lossy()
    );
    trace!(
        "Libraries Path: {}",
        libraries_path.as_ref().to_string_lossy()
    );
    trace!("Assets Path: {}", assets_path.as_ref().to_string_lossy());
    trace!("Natives Path: {}", natives_path.as_ref().to_string_lossy());
    trace!(
        "Log Configs Path: {}",
        log_configs_path.as_ref().to_string_lossy()
    );

    let mut classpath = build_classpath(version_data, &minecraft_path, &libraries_path)?;
    extend_classpath(
        &mut classpath,
        fabric_version_data_extension,
        &libraries_path,
    )?;

    let argument_replacements = ArgumentReplacements::build(
        launch_options,
        version_data,
        classpath,
        minecraft_path.as_ref().to_string_lossy().to_string(),
        assets_path.as_ref().to_string_lossy().to_string(),
        natives_path.as_ref().to_string_lossy().to_string(),
    );

    let mut jvm_args = build_jvm_args(version_data, &argument_replacements, launch_options);
    extend_jvm_args(
        &mut jvm_args,
        fabric_version_data_extension,
        &argument_replacements,
    );
    let mut game_args = build_game_args(version_data, &argument_replacements, launch_options);
    extend_game_args(
        &mut game_args,
        fabric_version_data_extension,
        &argument_replacements,
    );

    // Logging config
    let log_argument = version_data.logging.clone().map(|logging_info| {
        let mut log_config_path = PathBuf::from(log_configs_path.as_ref());
        log_config_path.push(
            logging_info
                .client
                .file
                .id
                .as_ref()
                .expect("Logging info has no ID"),
        );

        logging_info
            .client
            .argument
            .clone()
            .replace("${path}", &log_config_path.to_string_lossy())
    });

    let command = launch_command(
        get_java_exec(launch_options),
        minecraft_path,
        jvm_args,
        log_argument,
        fabric_version_data_extension.main_class.clone(),
        game_args,
    );

    Ok(command)
}

fn extend_game_args(
    game_args: &mut Vec<String>,
    fabric_version_data_extension: &FabricVersionDataExtension,
    argument_replacements: &ArgumentReplacements,
) {
    debug!("Extending Minecraft Args");

    if let Some(args) = &fabric_version_data_extension.arguments {
        for argument in args.game_arguments() {
            game_args.push(argument_replacements.replace(&argument));
        }
    }
}

fn extend_jvm_args(
    jvm_args: &mut Vec<String>,
    fabric_version_data_extension: &FabricVersionDataExtension,
    argument_replacements: &ArgumentReplacements,
) {
    debug!("Extending JVM Args");

    if let Some(args) = &fabric_version_data_extension.arguments {
        for argument in args.jvm_arguments() {
            jvm_args.push(argument_replacements.replace(&argument));
        }
    }
}

fn extend_classpath(
    classpath: &mut String,
    fabric_version_data_extension: &FabricVersionDataExtension,
    libraries_path: impl AsRef<Path>,
) -> Result<(), MinecraftError> {
    debug!("Extending classpath");

    let mut classes: Vec<String> = vec![];

    for library in &fabric_version_data_extension.libraries {
        let (mut package, name, version) = library.extract_information()?;
        package = package.replace('.', "/");

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        library_path.push(&package);
        library_path.push(&name);
        library_path.push(&version);

        let jar_name = format!("{}-{}.jar", name, version);
        library_path.push(jar_name);

        classes.push(library_path.to_string_lossy().to_string());
    }

    let classes = classes.join(&Platform::current().classpath_seperator().to_string());
    classpath.push(Platform::current().classpath_seperator());
    classpath.push_str(&classes);

    Ok(())
}
