use crate::instance::{GamePropertiesBuilder, InstanceBuilder, ProcessPropertiesBuilder};
use crate::Instance;
use glib::subclass::prelude::*;
use glib::{
    ObjectExt, ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecString, ParamSpecUInt, ToValue,
    Value,
};
use once_cell::sync::{Lazy, OnceCell};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GBlockyInstance {
        pub uuid: OnceCell<String>,
        pub name: RefCell<String>,
        pub description: RefCell<String>,
        pub version: RefCell<String>,
        pub instance_path: RefCell<String>,
        pub libraries_path: RefCell<String>,
        pub assets_path: RefCell<String>,
        pub custom_width: Cell<u32>,
        pub custom_height: Cell<u32>,
        pub use_custom_resolution: Cell<bool>,
        pub use_fullscreen: Cell<bool>,
        pub use_custom_java_executable: Cell<bool>,
        pub java_executable: RefCell<String>,
        pub use_custom_jvm_arguments: Cell<bool>,
        pub jvm_arguments: RefCell<String>,
        pub use_custom_memory: Cell<bool>,
        pub jvm_min_memory: Cell<u32>,
        pub jvm_max_memory: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GBlockyInstance {
        const NAME: &'static str = "GBlockyInstance";
        type Type = super::GBlockyInstance;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GBlockyInstance {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("uuid", "UUID", "UUID", None, ParamFlags::READWRITE),
                    ParamSpecString::new("name", "Name", "Name", None, ParamFlags::READWRITE),
                    ParamSpecString::new(
                        "description",
                        "Description",
                        "Description",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "version",
                        "Version",
                        "Version",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "instance-path",
                        "Instance Path",
                        "Instance Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "libraries-path",
                        "Libraries Path",
                        "Libraries Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "assets-path",
                        "Assets Path",
                        "Assets Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "custom-width",
                        "Custom Width",
                        "Custom Width",
                        0,
                        u32::MAX,
                        1280,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "custom-height",
                        "Custom Height",
                        "Custom Height",
                        0,
                        u32::MAX,
                        720,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "use-custom-resolution",
                        "Use Custom Resolution",
                        "Use Custom Resolution",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "use-fullscreen",
                        "Use Fullscreen",
                        "Use Fullscreen",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "use-custom-java-executable",
                        "Use Custom Java Executable",
                        "Use Custom Java Executable",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "java-executable",
                        "Java Executable",
                        "Java Executable",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "use-custom-jvm-arguments",
                        "Use Custom JVM Arguments",
                        "Use Custom JVM Arguments",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "jvm-arguments",
                        "JVM Arguments",
                        "JVM Arguments",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "use-custom-memory",
                        "Use Custom Memory",
                        "Use Custom Memory",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "jvm-min-memory",
                        "JVM Min Memory",
                        "JVM Min Memory",
                        0,
                        u32::MAX,
                        512,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "jvm-max-memory",
                        "JVM Max Memory",
                        "JVM Max Memory",
                        0,
                        u32::MAX,
                        1024,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "uuid" => self.uuid.set(value.get().unwrap()).unwrap(),
                "name" => *self.name.borrow_mut() = value.get().unwrap(),
                "description" => *self.description.borrow_mut() = value.get().unwrap(),
                "version" => *self.version.borrow_mut() = value.get().unwrap(),
                "instance-path" => *self.instance_path.borrow_mut() = value.get().unwrap(),
                "libraries-path" => *self.libraries_path.borrow_mut() = value.get().unwrap(),
                "assets-path" => *self.assets_path.borrow_mut() = value.get().unwrap(),
                "custom-width" => self.custom_width.set(value.get().unwrap()),
                "custom-height" => self.custom_height.set(value.get().unwrap()),
                "use-custom-resolution" => self.use_custom_resolution.set(value.get().unwrap()),
                "use-fullscreen" => self.use_fullscreen.set(value.get().unwrap()),
                "use-custom-java-executable" => {
                    self.use_custom_java_executable.set(value.get().unwrap())
                }
                "java-executable" => *self.java_executable.borrow_mut() = value.get().unwrap(),
                "use-custom-jvm-arguments" => {
                    self.use_custom_jvm_arguments.set(value.get().unwrap())
                }
                "jvm-arguments" => *self.jvm_arguments.borrow_mut() = value.get().unwrap(),
                "use-custom-memory" => self.use_custom_memory.set(value.get().unwrap()),
                "jvm-min-memory" => self.jvm_min_memory.set(value.get().unwrap()),
                "jvm-max-memory" => self.jvm_max_memory.set(value.get().unwrap()),
                x => {
                    error!("Property {} not a member of GBlockyInstance", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "uuid" => self.uuid.get().to_value(),
                "name" => self.name.borrow().to_value(),
                "description" => self.description.borrow().to_value(),
                "version" => self.version.borrow().to_value(),
                "instance-path" => self.instance_path.borrow().to_value(),
                "libraries-path" => self.libraries_path.borrow().to_value(),
                "assets-path" => self.assets_path.borrow().to_value(),
                "custom-width" => self.custom_width.get().to_value(),
                "custom-height" => self.custom_height.get().to_value(),
                "use-custom-resolution" => self.use_custom_resolution.get().to_value(),
                "use-fullscreen" => self.use_fullscreen.get().to_value(),
                "use-custom-java-executable" => self.use_custom_java_executable.get().to_value(),
                "java-executable" => self.java_executable.borrow().to_value(),
                "use-custom-jvm-arguments" => self.use_custom_jvm_arguments.get().to_value(),
                "jvm-arguments" => self.jvm_arguments.borrow().to_value(),
                "use-custom-memory" => self.use_custom_memory.get().to_value(),
                "jvm-min-memory" => self.jvm_min_memory.get().to_value(),
                "jvm-max-memory" => self.jvm_max_memory.get().to_value(),
                x => {
                    error!("Property {} not a member of GBlockyInstance", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GBlockyInstance(ObjectSubclass<imp::GBlockyInstance>);
}

impl GBlockyInstance {
    pub fn new(instances_path: &str, libraries_path: &str, assets_path: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();

        let mut instance_path = PathBuf::from(instances_path);
        instance_path.push(&uuid);

        glib::Object::new(&[
            ("uuid", &uuid.to_string()),
            (
                "instance-path",
                &instance_path.to_string_lossy().to_string(),
            ),
            ("libraries-path", &libraries_path),
            ("assets-path", &assets_path),
        ])
        .unwrap()
    }

    pub fn uuid(&self) -> Uuid {
        let uuid: String = self.property("uuid");
        Uuid::from_str(&uuid).unwrap()
    }

    pub fn name(&self) -> String {
        self.property("name")
    }
}

impl From<Instance> for GBlockyInstance {
    fn from(instance: Instance) -> Self {
        glib::Object::new(&[
            ("uuid", &instance.uuid.to_string()),
            ("name", &instance.name),
            ("description", &instance.description.unwrap_or_default()),
            ("version", &instance.version),
            ("instance-path", &instance.instance_path),
            ("libraries-path", &instance.game.libraries_path),
            ("assets-path", &instance.game.assets_path),
            ("custom-width", &instance.game.custom_width),
            ("custom-height", &instance.game.custom_height),
            (
                "use-custom-resolution",
                &instance.game.use_custom_resolution,
            ),
            ("use-fullscreen", &instance.game.use_fullscreen),
            (
                "use-custom-java-executable",
                &instance.process.use_custom_java_executable,
            ),
            ("java-executable", &instance.process.java_executable),
            (
                "use-custom-jvm-arguments",
                &instance.process.use_custom_jvm_arguments,
            ),
            ("jvm-arguments", &instance.process.jvm_arguments),
            ("use-custom-memory", &instance.process.use_custom_memory),
            ("jvm-min-memory", &instance.process.jvm_min_memory),
            ("jvm-max-memory", &instance.process.jvm_max_memory),
        ])
        .unwrap()
    }
}

impl From<GBlockyInstance> for Instance {
    fn from(instance: GBlockyInstance) -> Self {
        let mut instance_builder = InstanceBuilder::default();
        let mut game_builder = GamePropertiesBuilder::default();
        let mut process_builder = ProcessPropertiesBuilder::default();

        // Game Properties
        let libraries_path = instance
            .property::<String>("libraries-path")
            .trim()
            .to_string();
        let assets_path = instance
            .property::<String>("assets-path")
            .trim()
            .to_string();

        if !libraries_path.is_empty() {
            game_builder.libraries_path(libraries_path);
        }
        if !assets_path.is_empty() {
            game_builder.assets_path(assets_path);
        }

        game_builder
            .custom_width(instance.property("custom-width"))
            .custom_height(instance.property("custom-height"))
            .use_custom_resolution(instance.property("use-custom-resolution"))
            .use_fullscreen(instance.property("use-fullscreen"));

        // Process Properties
        process_builder
            .use_custom_java_executable(instance.property("use-custom-java-executable"))
            .java_executable(instance.property("java-executable"))
            .use_custom_jvm_arguments(instance.property("use-custom-jvm-arguments"))
            .jvm_arguments(instance.property("jvm-arguments"))
            .use_custom_memory(instance.property("use-custom-memory"))
            .jvm_min_memory(instance.property("jvm-min-memory"))
            .jvm_max_memory(instance.property("jvm-max-memory"));

        // Instance
        let uuid = instance.property::<String>("uuid");
        let uuid = match Uuid::from_str(&uuid) {
            Ok(uuid) => uuid,
            Err(_err) => {
                warn!("Instance UUID is not valid => Generating new one");
                Uuid::new_v4()
            }
        };

        instance_builder
            .uuid(uuid)
            .name(instance.property("name"))
            .version(instance.property("version"))
            .instance_path(instance.property("instance-path"))
            .game(game_builder.build().unwrap())
            .process(process_builder.build().unwrap());

        let description = instance
            .property::<String>("description")
            .trim()
            .to_string();
        if !description.is_empty() {
            instance_builder.description(description);
        }

        instance_builder.build().unwrap()
    }
}
