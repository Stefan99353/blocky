use crate::minecraft::models::version_summary::VersionSummary;
use crate::minecraft::models::version_type::VersionType;
use chrono::{DateTime, Utc};
use glib::subclass::prelude::*;
use glib::{ObjectExt, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::str::FromStr;

pub const ID: &str = "id";
pub const TYPE: &str = "type";
pub const RELEASE_TIME: &str = "release-time";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GVersionSummary {
        pub id: RefCell<String>,
        pub _type: RefCell<String>,
        pub release_time: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GVersionSummary {
        const NAME: &'static str = "GVersionSummary";
        type Type = super::GVersionSummary;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GVersionSummary {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(ID, "ID", "ID", None, ParamFlags::READWRITE),
                    ParamSpecString::new(TYPE, "Type", "Type", None, ParamFlags::READWRITE),
                    ParamSpecString::new(
                        RELEASE_TIME,
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
                ID => *self.id.borrow_mut() = value.get().unwrap(),
                TYPE => *self._type.borrow_mut() = value.get().unwrap(),
                RELEASE_TIME => *self.release_time.borrow_mut() = value.get().unwrap(),
                x => {
                    error!("Property {} not a member of GVersionSummary", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                ID => self.id.borrow().to_value(),
                TYPE => self._type.borrow().to_value(),
                RELEASE_TIME => self.release_time.borrow().to_value(),
                x => {
                    error!("Property {} not a member of GVersionSummary", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GVersionSummary(ObjectSubclass<imp::GVersionSummary>);
}

impl GVersionSummary {
    pub fn id(&self) -> String {
        self.property(ID)
    }

    pub fn _type(&self) -> VersionType {
        let typ = self.property::<String>(TYPE);
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
        let release_time = self.property::<String>(RELEASE_TIME);
        DateTime::from_str(&release_time).expect("Invalid release time")
    }
}

impl Default for GVersionSummary {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

impl From<VersionSummary> for GVersionSummary {
    fn from(summary: VersionSummary) -> Self {
        glib::Object::new(&[
            (ID, &summary.id),
            (TYPE, &summary._type.to_string()),
            (RELEASE_TIME, &summary.release_time.to_string()),
        ])
        .unwrap()
    }
}
