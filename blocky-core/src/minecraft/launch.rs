use crate::minecraft::argument_replacements::ArgumentReplacements;
use crate::minecraft::error::MinecraftError;
use crate::minecraft::launch_options::LaunchOptions;
use crate::minecraft::models::version_data::VersionData;
use crate::os::Platform;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn launch(mut command: Command) -> Result<i32, MinecraftError> {
    debug!("Launching minecraft");
    trace!("{:?}", command);

    match fork::fork() {
        Ok(fork::Fork::Parent(id)) => {
            return Ok(id);
        }
        Ok(fork::Fork::Child) => {
            let mut child = command.spawn().expect("Could not spawn new command");
            let status = child.wait().expect("Could not wait for command to finish");

            if !status.success() {
                warn!("Game exited with status '{:?}'", status.code());
            } else {
                debug!("Game exited successfully");
            }
        }
        Err(_) => {
            return Err(MinecraftError::Forking);
        }
    }

    Ok(0)
}

pub fn launch_command(
    version_data: &VersionData,
    minecraft_path: impl AsRef<Path>,
    libraries_path: impl AsRef<Path>,
    assets_path: impl AsRef<Path>,
    natives_path: impl AsRef<Path>,
    log_configs_path: impl AsRef<Path>,
    launch_options: &LaunchOptions,
) -> Result<Command, MinecraftError> {
    debug!("Building launch command");
    trace!("Version: {}", &version_data.id);
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

    let classpath = build_classpath(version_data, &minecraft_path, &libraries_path)?;
    let argument_replacements = ArgumentReplacements::build(
        launch_options,
        version_data,
        classpath,
        minecraft_path.as_ref().to_string_lossy().to_string(),
        assets_path.as_ref().to_string_lossy().to_string(),
        natives_path.as_ref().to_string_lossy().to_string(),
    );
    let jvm_args = build_jvm_args(version_data, &argument_replacements, launch_options);
    let game_args = build_game_args(version_data, &argument_replacements, launch_options);

    // Build command
    let mut command = Command::new(get_java_exec(launch_options));
    command.current_dir(&minecraft_path);

    // JVM arguments
    command.args(jvm_args);

    // Logging config
    if let Some(logging_info) = &version_data.logging {
        let mut log_config = PathBuf::from(log_configs_path.as_ref());
        log_config.push(
            logging_info
                .client
                .file
                .id
                .as_ref()
                .expect("Logging info has no ID"),
        );

        let argument = &logging_info
            .client
            .argument
            .replace("${path}", &log_config.to_string_lossy());
        command.arg(argument);
    }

    // Main class
    command.arg(&version_data.main_class);

    // Minecraft args
    command.args(game_args);

    Ok(command)
}

fn get_java_exec(launch_options: &LaunchOptions) -> String {
    if !launch_options.java_exec.is_empty() {
        return launch_options.java_exec.clone();
    }

    String::from("java")
}

fn build_game_args(
    version_data: &VersionData,
    argument_replacements: &ArgumentReplacements,
    launch_options: &LaunchOptions,
) -> Vec<String> {
    debug!("Building Minecraft Args");

    let mut arguments = vec![];

    // New style
    if let Some(args) = &version_data.arguments {
        for argument in args.game_arguments() {
            arguments.push(argument_replacements.replace(&argument));
        }
    }

    // Old style
    if let Some(args) = &version_data.minecraft_arguments {
        for argument in args.split_whitespace() {
            arguments.push(argument_replacements.replace(argument));
        }
    }

    if launch_options.use_fullscreen {
        arguments.push("--fullscreen".to_string());
    } else if launch_options.enable_window_size {
        arguments.extend_from_slice(&[
            "--width".to_string(),
            launch_options.window_width.to_string(),
            "--height".to_string(),
            launch_options.window_height.to_string(),
        ]);
    }

    arguments
}

fn build_jvm_args(
    version_data: &VersionData,
    argument_replacements: &ArgumentReplacements,
    launch_options: &LaunchOptions,
) -> Vec<String> {
    debug!("Building JVM Args");

    let mut arguments = vec![];

    if launch_options.enable_memory {
        arguments.push(format!("-Xms{}M", launch_options.min_memory));
        arguments.push(format!("-Xmx{}M", launch_options.max_memory));
    }

    if launch_options.enable_jvm_args {
        // Custom
        for argument in launch_options.jvm_args.split_whitespace() {
            arguments.push(argument_replacements.replace(argument));
        }
    } else if let Some(args) = &version_data.arguments {
        // Version Data
        for argument in args.jvm_arguments() {
            arguments.push(argument_replacements.replace(&argument));
        }
    }

    // Essentials
    if !arguments
        .iter()
        .any(|arg| arg.starts_with("-Djava.library.path="))
    {
        let arg = "-Djava.library.path=${natives_directory}";
        arguments.push(argument_replacements.replace(arg));
    }
    if !arguments.iter().any(|arg| arg.starts_with("-cp")) {
        let arg = "${classpath}";
        arguments.push("-cp".to_string());
        arguments.push(argument_replacements.replace(arg));
    }

    arguments
}

fn build_classpath(
    version_data: &VersionData,
    minecraft_path: impl AsRef<Path>,
    libraries_path: impl AsRef<Path>,
) -> Result<String, MinecraftError> {
    debug!("Building classpath");

    let mut classes: Vec<String> = vec![];

    for library in version_data.needed_libraries() {
        let (mut package, name, version) = library.extract_information()?;
        package = package.replace('.', "/");

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        library_path.push(&package);
        library_path.push(&name);
        library_path.push(&version);

        if let Some(native) = &library.get_native() {
            let native_jar_name = format!("{}-{}-{}.jar", name, version, native);
            library_path.push(native_jar_name);
        } else {
            let jar_name = format!("{}-{}.jar", name, version);
            library_path.push(jar_name);
        }

        classes.push(library_path.to_string_lossy().to_string());
    }

    let mut minecraft_path = PathBuf::from(minecraft_path.as_ref());
    minecraft_path.push("bin");
    minecraft_path.push(format!("minecraft-{}-client.jar", &version_data.id));

    classes.push(minecraft_path.to_string_lossy().to_string());

    Ok(classes.join(&Platform::current().classpath_seperator().to_string()))
}
