use crate::managers::BlockyProfileManager;
use crate::{helpers, BlockyApplication};
use anyhow::anyhow;
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
use gtk_macros::send;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

pub const INSTANCES: &str = "instances";

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
                    INSTANCES,
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
                INSTANCES => self.instances.to_value(),
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
        self.property(INSTANCES)
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
                let instances = instances
                    .into_iter()
                    .map(GInstance::from)
                    .collect::<Vec<GInstance>>();

                this.instances().splice(0, this.instances().n_items(), &instances);
                this.notify(INSTANCES);

                glib::Continue(false)
            })
        );
    }

    pub fn full_instances(&self) -> glib::Receiver<Vec<Instance>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let instances = helpers::instances::load_instances();
            send!(sender, instances);
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
            let instance = helpers::instances::find_instance(uuid);
            send!(sender, instance);
        });

        receiver
    }

    pub fn add_instance(&self, instance: Instance) {
        // Add to ListStore
        let g_instance = GInstance::from(instance.clone());
        self.instances().append(&g_instance);
        self.notify(INSTANCES);

        // Add to disk
        thread::spawn(move || {
            helpers::instances::save_instance(instance);
        });
    }

    pub fn update_instance(&self, instance: Instance) {
        let uuid = instance.uuid;
        let g_instance = GInstance::from(instance.clone());

        // Update in ListStore
        let instances = self.instances();
        for pos in 0..instances.n_items() {
            let instance = instances
                .item(pos)
                .unwrap()
                .downcast::<GInstance>()
                .unwrap();

            if instance.uuid() == uuid {
                instances.splice(pos, 1, &[g_instance]);
                self.notify(INSTANCES);
                break;
            }
        }

        // Save to disk
        thread::spawn(move || {
            helpers::instances::save_instance(instance);
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

        self.notify(INSTANCES);

        // Remove from disk
        thread::spawn(move || {
            helpers::instances::remove_instance(uuid);
        });
    }

    pub fn install(&self, uuid: Uuid) -> glib::Receiver<InstallationUpdate> {
        let imp = imp::BlockyInstanceManager::from_instance(self);
        imp.cancel_current_installation
            .store(false, Ordering::Relaxed);

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        helpers::instances::install(uuid, sender, imp.cancel_current_installation.clone());

        receiver
    }

    pub fn cancel_current_installation(&self) {
        let imp = imp::BlockyInstanceManager::from_instance(self);
        imp.cancel_current_installation
            .store(true, Ordering::Relaxed);
    }

    pub fn launch(&self, uuid: Uuid) {
        let _handle = thread::spawn(move || {
            let profile = match BlockyProfileManager::default().current_profile() {
                None => {
                    helpers::error::error_dialog(anyhow!("No profile selected"));
                    return;
                }
                Some(profile) => profile.uuid(),
            };

            let options = match helpers::instances::build_launch_options(uuid, profile) {
                Ok(options) => options,
                Err(err) => {
                    helpers::error::error_dialog(err);
                    return;
                }
            };

            helpers::instances::launch(uuid, &options);
        });
    }

    pub fn check_installed(&self, uuid: Uuid) -> glib::Receiver<bool> {
        helpers::instances::check_installed(uuid).0
    }
}

impl Default for BlockyInstanceManager {
    fn default() -> Self {
        BlockyApplication::default().instance_manager()
    }
}
