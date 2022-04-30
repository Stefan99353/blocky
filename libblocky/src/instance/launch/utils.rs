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
        warn!("{}", &library.name);
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
    version_data: &VersionData,
    arg_replacers: &ArgumentReplacements,
) -> Vec<String> {
    let mut arguments = vec![];

    if let Some(args) = &version_data.arguments {
        for argument in args.game_arguments() {
            arguments.push(arg_replacers.replace(&argument));
        }
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
    if instance.process.use_custom_memory {
        arguments.push(format!("-Xms{}M", instance.process.jvm_min_memory));
        arguments.push(format!("-Xmx{}M", instance.process.jvm_max_memory));
    } else if options.use_custom_memory {
        arguments.push(format!("-Xms{}M", options.jvm_min_memory));
        arguments.push(format!("-Xmx{}M", options.jvm_max_memory));
    }

    // Other arguments
    if instance.process.use_custom_jvm_arguments {
        for argument in instance.process.jvm_arguments.split_whitespace() {
            arguments.push(arg_replacers.replace(argument));
        }
    } else if let Some(args) = &version_data.arguments {
        for argument in args.jvm_arguments() {
            arguments.push(arg_replacers.replace(&argument));
        }
    }

    // Essentials
    // if arguments
    //     .iter()
    //     .any(|arg| arg.starts_with("-Djava.library.path="))
    // {
    //     let arg = "-Djava.library.path=${natives_directory}";
    //     arguments.push(arg_replacers.replace(arg));
    // }
    // if arguments.iter().any(|arg| arg.starts_with("-cp")) {
    //     let arg = "${classpath}";
    //     arguments.push("-cp".to_string());
    //     arguments.push(arg_replacers.replace(arg));
    // }

    arguments
}

pub fn java_executable(instance: &Instance, options: &GlobalLaunchOptions) -> String {
    if instance.process.use_custom_java_executable && !instance.process.java_executable.is_empty() {
        // Use instance specified
        return instance.process.java_executable.clone();
    } else if !options.java_executable.is_empty() {
        // Global
        return options.java_executable.clone();
    }

    String::from("java")
}
