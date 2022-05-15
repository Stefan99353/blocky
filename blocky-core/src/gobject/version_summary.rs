use crate::instance::models::{VersionSummary, VersionType};
use chrono::{DateTime, Utc};
use glib::subclass::prelude::*;
use glib::{ObjectExt, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::str::FromStr;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GBlockyVersionSummary {
        pub id: RefCell<String>,
        pub _type: RefCell<String>,
        pub release_time: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GBlockyVersionSummary {
        const NAME: &'static str = "GBlockyVersionSummary";
        type Type = super::GBlockyVersionSummary;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GBlockyVersionSummary {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("id", "ID", "ID", None, ParamFlags::READWRITE),
                    ParamSpecString::new("type", "Type", "Type", None, ParamFlags::READWRITE),
                    ParamSpecString::new(
                        "release-time",
                        "Release Time",
                        "Release Time",
                        None,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "id" => *self.id.borrow_mut() = value.get().unwrap(),
                "type" => *self._type.borrow_mut() = value.get().unwrap(),
                "release-time" => *self.release_time.borrow_mut() = value.get().unwrap(),
                x => {
                    error!("Property {} not a member of GBlockyVersionSummary", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "id" => self.id.borrow().to_value(),
                "type" => self._type.borrow().to_value(),
                "release-time" => self.release_time.borrow().to_value(),
                x => {
                    error!("Property {} not a member of GBlockyVersionSummary", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GBlockyVersionSummary(ObjectSubclass<imp::GBlockyVersionSummary>);
}

impl GBlockyVersionSummary {
    pub fn id(&self) -> String {
        self.property("id")
    }

    pub fn _type(&self) -> VersionType {
        let typ = self.property::<String>("type");
        match typ.as_str() {
            "release" => VersionType::Release,
            "snapshot" => VersionType::Snapshot,
            "old_alpha" => VersionType::OldAlpha,
            "old_beta" => VersionType::OldBeta,
            x => {
                error!("'{}' is not a valid version type", x);
                unimplemented!();
            }
        }
    }

    pub fn release_time(&self) -> DateTime<Utc> {
        let release_time = self.property::<String>("release-time");
        DateTime::from_str(&release_time).expect("Invalid release time")
    }
}

impl Default for GBlockyVersionSummary {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

impl From<VersionSummary> for GBlockyVersionSummary {
    fn from(summary: VersionSummary) -> Self {
        glib::Object::new(&[
            ("id", &summary.id),
            ("type", &summary._type.to_string()),
            ("release-time", &summary.release_time.to_string()),
        ])
        .unwrap()
    }
}
