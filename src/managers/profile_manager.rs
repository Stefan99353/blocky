use crate::application::RUNTIME;
use crate::settings::SettingKey;
use crate::{settings, BlockyApplication};
use gio::traits::{ListModelExt, SettingsExt};
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::{Cast, ObjectExt, ParamFlags, ParamSpec, ParamSpecObject, StaticType, ToValue, Value};
use libblocky::gobject::GBlockyProfile;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::str::FromStr;
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
                "current-profile" => self.current_profile.borrow().to_value(),
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

    pub fn current_profile(&self) -> Option<GBlockyProfile> {
        let profile = self.property::<GBlockyProfile>("current-profile");
        let uuid = profile.uuid().to_string();
        let no_profile_uuid = libblocky::gobject::NO_PROFILE_UUID.to_string();

        if uuid == no_profile_uuid {
            return None;
        }

        Some(profile)
    }

    pub fn set_current_profile(&self, profile: &GBlockyProfile) {
        let imp = imp::BlockyProfileManager::from_instance(self);

        let uuid = profile.uuid().to_string();
        settings::set_string(SettingKey::DefaultProfile, &uuid);

        *imp.current_profile.borrow_mut() = profile.clone();
        self.notify("current-profile");
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

    pub async fn init(&self) {
        // Load profiles
        self.full_profiles()
            .await
            .into_iter()
            .map(|p| {
                let uuid = p.uuid;
                let username = p.minecraft_profile.as_ref().unwrap().name.clone();

                GBlockyProfile::new(&uuid, &username)
            })
            .for_each(|p| {
                self.profiles().remove_all();
                self.profiles().append(&p);
            });

        // Set default profile
        let default = settings::get_string(SettingKey::DefaultProfile);
        if let Ok(uuid) = Uuid::from_str(&default) {
            let profile = self.profile_by_uuid(&uuid);
            if let Some(profile) = profile {
                self.set_current_profile(&profile);
            } else {
                settings::get_settings().reset(SettingKey::DefaultProfile.to_key())
            }
        }
    }

    pub async fn full_profile_by_uuid(&self, uuid: Uuid) -> Option<libblocky::BlockyProfile> {
        RUNTIME
            .spawn(async move {
                let profiles_path = settings::get_string(SettingKey::ProfilesFilePath);

                match libblocky::helpers::find_profile(uuid, &profiles_path).await {
                    Ok(profile) => profile,
                    Err(err) => {
                        error!(
                            "Error while searching for profile '{}' on disk - {}",
                            uuid, err
                        );
                        None
                    }
                }
            })
            .await
            .unwrap()
    }

    pub async fn full_current_profile(&self) -> Option<libblocky::BlockyProfile> {
        let uuid = self.property::<GBlockyProfile>("current-profile").uuid();
        self.full_profile_by_uuid(uuid).await
    }

    pub async fn full_profiles(&self) -> Vec<libblocky::BlockyProfile> {
        RUNTIME
            .spawn(async move {
                let profiles_path = settings::get_string(SettingKey::ProfilesFilePath);

                match libblocky::helpers::load_profiles(profiles_path).await {
                    Ok(profiles) => profiles,
                    Err(err) => {
                        error!("Error while loading profiles - {}", err);
                        vec![]
                    }
                }
            })
            .await
            .unwrap()
    }

    pub async fn add_profile(&self, profile: libblocky::BlockyProfile) {
        RUNTIME
            .spawn(async move {
                let uuid = profile.uuid;
                let username = profile.minecraft_profile.as_ref().unwrap().name.clone();
                let profiles_path = settings::get_string(SettingKey::ProfilesFilePath);

                if let Err(err) = libblocky::helpers::save_profile(profile, profiles_path).await {
                    error!("Error while saving profile - {}", err);
                }

                let pm = BlockyProfileManager::default();
                pm.profiles().append(&GBlockyProfile::new(&uuid, &username));

                glib::MainContext::default().spawn(async move {
                    let pm = BlockyProfileManager::default();
                    pm.set_current_profile(&GBlockyProfile::new(&uuid, &username));
                });
            })
            .await
            .unwrap();
    }

    pub async fn remove_profile(&self, uuid: &Uuid) {
        let profiles = self.profiles();

        for pos in 0..profiles.n_items() {
            let profile = profiles
                .item(pos)
                .unwrap()
                .downcast::<GBlockyProfile>()
                .unwrap();

            if &profile.uuid() == uuid {
                profiles.remove(pos);
                return;
            }
        }
    }
}

impl Default for BlockyProfileManager {
    fn default() -> Self {
        BlockyApplication::default().profile_manager()
    }
}
