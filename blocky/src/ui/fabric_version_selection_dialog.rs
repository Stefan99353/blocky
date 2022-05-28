use crate::helpers;
use crate::utils::fabric_loader_summary::fabric_loader_list_factory;
use blocky_core::gobject::fabric::fabric_loader_summary::GFabricLoaderSummary;
use blocky_core::gobject::{instance, GInstance};
use blocky_core::minecraft::fabric::loader_manifest::FabricLoaderSummary;
use gio::ListStore;
use glib::subclass::InitializingObject;
use glib::ParamSpecString;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::SingleSelection;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::cell::{Cell, RefCell};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/fabric_version_selection_dialog.ui")]
    pub struct BlockyFabricVersionSelectionDialog {
        #[template_child]
        pub select_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub version_list: TemplateChild<gtk::ListView>,

        pub instance: OnceCell<GInstance>,
        pub version: RefCell<String>,
        pub manifest: RefCell<Vec<FabricLoaderSummary>>,
        pub version_list_store: ListStore,
        pub version_selection_model: SingleSelection,

        pub version_valid: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyFabricVersionSelectionDialog {
        const NAME: &'static str = "BlockyFabricVersionSelectionDialog";
        type Type = super::BlockyFabricVersionSelectionDialog;
        type ParentType = gtk::Dialog;

        fn new() -> Self {
            let list_store = gio::ListStore::new(GFabricLoaderSummary::static_type());

            let selection = SingleSelection::builder()
                .autoselect(false)
                .model(&list_store)
                .build();

            Self {
                select_button: Default::default(),
                version_label: Default::default(),
                version_list: Default::default(),
                instance: Default::default(),
                version: Default::default(),
                manifest: Default::default(),
                version_list_store: list_store,
                version_selection_model: selection,
                version_valid: Default::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyFabricVersionSelectionDialog {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecObject::new(
                        "instance",
                        "Instance",
                        "Instance",
                        GInstance::static_type(),
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                    ParamSpecString::new(
                        "version",
                        "Version",
                        "Version",
                        None,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "instance" => self.instance.set(value.get().unwrap()).unwrap(),
                "version" => *self.version.borrow_mut() = value.get().unwrap(),
                x => {
                    error!(
                        "Property {} not a member of BlockyFabricVersionSelectionDialog",
                        x
                    );
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "instance" => self.instance.get().to_value(),
                "version" => self.version.borrow().to_value(),
                x => {
                    error!(
                        "Property {} not a member of BlockyFabricVersionSelectionDialog",
                        x
                    );
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

    impl WidgetImpl for BlockyFabricVersionSelectionDialog {}

    impl WindowImpl for BlockyFabricVersionSelectionDialog {}

    impl DialogImpl for BlockyFabricVersionSelectionDialog {}
}

glib::wrapper! {
    pub struct BlockyFabricVersionSelectionDialog(ObjectSubclass<imp::BlockyFabricVersionSelectionDialog>)
    @extends gtk::Widget, gtk::Window, adw::Window, gtk::Dialog;
}

#[gtk::template_callbacks]
impl BlockyFabricVersionSelectionDialog {
    #[template_callback]
    fn select_button_clicked(&self) {
        let instance = self.instance();
        instance.set_property(instance::FABRIC_VERSION, self.version());
        instance.notify(instance::FABRIC_VERSION);
        self.close();
    }

    #[template_callback]
    fn validate_version(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        if self.version().is_empty() {
            imp.version_valid.set(false);
        } else {
            imp.version_valid.set(true);
        }

        self.update_select_button();
    }
}

impl BlockyFabricVersionSelectionDialog {
    pub fn new(parent: &gtk::Window, instance: &GInstance) -> Self {
        let init_version = instance.property::<String>(instance::FABRIC_VERSION);
        let dialog: Self = glib::Object::new(&[
            ("use-header-bar", &1),
            ("instance", instance),
            ("version", &init_version),
        ])
        .unwrap();

        dialog.set_transient_for(Some(parent));

        dialog
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        imp.version_list
            .set_factory(Some(&fabric_loader_list_factory()));
        imp.version_list
            .set_model(Some(&imp.version_selection_model));

        self.bind_property("version", &imp.version_label.get(), "label")
            .flags(glib::BindingFlags::DEFAULT)
            .build();

        let game_version = self.instance().property(instance::VERSION);
        helpers::fabric::get_loader_manifest(game_version).0.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |manifest| {
                    let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(&this);
                    *imp.manifest.borrow_mut() = manifest;
                    this.refresh_version_list();
                    glib::Continue(true)
                }
            ),
        );
    }

    fn setup_signals(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        imp.version_selection_model.connect_selected_notify(
            glib::clone!(@weak self as this => move |_| {
                this.update_selected_version();
            }),
        );
    }

    fn update_selected_version(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        if let Some(item) = imp.version_selection_model.selected_item() {
            let summary = item.downcast::<GFabricLoaderSummary>().unwrap();

            self.set_version(summary.version());
            imp.version_valid.set(true);
        } else {
            self.set_version(String::new());
            imp.version_valid.set(false);
        }

        self.update_select_button();
    }

    fn refresh_version_list(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        let versions = imp
            .manifest
            .borrow()
            .iter()
            .map(|s| GFabricLoaderSummary::from(s.clone()))
            .collect::<Vec<GFabricLoaderSummary>>();

        imp.version_list_store
            .splice(0, imp.version_list_store.n_items(), &versions);

        if !versions.is_empty() {
            imp.version_selection_model.set_selected(0);
        }

        self.update_selected_version();
    }

    fn update_select_button(&self) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        let sensitive = imp.version_valid.get();
        imp.select_button.set_sensitive(sensitive);
    }

    pub fn instance(&self) -> GInstance {
        self.property("instance")
    }
    pub fn version(&self) -> String {
        self.property("version")
    }
    pub fn set_version(&self, version: String) {
        let imp = imp::BlockyFabricVersionSelectionDialog::from_instance(self);

        *imp.version.borrow_mut() = version;
        self.notify("version");
    }
}
