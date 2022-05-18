use crate::settings::SettingKey;
use crate::{helpers, settings, BlockyApplication};
use blocky_core::gobject::profile::GProfile;
use blocky_core::profile::Profile;
use gio::traits::{ListModelExt, SettingsExt};
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::{
    Cast, MainContext, ObjectExt, ParamFlags, ParamSpec, ParamSpecObject, StaticType, ToValue,
    Value,
};
use gtk_macros::send;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::str::FromStr;
use std::thread;
use uuid::Uuid;

pub const PROFILES: &str = "profiles";
pub const CURRENT_PROFILE: &str = "current-profile";

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct BlockyProfileManager {
        pub profiles: ListStore,
        pub current_profile: RefCell<GProfile>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyProfileManager {
        const NAME: &'static str = "BlockyProfileManager";
        type Type = super::BlockyProfileManager;
        type ParentType = glib::Object;

        fn new() -> Self {
            let profiles = ListStore::new(GProfile::static_type());

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
                        PROFILES,
                        "Profiles",
                        "Profiles",
                        ListStore::static_type(),
                        ParamFlags::READABLE,
                    ),
                    ParamSpecObject::new(
                        CURRENT_PROFILE,
                        "Current Profile",
                        "Current Profile",
                        GProfile::static_type(),
                        ParamFlags::READABLE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                PROFILES => self.profiles.to_value(),
                CURRENT_PROFILE => self.current_profile.borrow().to_value(),
                x => {
                    error!("Property {} not a member of BlockyProfileManager", x);
                    unimplemented!()
                }
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
        self.property(PROFILES)
    }

    pub fn current_profile(&self) -> Option<GProfile> {
        let profile = self.property::<GProfile>(CURRENT_PROFILE);

        if profile.uuid().is_nil() {
            return None;
        }

        Some(profile)
    }

    pub fn set_current_profile(&self, profile: &GProfile) {
        let imp = imp::BlockyProfileManager::from_instance(self);

        let uuid = profile.uuid().to_string();
        settings::set_string(SettingKey::DefaultProfile, &uuid);

        *imp.current_profile.borrow_mut() = profile.clone();
        self.notify(CURRENT_PROFILE);
    }

    pub fn set_current_profile_by_uuid(&self, uuid: Uuid) {
        if let Some(profile) = self.find_profile(&uuid) {
            self.set_current_profile(&profile);
        }
    }

    pub fn find_profile(&self, uuid: &Uuid) -> Option<GProfile> {
        let profiles = self.profiles();

        for pos in 0..profiles.n_items() {
            let profile = profiles.item(pos).unwrap().downcast::<GProfile>().unwrap();

            if &profile.uuid() == uuid {
                return Some(profile);
            }
        }

        None
    }

    pub fn initialize(&self) {
        self.full_profiles().attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false), move |profiles| {
                // Add profiles
                let profiles = profiles
                    .into_iter()
                    .map(|p| {
                        let uuid = p.uuid;
                        let username = p.minecraft_profile.map(|mp| mp.name).unwrap_or_else(|| uuid.to_string());

                        GProfile::new(&uuid, &username)
                    })
                    .collect::<Vec<GProfile>>();

                this.profiles().splice(0, this.profiles().n_items(), &profiles);
                this.notify(PROFILES);

                // Set default
                let default = settings::get_string(SettingKey::DefaultProfile);
                if let Ok(uuid) = Uuid::from_str(&default) {
                    match this.find_profile(&uuid) {
                        None => {
                            settings::get_settings().reset(SettingKey::DefaultProfile.to_key())
                        }
                        Some(profile) => {
                            this.set_current_profile(&profile);
                        }
                    }
                }

                glib::Continue(false)
            })
        );
    }

    pub fn full_profiles(&self) -> glib::Receiver<Vec<Profile>> {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let profiles = helpers::profiles::load_profiles();
            send!(sender, profiles);
        });

        receiver
    }

    pub fn find_full_profile(&self, profile: &GProfile) -> glib::Receiver<Option<Profile>> {
        let uuid = profile.uuid();
        self.find_full_profile_by_uuid(uuid)
    }

    pub fn find_full_profile_by_uuid(&self, uuid: Uuid) -> glib::Receiver<Option<Profile>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            let profile = helpers::profiles::find_profile(uuid);
            send!(sender, profile);
        });

        receiver
    }

    pub fn full_current_profile(&self) -> glib::Receiver<Option<Profile>> {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);

        match self.current_profile() {
            None => send!(sender, None),
            Some(profile) => {
                let uuid = profile.uuid();

                thread::spawn(move || {
                    let profile = helpers::profiles::find_profile(uuid);
                    send!(sender, profile);
                });
            }
        }

        receiver
    }

    pub fn add_profile(&self, profile: Profile) {
        // Add to ListStore
        let uuid = profile.uuid;
        let username = profile.minecraft_profile.as_ref().unwrap().name.clone();
        let g_profile = GProfile::new(&uuid, &username);
        self.profiles().append(&g_profile);
        self.notify(PROFILES);

        // Set as current
        self.set_current_profile(&g_profile);

        // Add to disk
        thread::spawn(move || {
            helpers::profiles::save_profile(profile);
        });
    }

    pub fn remove_profile(&self, profile: &GProfile) {
        let uuid = profile.uuid();
        self.remove_profile_by_uuid(uuid);
    }

    pub fn remove_profile_by_uuid(&self, uuid: Uuid) {
        // Remove from ListStore
        let profiles = self.profiles();
        for pos in 0..profiles.n_items() {
            let profile = profiles.item(pos).unwrap().downcast::<GProfile>().unwrap();

            if profile.uuid() == uuid {
                profiles.remove(pos);
                break;
            }
        }

        self.notify(PROFILES);

        // Remove from disk
        thread::spawn(move || {
            helpers::profiles::remove_profile(uuid);
        });
    }
}

impl Default for BlockyProfileManager {
    fn default() -> Self {
        BlockyApplication::default().profile_manager()
    }
}
