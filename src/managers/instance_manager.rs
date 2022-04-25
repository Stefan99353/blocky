use crate::settings::SettingKey;
use crate::{settings, BlockyApplication};
use gio::prelude::ListModelExt;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{Cast, MainContext, ObjectExt, StaticType};
use glib::{ParamFlags, ParamSpecObject, ToValue};
use glib::{ParamSpec, Value};
use libblocky::gobject::GBlockyInstance;
use libblocky::helpers::HelperError;
use libblocky::Instance;
use once_cell::sync::Lazy;
use std::thread;
use uuid::Uuid;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct BlockyInstanceManager {
        pub instances: ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstanceManager {
        const NAME: &'static str = "BlockyInstanceManager";
        type Type = super::BlockyInstanceManager;
        type ParentType = glib::Object;

        fn new() -> Self {
            let instances = ListStore::new(GBlockyInstance::static_type());

            Self { instances }
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

        // Remove from disk
        thread::spawn(move || {
            let path = settings::get_string(SettingKey::InstancesFilePath);
            if let Err(err) = libblocky::helpers::remove_instance(uuid, path) {
                error!("Error while removing instance - {}", err);
            }
        });
    }
}

impl Default for BlockyInstanceManager {
    fn default() -> Self {
        BlockyApplication::default().instance_manager()
    }
}
