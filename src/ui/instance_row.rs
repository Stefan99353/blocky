use crate::managers::BlockyInstanceManager;
use glib::subclass::{InitializingObject, InitializingType, Signal};
use glib::ToValue;
use glib::{IsA, ObjectExt, ParamSpec, Value};
use glib::{ParamFlags, ParamSpecObject, StaticType};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use libblocky::gobject::GBlockyInstance;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/instance_row.ui")]
    pub struct BlockyInstanceRow {
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,

        #[template_child]
        pub launch_button: TemplateChild<gtk::Button>,

        pub instance: OnceCell<GBlockyInstance>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstanceRow {
        const NAME: &'static str = "BlockyInstanceRow";
        type Type = super::BlockyInstanceRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyInstanceRow {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::new(
                    "instance",
                    "Instance",
                    "Instance",
                    GBlockyInstance::static_type(),
                    ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                )]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "instance" => self.instance.set(value.get().unwrap()).unwrap(),
                x => {
                    error!("Property {} not a member of BlockyInstanceRow", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "instance" => self.instance.get().to_value(),
                x => {
                    error!("Property {} not a member of BlockyInstanceRow", x);
                    unimplemented!()
                }
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_signals();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyInstanceRow {}

    impl ListBoxRowImpl for BlockyInstanceRow {}
}

glib::wrapper! {
    pub struct BlockyInstanceRow(ObjectSubclass<imp::BlockyInstanceRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl BlockyInstanceRow {
    pub fn new(instance: &GBlockyInstance) -> Self {
        glib::Object::new(&[("instance", instance)]).unwrap()
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyInstanceRow::from_instance(self);

        self.bind_property("name", &imp.name_label.get(), "label");
        self.bind_property("description", &imp.description_label.get(), "label");
        self.bind_property("version", &imp.version_label.get(), "label");
    }

    fn setup_signals(&self) {
        let imp = imp::BlockyInstanceRow::from_instance(self);

        imp.launch_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.launch();
            }));
    }

    fn launch(&self) {
        let instance_manager = BlockyInstanceManager::default();
        let uuid = self.instance().uuid();

        let receiver = instance_manager.launch_instance(uuid);

        receiver.attach(None, move |update| {
            match update {
                Ok(update) => {
                    info!("{:?}", update);
                }
                Err(err) => {
                    error!("Error during installation: {}", err);
                }
            }

            glib::Continue(true)
        });
    }

    pub fn instance(&self) -> GBlockyInstance {
        self.property("instance")
    }

    fn bind_property<T: IsA<gtk::Widget>>(
        &self,
        prop_name: &str,
        widget: &T,
        widget_prop_name: &str,
    ) {
        self.instance()
            .bind_property(prop_name, widget, widget_prop_name)
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
    }
}
