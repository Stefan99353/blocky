use crate::instance::{Instance, InstanceBuilder};
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

pub const UUID: &str = "uuid";
pub const NAME: &str = "name";
pub const DESCRIPTION: &str = "description";
pub const VERSION: &str = "version";
pub const INSTANCE_PATH: &str = "instance-path";
pub const LIBRARIES_PATH: &str = "libraries-path";
pub const ASSETS_PATH: &str = "assets-path";
pub const USE_FULLSCREEN: &str = "use-fullscreen";
pub const ENABLE_WINDOW_SIZE: &str = "enable-window-size";
pub const WINDOW_WIDTH: &str = "window-width";
pub const WINDOW_HEIGHT: &str = "window-height";
pub const ENABLE_MEMORY: &str = "enable-memory";
pub const MIN_MEMORY: &str = "min-memory";
pub const MAX_MEMORY: &str = "max-memory";
pub const ENABLE_JAVA_EXEC: &str = "enable-java-exec";
pub const JAVA_EXEC: &str = "java-exec";
pub const ENABLE_JVM_ARGS: &str = "enable-jvm-args";
pub const JVM_ARGS: &str = "jvm-args";
pub const USE_FABRIC: &str = "use-fabric";
pub const FABRIC_VERSION: &str = "fabric-version";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GInstance {
        pub uuid: OnceCell<String>,
        pub name: RefCell<String>,
        pub description: RefCell<String>,
        pub version: RefCell<String>,
        pub instance_path: RefCell<String>,
        pub libraries_path: RefCell<String>,
        pub assets_path: RefCell<String>,
        pub use_fullscreen: Cell<bool>,
        pub enable_window_size: Cell<bool>,
        pub window_width: Cell<u32>,
        pub window_height: Cell<u32>,
        pub enable_memory: Cell<bool>,
        pub min_memory: Cell<u32>,
        pub max_memory: Cell<u32>,
        pub enable_java_exec: Cell<bool>,
        pub java_exec: RefCell<String>,
        pub enable_jvm_args: Cell<bool>,
        pub jvm_args: RefCell<String>,

        pub use_fabric: Cell<bool>,
        pub fabric_version: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GInstance {
        const NAME: &'static str = "GInstance";
        type Type = super::GInstance;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GInstance {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(UUID, "UUID", "UUID", None, ParamFlags::READWRITE),
                    ParamSpecString::new(NAME, "Name", "Name", None, ParamFlags::READWRITE),
                    ParamSpecString::new(
                        DESCRIPTION,
                        "Description",
                        "Description",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        VERSION,
                        "Version",
                        "Version",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        INSTANCE_PATH,
                        "Instance Path",
                        "Instance Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        LIBRARIES_PATH,
                        "Libraries Path",
                        "Libraries Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        ASSETS_PATH,
                        "Assets Path",
                        "Assets Path",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        USE_FULLSCREEN,
                        "Use Fullscreen",
                        "Use Fullscreen",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        ENABLE_WINDOW_SIZE,
                        "Use Custom Resolution",
                        "Use Custom Resolution",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        WINDOW_WIDTH,
                        "Custom Width",
                        "Custom Width",
                        0,
                        u32::MAX,
                        1280,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        WINDOW_HEIGHT,
                        "Custom Height",
                        "Custom Height",
                        0,
                        u32::MAX,
                        720,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        ENABLE_MEMORY,
                        "Use Custom Memory",
                        "Use Custom Memory",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        MIN_MEMORY,
                        "JVM Min Memory",
                        "JVM Min Memory",
                        0,
                        u32::MAX,
                        512,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        MAX_MEMORY,
                        "JVM Max Memory",
                        "JVM Max Memory",
                        0,
                        u32::MAX,
                        1024,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        ENABLE_JAVA_EXEC,
                        "Use Custom Java Executable",
                        "Use Custom Java Executable",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        JAVA_EXEC,
                        "Java Executable",
                        "Java Executable",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        ENABLE_JVM_ARGS,
                        "Use Custom JVM Arguments",
                        "Use Custom JVM Arguments",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        JVM_ARGS,
                        "JVM Arguments",
                        "JVM Arguments",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        USE_FABRIC,
                        "Use Fabric",
                        "Use Fabric",
                        false,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        FABRIC_VERSION,
                        "Fabric Version",
                        "Fabric Version",
                        None,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                UUID => self.uuid.set(value.get().unwrap()).unwrap(),
                NAME => *self.name.borrow_mut() = value.get().unwrap(),
                DESCRIPTION => *self.description.borrow_mut() = value.get().unwrap(),
                VERSION => *self.version.borrow_mut() = value.get().unwrap(),
                INSTANCE_PATH => *self.instance_path.borrow_mut() = value.get().unwrap(),
                LIBRARIES_PATH => *self.libraries_path.borrow_mut() = value.get().unwrap(),
                ASSETS_PATH => *self.assets_path.borrow_mut() = value.get().unwrap(),
                USE_FULLSCREEN => self.use_fullscreen.set(value.get().unwrap()),
                ENABLE_WINDOW_SIZE => self.enable_window_size.set(value.get().unwrap()),
                WINDOW_WIDTH => self.window_width.set(value.get().unwrap()),
                WINDOW_HEIGHT => self.window_height.set(value.get().unwrap()),
                ENABLE_MEMORY => self.enable_memory.set(value.get().unwrap()),
                MIN_MEMORY => self.min_memory.set(value.get().unwrap()),
                MAX_MEMORY => self.max_memory.set(value.get().unwrap()),
                ENABLE_JAVA_EXEC => self.enable_java_exec.set(value.get().unwrap()),
                JAVA_EXEC => *self.java_exec.borrow_mut() = value.get().unwrap(),
                ENABLE_JVM_ARGS => self.enable_jvm_args.set(value.get().unwrap()),
                JVM_ARGS => *self.jvm_args.borrow_mut() = value.get().unwrap(),
                USE_FABRIC => self.use_fabric.set(value.get().unwrap()),
                FABRIC_VERSION => *self.fabric_version.borrow_mut() = value.get().unwrap(),
                x => {
                    error!("Property {} not a member of GInstance", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                UUID => self.uuid.get().to_value(),
                NAME => self.name.borrow().to_value(),
                DESCRIPTION => self.description.borrow().to_value(),
                VERSION => self.version.borrow().to_value(),
                INSTANCE_PATH => self.instance_path.borrow().to_value(),
                LIBRARIES_PATH => self.libraries_path.borrow().to_value(),
                ASSETS_PATH => self.assets_path.borrow().to_value(),
                USE_FULLSCREEN => self.use_fullscreen.get().to_value(),
                ENABLE_WINDOW_SIZE => self.enable_window_size.get().to_value(),
                WINDOW_WIDTH => self.window_width.get().to_value(),
                WINDOW_HEIGHT => self.window_height.get().to_value(),
                ENABLE_MEMORY => self.enable_memory.get().to_value(),
                MIN_MEMORY => self.min_memory.get().to_value(),
                MAX_MEMORY => self.max_memory.get().to_value(),
                ENABLE_JAVA_EXEC => self.enable_java_exec.get().to_value(),
                JAVA_EXEC => self.java_exec.borrow().to_value(),
                ENABLE_JVM_ARGS => self.enable_jvm_args.get().to_value(),
                JVM_ARGS => self.jvm_args.borrow().to_value(),
                USE_FABRIC => self.use_fabric.get().to_value(),
                FABRIC_VERSION => self.fabric_version.borrow().to_value(),
                x => {
                    error!("Property {} not a member of GInstance", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GInstance(ObjectSubclass<imp::GInstance>);
}

impl GInstance {
    pub fn new(instances_path: &str, libraries_path: &str, assets_path: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();

        let mut instance_path = PathBuf::from(instances_path);
        instance_path.push(&uuid);

        glib::Object::new(&[
            (UUID, &uuid.to_string()),
            (INSTANCE_PATH, &instance_path.to_string_lossy().to_string()),
            (LIBRARIES_PATH, &libraries_path),
            (ASSETS_PATH, &assets_path),
        ])
        .unwrap()
    }

    pub fn uuid(&self) -> Uuid {
        let uuid: String = self.property(UUID);
        Uuid::from_str(&uuid).unwrap()
    }

    pub fn name(&self) -> String {
        self.property(NAME)
    }
}

impl From<Instance> for GInstance {
    fn from(instance: Instance) -> Self {
        glib::Object::new(&[
            (UUID, &instance.uuid.to_string()),
            (NAME, &instance.name),
            (DESCRIPTION, &instance.description.unwrap_or_default()),
            (VERSION, &instance.version),
            (INSTANCE_PATH, &instance.instance_path),
            (LIBRARIES_PATH, &instance.libraries_path),
            (ASSETS_PATH, &instance.assets_path),
            (USE_FULLSCREEN, &instance.use_fullscreen),
            (ENABLE_WINDOW_SIZE, &instance.enable_window_size),
            (WINDOW_WIDTH, &instance.window_width),
            (WINDOW_HEIGHT, &instance.window_height),
            (ENABLE_MEMORY, &instance.enable_memory),
            (MIN_MEMORY, &instance.min_memory),
            (MAX_MEMORY, &instance.max_memory),
            (ENABLE_JAVA_EXEC, &instance.enable_java_exec),
            (JAVA_EXEC, &instance.java_exec),
            (ENABLE_JVM_ARGS, &instance.enable_jvm_args),
            (JVM_ARGS, &instance.jvm_args),
            (USE_FABRIC, &instance.use_fabric),
            (FABRIC_VERSION, &instance.fabric_version),
        ])
        .unwrap()
    }
}

impl From<GInstance> for Instance {
    fn from(instance: GInstance) -> Self {
        let mut instance_builder = InstanceBuilder::default();

        let uuid = match Uuid::from_str(&instance.property::<String>(UUID)) {
            Ok(uuid) => uuid,
            Err(_err) => {
                warn!("Instance UUID is not valid => Generating new one");
                Uuid::new_v4()
            }
        };
        let description = instance.property::<String>(DESCRIPTION).trim().to_string();
        let libraries_path = instance
            .property::<String>(LIBRARIES_PATH)
            .trim()
            .to_string();
        let assets_path = instance.property::<String>(ASSETS_PATH).trim().to_string();

        instance_builder
            .uuid(uuid)
            .name(instance.property(NAME))
            .version(instance.property(VERSION))
            .instance_path(instance.property(INSTANCE_PATH))
            .use_fullscreen(instance.property(USE_FULLSCREEN))
            .enable_window_size(instance.property(ENABLE_WINDOW_SIZE))
            .window_width(instance.property(WINDOW_WIDTH))
            .window_height(instance.property(WINDOW_HEIGHT))
            .enable_memory(instance.property(ENABLE_MEMORY))
            .min_memory(instance.property(MIN_MEMORY))
            .max_memory(instance.property(MAX_MEMORY))
            .enable_java_exec(instance.property(ENABLE_JAVA_EXEC))
            .java_exec(instance.property(JAVA_EXEC))
            .enable_jvm_args(instance.property(ENABLE_JVM_ARGS))
            .jvm_args(instance.property(JVM_ARGS))
            .use_fabric(instance.property(USE_FABRIC))
            .fabric_version(instance.property(FABRIC_VERSION));

        if !description.is_empty() {
            instance_builder.description(description);
        }

        if !libraries_path.is_empty() {
            instance_builder.libraries_path(libraries_path);
        }
        if !assets_path.is_empty() {
            instance_builder.assets_path(assets_path);
        }

        instance_builder.build().unwrap()
    }
}
