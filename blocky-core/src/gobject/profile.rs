use glib::subclass::prelude::*;
use glib::{ObjectExt, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value};
use once_cell::sync::{Lazy, OnceCell};
use std::str::FromStr;
use uuid::Uuid;

pub const UUID: &str = "uuid";
pub const USERNAME: &str = "username";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GProfile {
        pub uuid: OnceCell<String>,
        pub username: OnceCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GProfile {
        const NAME: &'static str = "GProfile";
        type Type = super::GProfile;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GProfile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        UUID,
                        "UUID",
                        "UUID",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                    ParamSpecString::new(
                        USERNAME,
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
                UUID => self.uuid.set(value.get().unwrap()).unwrap(),
                USERNAME => self.username.set(value.get().unwrap()).unwrap(),
                x => {
                    error!("Property {} not a member of GProfile", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                UUID => self.uuid.get().to_value(),
                USERNAME => self.username.get().to_value(),
                x => {
                    error!("Property {} not a member of GProfile", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GProfile(ObjectSubclass<imp::GProfile>);
}

impl GProfile {
    pub fn new(uuid: &Uuid, username: &str) -> Self {
        let uuid = uuid.to_string();
        glib::Object::new(&[(UUID, &uuid.as_str()), (USERNAME, &username)]).unwrap()
    }

    pub fn uuid(&self) -> Uuid {
        let uuid: String = self.property(UUID);
        Uuid::from_str(&uuid).unwrap()
    }

    pub fn username(&self) -> String {
        self.property(USERNAME)
    }
}

impl Default for GProfile {
    fn default() -> Self {
        let uuid = Uuid::nil();
        let username = "";

        Self::new(&uuid, username)
    }
}
