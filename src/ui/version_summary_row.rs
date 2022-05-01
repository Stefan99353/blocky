use glib::subclass::InitializingObject;
use glib::{IsA, ObjectExt, ParamFlags, ParamSpec, ParamSpecObject, StaticType, ToValue, Value};
use gtk::prelude::InitializingWidgetExt;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use libblocky::gobject::GBlockyVersionSummary;
use once_cell::sync::Lazy;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/version_summary_row.ui")]
    pub struct BlockyVersionSummaryRow {
        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub type_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub release_label: TemplateChild<gtk::Label>,

        pub version_summary: RefCell<GBlockyVersionSummary>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyVersionSummaryRow {
        const NAME: &'static str = "BlockyVersionSummaryRow";
        type Type = super::BlockyVersionSummaryRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyVersionSummaryRow {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::new(
                    "version-summary",
                    "Version Summary",
                    "Version Summary",
                    GBlockyVersionSummary::static_type(),
                    ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "version-summary" => match value.get::<GBlockyVersionSummary>() {
                    Ok(value) => *self.version_summary.borrow_mut() = value,
                    Err(err) => {
                        error!("Set version-summary: {}", err);
                    }
                },
                x => {
                    error!("Property {} not a member of BlockyVersionSummaryRow", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "version-summary" => self.version_summary.borrow().to_value(),
                x => {
                    error!("Property {} not a member of BlockyVersionSummaryRow", x);
                    unimplemented!()
                }
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyVersionSummaryRow {}

    impl BoxImpl for BlockyVersionSummaryRow {}
}

glib::wrapper! {
    pub struct BlockyVersionSummaryRow(ObjectSubclass<imp::BlockyVersionSummaryRow>)
        @extends gtk::Widget, gtk::Box;
}

impl BlockyVersionSummaryRow {
    pub fn new(version: &GBlockyVersionSummary) -> Self {
        glib::Object::new(&[("version-summary", version)]).unwrap()
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyVersionSummaryRow::from_instance(self);

        self.bind_property("id", &imp.version_label.get(), "label");
        self.bind_property("type", &imp.type_label.get(), "label");
        self.bind_property("release-time", &imp.release_label.get(), "label");
    }

    pub fn set_version_summary(&self, summary: &GBlockyVersionSummary) {
        let imp = imp::BlockyVersionSummaryRow::from_instance(self);

        *imp.version_summary.borrow_mut() = summary.clone();
        self.notify("version-summary");
    }

    pub fn version_summary(&self) -> GBlockyVersionSummary {
        self.property("version-summary")
    }

    fn bind_property<T: IsA<gtk::Widget>>(
        &self,
        prop_name: &str,
        widget: &T,
        widget_prop_name: &str,
    ) {
        self.version_summary()
            .bind_property(prop_name, widget, widget_prop_name)
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
    }
}
