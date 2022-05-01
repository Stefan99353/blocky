use crate::managers::BlockyProfileManager;
use crate::settings::SettingKey;
use crate::{settings, BlockyApplication};
use anyhow::anyhow;
use gio::prelude::ListModelExt;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{Cast, MainContext, ObjectExt, StaticType};
use glib::{ParamFlags, ParamSpecObject, ToValue};
use glib::{ParamSpec, Value};
use libblocky::error::Error;
use libblocky::gobject::{GBlockyInstance, GBlockyProfile};
use libblocky::helpers::HelperError;
use libblocky::instance::launch_options::{GlobalLaunchOptions, GlobalLaunchOptionsBuilder};
use libblocky::instance::resource_update::ResourceInstallationUpdate;
use libblocky::Instance;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct BlockyInstanceManager {
        pub instances: ListStore,

        pub cancel_current_installation: Arc<AtomicBool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstanceManager {
        const NAME: &'static str = "BlockyInstanceManager";
        type Type = super::BlockyInstanceManager;
        type ParentType = glib::Object;

        fn new() -> Self {
            let instances = ListStore::new(GBlockyInstance::static_type());

            Self {
                instances,
                cancel_current_installation: Arc::new(AtomicBool::default()),
            }
        }
    }

    impl ObjectImpl for BlockyInstanceManager {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::new(
                    "instances",
                    "Instances",
                    "Instances",
                    ListStore::static_type(),
                    ParamFlags::READABLE,
                )]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "instances" => self.instances.to_value(),
                x => {
                    error!("Property {} not a member of BlockyInstanceManager", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct BlockyInstanceManager(ObjectSubclass<imp::BlockyInstanceManager>);
}

impl BlockyInstanceManager {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn instances(&self) -> ListStore {
        self.property("instances")
    }

    pub fn find_instance(&self, uuid: &Uuid) -> Option<GBlockyInstance> {
        let instances = self.instances();

        for pos in 0..instances.n_items() {
            let instance = instances
                .item(pos)
                .unwrap()
                .downcast::<GBlockyInstance>()
                .unwrap();

            if &instance.uuid() == uuid {
                return Some(instance);
            }
        }

        None
    }

    pub fn initialize(&self) {
        self.full_instances().attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false), move |instances| {
                // Add instances
                let instances = instances.into_iter()
                    .map(GBlockyInstance::from)
                    .collect::<Vec<GBlockyInstance>>();

                this.instances().splice(0, this.instances().n_items(), &instances);
                this.notify("instances");

                glib::Continue(true)
            })
        );
    }

    pub fn full_instances(&self) -> glib::Receiver<Vec<libblocky::Instance>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);

            match libblocky::helpers::load_instances(path) {
                Ok(instances) => {
                    sender
                        .send(instances)
                        .expect("Could not send instances through channel");
                }
                Err(err) => {
                    error!("Error while loading instances - {}", err);
                    sender
                        .send(vec![])
                        .expect("Could not send instances through channel");
                }
            }
        });

        receiver
    }

    pub fn find_full_instance(
        &self,
        instance: &GBlockyInstance,
    ) -> glib::Receiver<Option<libblocky::Instance>> {
        let uuid = instance.uuid();
        self.find_full_instance_by_uuid(uuid)
    }

    pub fn find_full_instance_by_uuid(
        &self,
        uuid: Uuid,
    ) -> glib::Receiver<Option<libblocky::Instance>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);

            match libblocky::helpers::find_instance(uuid, path) {
                Ok(instance) => {
                    sender
                        .send(instance)
                        .expect("Could not send instance through channel");
                }
                Err(err) => {
                    error!("Error while loading instance - {}", err);
                    sender
                        .send(None)
                        .expect("Could not send instance through channel");
                }
            }
        });

        receiver
    }

    pub fn add_instance(&self, instance: libblocky::Instance) {
        // Add to ListStore
        let g_instance = GBlockyInstance::from(instance.clone());
        self.instances().append(&g_instance);
        self.notify("instances");

        // Add to disk
        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);
            if let Err(err) = libblocky::helpers::save_instance(instance, path) {
                error!("Error while saving instance - {}", err);
            }
        });
    }

    pub fn remove_instance(&self, instance: &GBlockyInstance) {
        let uuid = instance.uuid();
        self.remove_instance_by_uuid(uuid);
    }

    pub fn remove_instance_by_uuid(&self, uuid: Uuid) {
        // Remove from ListStore
        let instances = self.instances();
        for pos in 0..instances.n_items() {
            let instance = instances
                .item(pos)
                .unwrap()
                .downcast::<GBlockyInstance>()
                .unwrap();

            if instance.uuid() == uuid {
                instances.remove(pos);
                break;
            }
        }

        self.notify("instances");

        // Remove from disk
        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);
            if let Err(err) = libblocky::helpers::remove_instance(uuid, path) {
                error!("Error while removing instance - {}", err);
            }
        });
    }

    pub fn install_instance(
        &self,
        uuid: Uuid,
    ) -> glib::Receiver<libblocky::error::Result<Option<ResourceInstallationUpdate>>> {
        info!("Installing instance '{}'", &uuid);
        let imp = imp::BlockyInstanceManager::from_instance(self);
        imp.cancel_current_installation
            .store(false, Ordering::Relaxed);

        let (g_sender, g_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let path = settings::get_string(SettingKey::InstancesFilePath);

        let cancel_flag = imp.cancel_current_installation.clone();
        thread::spawn(move || {
            let receiver = libblocky::helpers::install_threaded(uuid, path.clone(), cancel_flag);

            while let Ok(update) = receiver.recv() {
                debug!("Received update: {:?}", &update);

                g_sender
                    .send(update)
                    .expect("Could not send update through channel");
                g_sender
                    .send(Ok(None))
                    .expect("Could not send update through channel");
            }
        });

        g_receiver
    }

    pub fn cancel_current_installation(&self) {
        let imp = imp::BlockyInstanceManager::from_instance(self);
        imp.cancel_current_installation
            .store(true, Ordering::Relaxed);
    }

    pub fn launch_instance(&self, uuid: Uuid) {
        info!("Launching instance '{}'", &uuid);
        let instances_path = settings::get_string(SettingKey::InstancesFilePath);
        let profiles_path = settings::get_string(SettingKey::ProfilesFilePath);

        thread::spawn(move || {
            let current_profile = BlockyProfileManager::default().current_profile();

            if current_profile.is_none() {
                error!("No profile selected");
                // TODO: Offline launch
                return;
            }

            let profile_uuid = current_profile.unwrap().uuid();

            libblocky::helpers::launch_instance(
                uuid,
                instances_path,
                profile_uuid,
                profiles_path,
                launch_options(),
            );
        });
    }

    pub fn check_instance_installed(&self, uuid: Uuid) -> glib::Receiver<bool> {
        let (g_sender, g_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let path = settings::get_string(SettingKey::InstancesFilePath);

        thread::spawn(
            move || match libblocky::helpers::check_install_state(uuid, path) {
                Ok(installed) => {
                    g_sender
                        .send(installed)
                        .expect("Could not send status through channel");
                }
                Err(err) => match err {
                    Error::InstanceNotFound(uuid) => {
                        debug!("Instance not yet installed: {}", uuid);
                        g_sender
                            .send(false)
                            .expect("Could not send status through channel");
                    }
                    err => {
                        error!("Error while checking installed state: {}", err);
                        g_sender
                            .send(false)
                            .expect("Could not send status through channel");
                    }
                },
            },
        );

        g_receiver
    }
}

impl Default for BlockyInstanceManager {
    fn default() -> Self {
        BlockyApplication::default().instance_manager()
    }
}

fn launch_options() -> GlobalLaunchOptions {
    let mut builder = GlobalLaunchOptionsBuilder::default();

    builder
        .launcher_name("Blocky".to_string())
        .launcher_version(env!("CARGO_PKG_VERSION").to_string())
        .use_fullscreen(settings::get_bool(SettingKey::DefaultFullscreen))
        // TODO: .use_custom_resolution()
        .custom_width(settings::get_integer(SettingKey::DefaultWidth) as u32)
        .custom_height(settings::get_integer(SettingKey::DefaultHeight) as u32)
        .java_executable(settings::get_string(SettingKey::DefaultJavaExec))
        // TODO: .use_custom_memory()
        .jvm_min_memory(settings::get_integer(SettingKey::DefaultMinMemory) as u32)
        .jvm_max_memory(settings::get_integer(SettingKey::DefaultMaxMemory) as u32)
        .use_custom_jvm_arguments(settings::get_bool(SettingKey::DefaultUseJvmArgs))
        .jvm_arguments(settings::get_string(SettingKey::DefaultJvmArgs));

    builder.build().unwrap()
}
