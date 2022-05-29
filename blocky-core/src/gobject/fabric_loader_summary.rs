use crate::minecraft::fabric::loader_manifest::FabricLoaderSummary;
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::ObjectSubclass;
use glib::{
    ObjectExt, ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecInt, ParamSpecString, ToValue,
    Value,
};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

pub const SEPARATOR: &str = "separator";
pub const BUILD: &str = "build";
pub const MAVEN: &str = "maven";
pub const VERSION: &str = "version";
pub const STABLE: &str = "stable";

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GFabricLoaderSummary {
        pub separator: RefCell<String>,
        pub build: Cell<i32>,
        pub maven: RefCell<String>,
        pub version: RefCell<String>,
        pub stable: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GFabricLoaderSummary {
        const NAME: &'static str = "GFabricLoaderSummary";
        type Type = super::GFabricLoaderSummary;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for GFabricLoaderSummary {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        SEPARATOR,
                        "Separator",
                        "Separator",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecInt::new(
                        BUILD,
                        "Build",
                        "Build",
                        i32::MIN,
                        i32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(MAVEN, "Maven", "Maven", None, ParamFlags::READWRITE),
                    ParamSpecString::new(
                        VERSION,
                        "Version",
                        "Version",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(STABLE, "Stable", "Stable", false, ParamFlags::READWRITE),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                SEPARATOR => *self.separator.borrow_mut() = value.get().unwrap(),
                BUILD => self.build.set(value.get().unwrap()),
                MAVEN => *self.maven.borrow_mut() = value.get().unwrap(),
                VERSION => *self.version.borrow_mut() = value.get().unwrap(),
                STABLE => self.stable.set(value.get().unwrap()),
                x => {
                    error!("Property {} not a member of GFabricLoaderSummary", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                SEPARATOR => self.separator.borrow().to_value(),
                BUILD => self.build.get().to_value(),
                MAVEN => self.maven.borrow().to_value(),
                VERSION => self.version.borrow().to_value(),
                STABLE => self.stable.get().to_value(),
                x => {
                    error!("Property {} not a member of GFabricLoaderSummary", x);
                    unimplemented!()
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct GFabricLoaderSummary(ObjectSubclass<imp::GFabricLoaderSummary>);
}

impl GFabricLoaderSummary {
    pub fn separator(&self) -> String {
        self.property(SEPARATOR)
    }
    pub fn build(&self) -> i32 {
        self.property(BUILD)
    }
    pub fn maven(&self) -> String {
        self.property(MAVEN)
    }
    pub fn version(&self) -> String {
        self.property(VERSION)
    }
    pub fn stable(&self) -> bool {
        self.property(STABLE)
    }
}

impl Default for GFabricLoaderSummary {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

impl From<FabricLoaderSummary> for GFabricLoaderSummary {
    fn from(summary: FabricLoaderSummary) -> Self {
        glib::Object::new(&[
            (SEPARATOR, &summary.separator),
            (BUILD, &summary.build),
            (MAVEN, &summary.maven),
            (VERSION, &summary.version),
            (STABLE, &summary.stable),
        ])
        .unwrap()
    }
}
