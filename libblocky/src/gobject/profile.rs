use glib::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{ObjectExt, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value};
use once_cell::sync::{Lazy, OnceCell};
use std::process::id;
use std::str::FromStr;
use uuid::Uuid;

pub const NO_PROFILE_UUID: &str = "00000000-0000-0000-0000-000000000000";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GBlockyProfile {
        pub uuid: OnceCell<String>,
        pub username: OnceCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GBlockyProfile {
        const NAME: &'static str = "GBlockyProfile";
        type Type = super::GBlockyProfile;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GBlockyProfile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        "uuid",
                        "UUID",
                        "UUID",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                    ParamSpecString::new(
                        "username",
                        "Username",
                        "Username",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "uuid" => self.uuid.set(value.get().unwrap()).unwrap(),
                "username" => self.username.set(value.get().unwrap()).unwrap(),
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "uuid" => self.uuid.get().to_value(),
                "username" => self.username.get().to_value(),
                x => {
                    error!("Property {} not a member of GBlockyProfile", x);
                    unimplemented!()
                }
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

glib::wrapper! {
    pub struct GBlockyProfile(ObjectSubclass<imp::GBlockyProfile>);
}

impl GBlockyProfile {
    pub fn new(uuid: &Uuid, username: &str) -> Self {
        let uuid = uuid.to_string();
        glib::Object::new(&[("uuid", &uuid.as_str()), ("username", &username)]).unwrap()
    }

    pub fn uuid(&self) -> Uuid {
        let id: String = self.property("uuid");
        Uuid::from_str(&id).unwrap()
    }

    pub fn username(&self) -> String {
        self.property("username")
    }
}

impl Default for GBlockyProfile {
    fn default() -> Self {
        let uuid = Uuid::from_str(NO_PROFILE_UUID).unwrap();
        let username = "";

        Self::new(&uuid, username)
    }
}
