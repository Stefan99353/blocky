use super::GlobalLaunchOptions;
use crate::instance::install::error::InstallationError;
use crate::instance::launch::argument_replacements::ArgumentReplacements;
use crate::instance::models::VersionData;
use crate::os::Platform;
use crate::Instance;
use itertools::Itertools;

pub fn classpath(instance: &Instance, version_data: &VersionData) -> crate::error::Result<String> {
    let mut classpath = String::new();

    for library in version_data.libraries.iter().filter(|l| l.check_use()) {
        let (mut package, name, version): (String, String, String) = library
            .name
            .split(':')
            .map(|x| x.to_string())
            .collect_tuple()
            .ok_or(InstallationError::LibraryNameFormat)?;
        package = package.replace('.', "/");

        let mut library_path = instance.libraries_path();
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

        classpath.push_str(&library_path.to_string_lossy());
        classpath.push(Platform::current().classpath_seperator());
    }

    let mut client_path = instance.dot_minecraft_path();
    client_path.push("bin");
    client_path.push(format!("minecraft-{}-client.jar", &version_data.id));
    classpath.push_str(&client_path.to_string_lossy());

    Ok(classpath)
}

pub fn game_arguments(
    instance: &Instance,
    options: &GlobalLaunchOptions,
    version_data: &VersionData,
    arg_replacers: &ArgumentReplacements,
) -> Vec<String> {
    let mut arguments = vec![];

    // New arguments
    if let Some(args) = &version_data.arguments {
        for argument in args.game_arguments() {
            arguments.push(arg_replacers.replace(&argument));
        }
    }

    // Old style of arguments
    if let Some(args) = &version_data.minecraft_arguments {
        for argument in args.split_whitespace() {
            arguments.push(arg_replacers.replace(argument));
        }
    }

    // Fullscreen
    if instance.game.use_fullscreen || options.use_fullscreen {
        arguments.push("--fullscreen".to_string());
    } else if instance.game.enable_window_size {
        // Instance window size
        arguments.extend_from_slice(&[
            "--width".to_string(),
            instance.game.window_width.to_string(),
            "--height".to_string(),
            instance.game.window_height.to_string(),
        ]);
    } else if options.enable_window_size {
        // Global window size
        arguments.extend_from_slice(&[
            "--width".to_string(),
            options.window_width.to_string(),
            "--height".to_string(),
            options.window_height.to_string(),
        ]);
    }

    arguments
}

pub fn jvm_arguments(
    instance: &Instance,
    options: &GlobalLaunchOptions,
    version_data: &VersionData,
    arg_replacers: &ArgumentReplacements,
) -> Vec<String> {
    let mut arguments = vec![];

    // Add memory options
    if instance.process.enable_memory {
        arguments.push(format!("-Xms{}M", instance.process.min_memory));
        arguments.push(format!("-Xmx{}M", instance.process.max_memory));
    } else if options.enable_memory {
        arguments.push(format!("-Xms{}M", options.min_memory));
        arguments.push(format!("-Xmx{}M", options.max_memory));
    }

    // Other arguments
    if instance.process.enable_jvm_args {
        // Instance
        for argument in instance.process.jvm_args.split_whitespace() {
            arguments.push(arg_replacers.replace(argument));
        }
    } else if options.enable_jvm_args {
        // Global
        for argument in options.jvm_args.split_whitespace() {
            arguments.push(arg_replacers.replace(argument));
        }
    } else if let Some(args) = &version_data.arguments {
        // Version Data
        for argument in args.jvm_arguments() {
            arguments.push(arg_replacers.replace(&argument));
        }
    }

    // Essentials
    if !arguments
        .iter()
        .any(|arg| arg.starts_with("-Djava.library.path="))
    {
        let arg = "-Djava.library.path=${natives_directory}";
        arguments.push(arg_replacers.replace(arg));
    }
    if !arguments.iter().any(|arg| arg.starts_with("-cp")) {
        let arg = "${classpath}";
        arguments.push("-cp".to_string());
        arguments.push(arg_replacers.replace(arg));
    }

    arguments
}

pub fn java_executable(instance: &Instance, options: &GlobalLaunchOptions) -> String {
    if instance.process.enable_java_exec && !instance.process.java_exec.is_empty() {
        // Use instance specified
        return instance.process.java_exec.clone();
    } else if !options.java_exec.is_empty() {
        // Global
        return options.java_exec.clone();
    }

    String::from("java")
}
