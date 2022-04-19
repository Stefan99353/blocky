use crate::application::RUNTIME;
use crate::settings::SettingKey;
use crate::{settings, BlockyApplication};
use gio::traits::ListModelExt;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::{Cast, ObjectExt, ParamFlags, ParamSpec, ParamSpecObject, StaticType, ToValue, Value};
use libblocky::gobject::GBlockyProfile;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use uuid::Uuid;

mod imp {
    use super::*;

    pub struct BlockyProfileManager {
        pub profiles: ListStore,
        pub current_profile: RefCell<GBlockyProfile>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyProfileManager {
        const NAME: &'static str = "BlockyProfileManager";
        type Type = super::BlockyProfileManager;
        type ParentType = glib::Object;

        fn new() -> Self {
            let profiles = ListStore::new(GBlockyProfile::static_type());

            Self {
                profiles,
                current_profile: RefCell::default(),
            }
        }
    }

    impl ObjectImpl for BlockyProfileManager {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecObject::new(
                        "profiles",
                        "Profiles",
                        "Profiles",
                        ListStore::static_type(),
                        ParamFlags::READABLE,
                    ),
                    ParamSpecObject::new(
                        "current-profile",
                        "Current Profile",
                        "Current Profile",
                        GBlockyProfile::static_type(),
                        ParamFlags::READABLE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "profiles" => self.profiles.to_value(),
                "current-profile" => self.profiles.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct BlockyProfileManager(ObjectSubclass<imp::BlockyProfileManager>);
}

impl BlockyProfileManager {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn profiles(&self) -> ListStore {
        self.property("profiles")
    }

    pub fn current_profile(&self) -> GBlockyProfile {
        self.property("current-profile")
    }

    pub fn profile_by_uuid(&self, uuid: &Uuid) -> Option<GBlockyProfile> {
        let profiles = self.profiles();

        for pos in 0..profiles.n_items() {
            let profile = profiles
                .item(pos)
                .unwrap()
                .downcast::<GBlockyProfile>()
                .unwrap();

            if &profile.uuid() == uuid {
                return Some(profile);
            }
        }

        None
    }

    pub async fn full_profile_by_uuid(&self, _uuid: Uuid) -> Option<libblocky::BlockyProfile> {
        todo!("Read from fs;");
    }

    pub async fn full_current_profile(&self) -> Option<libblocky::BlockyProfile> {
        todo!("Read from fs; path is in settings");
    }

    pub async fn full_profiles(&self) -> Vec<libblocky::BlockyProfile> {
        todo!("Read from fs; path is in settings");
    }

    pub async fn add_profile(&self, profile: libblocky::BlockyProfile) {
        RUNTIME
            .spawn(async move {
                let uuid = profile.uuid.clone();
                let username = profile.minecraft_profile.as_ref().unwrap().name.clone();

                if let Err(err) = libblocky::helpers::save_profile(
                    profile,
                    settings::get_string(SettingKey::InstancesFilePath),
                )
                .await
                {
                    error!("Error while saving profile - {}", err);
                }

                let pm = BlockyProfileManager::default();
                pm.profiles().append(&GBlockyProfile::new(&uuid, &username));
            })
            .await
            .unwrap();
    }
}

impl Default for BlockyProfileManager {
    fn default() -> Self {
        BlockyApplication::default().profile_manager()
    }
}
