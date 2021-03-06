use crate::helpers::{build_launch_options, launch_instance};
use crate::managers::BlockyProfileManager;
use crate::settings::SettingKey;
use crate::{helpers, settings, BlockyApplication};
use blocky_core::gobject::GInstance;
use blocky_core::instance::Instance;
use blocky_core::minecraft::installation_update::InstallationUpdate;
use gio::prelude::*;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::{
    Cast, MainContext, ObjectExt, ParamFlags, ParamSpec, ParamSpecObject, StaticType, ToValue,
    Value,
};
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
            let instances = ListStore::new(GInstance::static_type());

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

    pub fn find_instance(&self, uuid: &Uuid) -> Option<GInstance> {
        let instances = self.instances();

        for pos in 0..instances.n_items() {
            let instance = instances
                .item(pos)
                .unwrap()
                .downcast::<GInstance>()
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
                    .map(GInstance::from)
                    .collect::<Vec<GInstance>>();

                this.instances().splice(0, this.instances().n_items(), &instances);
                this.notify("instances");

                glib::Continue(true)
            })
        );
    }

    pub fn full_instances(&self) -> glib::Receiver<Vec<Instance>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);

            match helpers::load_instances(path) {
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

    pub fn find_full_instance(&self, instance: &GInstance) -> glib::Receiver<Option<Instance>> {
        let uuid = instance.uuid();
        self.find_full_instance_by_uuid(uuid)
    }

    pub fn find_full_instance_by_uuid(&self, uuid: Uuid) -> glib::Receiver<Option<Instance>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);

            match helpers::find_instance(uuid, path) {
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

    pub fn add_instance(&self, instance: Instance) {
        // Add to ListStore
        let g_instance = GInstance::from(instance.clone());
        self.instances().append(&g_instance);
        self.notify("instances");

        // Add to disk
        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);
            if let Err(err) = helpers::save_instance(instance, path) {
                error!("Error while saving instance - {}", err);
            }
        });
    }

    pub fn update_instance(&self, instance: Instance) {
        let uuid = instance.uuid;
        let g_instance = GInstance::from(instance.clone());

        let instances = self.instances();
        for pos in 0..instances.n_items() {
            let instance = instances
                .item(pos)
                .unwrap()
                .downcast::<GInstance>()
                .unwrap();

            if instance.uuid() == uuid {
                instances.splice(pos, 1, &[g_instance]);
                self.notify("instances");
                break;
            }
        }

        // Save to disk
        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);
            if let Err(err) = helpers::save_instance(instance, path) {
                error!("Error while saving instance - {}", err);
            }
        });
    }

    pub fn remove_instance(&self, instance: &GInstance) {
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
                .downcast::<GInstance>()
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
            if let Err(err) = helpers::remove_instance(uuid, path) {
                error!("Error while removing instance - {}", err);
            }
        });
    }

    pub fn install_instance(&self, uuid: Uuid) -> glib::Receiver<InstallationUpdate> {
        info!("Installing instance '{}'", &uuid);
        let imp = imp::BlockyInstanceManager::from_instance(self);
        imp.cancel_current_installation
            .store(false, Ordering::Relaxed);

        let (g_sender, g_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let path = settings::get_string(SettingKey::InstancesFilePath);

        let cancel_flag = imp.cancel_current_installation.clone();
        thread::spawn(move || {
            let receiver = helpers::install_threaded(uuid, path.clone(), cancel_flag);

            while let Ok(update) = receiver.recv() {
                g_sender
                    .send(update)
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

        // TODO: Offline launch
        thread::spawn(move || {
            let current_profile = BlockyProfileManager::default().current_profile();
            if current_profile.is_none() {
                error!("No profile selected");
                return;
            }
            let profile_uuid = current_profile.unwrap().uuid();

            let launch_options =
                build_launch_options(uuid, instances_path.clone(), profile_uuid, profiles_path);
            if let Err(err) = launch_options {
                error!("Error while building launch options: {}", err);
                return;
            }

            let launch_result = launch_instance(uuid, instances_path, launch_options.unwrap());

            if let Err(err) = launch_result {
                error!("Error while launching instance: {}", err);
            }
        });
    }

    pub fn check_instance_installed(&self, uuid: Uuid) -> glib::Receiver<bool> {
        let (g_sender, g_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let path = settings::get_string(SettingKey::InstancesFilePath);

        thread::spawn(move || match helpers::check_install_state(uuid, path) {
            Ok(installed) => {
                g_sender
                    .send(installed)
                    .expect("Could not send status through channel");
            }
            Err(err) => {
                error!("Error while checking installed state: {}", err);
                g_sender
                    .send(false)
                    .expect("Could not send status through channel");
            }
        });

        g_receiver
    }
}

impl Default for BlockyInstanceManager {
    fn default() -> Self {
        BlockyApplication::default().instance_manager()
    }
}
