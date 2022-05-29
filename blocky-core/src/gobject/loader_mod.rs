use crate::instance::mods::loader_mod::LoaderMod;
use glib::subclass::prelude::*;
use glib::{ObjectExt, ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecString, ToValue, Value};
use once_cell::sync::{Lazy, OnceCell};
use std::cell::{Cell, RefCell};
use std::str::FromStr;
use uuid::Uuid;

pub const UUID: &str = "uuid";
pub const FILENAME: &str = "filename";
pub const ENABLED: &str = "enabled";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GLoaderMod {
        pub uuid: OnceCell<String>,
        pub filename: RefCell<String>,
        pub enabled: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GLoaderMod {
        const NAME: &'static str = "GLoaderMod";
        type Type = super::GLoaderMod;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GLoaderMod {
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
                    ParamSpecString::new(FILENAME, "UUID", "UUID", None, ParamFlags::READWRITE),
                    ParamSpecBoolean::new(
                        ENABLED,
                        "Enabled",
                        "Enabled",
                        false,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                UUID => self.uuid.set(value.get().unwrap()).unwrap(),
                FILENAME => *self.filename.borrow_mut() = value.get().unwrap(),
                ENABLED => self.enabled.set(value.get().unwrap()),
                x => {
                    error!("Property {} not a member of GLoaderMod", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                UUID => self.uuid.get().to_value(),
                FILENAME => self.filename.borrow().to_value(),
                ENABLED => self.enabled.get().to_value(),
                x => {
                    error!("Property {} not a member of GLoaderMod", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GLoaderMod(ObjectSubclass<imp::GLoaderMod>);
}

impl GLoaderMod {
    pub fn uuid(&self) -> Uuid {
        let uuid: String = self.property(UUID);
        Uuid::from_str(&uuid).unwrap()
    }
}

impl From<LoaderMod> for GLoaderMod {
    fn from(loader_mod: LoaderMod) -> Self {
        glib::Object::new(&[
            (UUID, &loader_mod.uuid.to_string()),
            (FILENAME, &loader_mod.filename),
            (ENABLED, &loader_mod.enabled),
        ])
        .unwrap()
    }
}
