use crate::managers::BlockyInstanceManager;
use crate::ui::BlockyApplicationWindow;
use crate::utils::version_summary::{fetch_manifest, filter_versions, version_list_factory};
use adw::prelude::*;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::SingleSelection;
use libblocky::gobject::{GBlockyInstance, GBlockyVersionSummary};
use libblocky::instance::models::VersionSummary;
use once_cell::sync::{Lazy, OnceCell};
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/edit_instance_dialog.ui")]
    pub struct BlockyEditInstanceDialog {
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub pages_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,

        // General
        #[template_child]
        pub name_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub description_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub uuid_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub instance_location_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub libraries_location_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub assets_location_label: TemplateChild<gtk::Label>,

        pub instance: OnceCell<GBlockyInstance>,
        pub name_valid: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyEditInstanceDialog {
        const NAME: &'static str = "BlockyEditInstanceDialog";
        type Type = super::BlockyEditInstanceDialog;
        type ParentType = gtk::Dialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyEditInstanceDialog {
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
                    error!("Property {} not a member of BlockyEditInstanceDialog", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "instance" => self.instance.get().to_value(),
                x => {
                    error!("Property {} not a member of BlockyEditInstanceDialog", x);
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

    impl WidgetImpl for BlockyEditInstanceDialog {}

    impl WindowImpl for BlockyEditInstanceDialog {}

    impl DialogImpl for BlockyEditInstanceDialog {}
}

glib::wrapper! {
    pub struct BlockyEditInstanceDialog(ObjectSubclass<imp::BlockyEditInstanceDialog>)
    @extends gtk::Widget, gtk::Window, adw::Window, gtk::Dialog;
}

#[gtk::template_callbacks]
impl BlockyEditInstanceDialog {
    #[template_callback]
    fn save_button_clicked(&self) {
        let instance_manager = BlockyInstanceManager::default();
        info!("Saving instance");

        let instance = libblocky::Instance::from(self.instance());
        instance_manager.update_instance(instance);
        self.close();
    }

    #[template_callback]
    fn validate_name(&self) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        if imp.name_entry.text().is_empty() {
            imp.name_entry.add_css_class("error");
            imp.name_valid.set(false);
        } else {
            imp.name_entry.remove_css_class("error");
            imp.name_valid.set(true);
        }

        self.update_save_button();
    }
}

impl BlockyEditInstanceDialog {
    #[allow(clippy::new_without_default)]
    pub fn new(instance: &GBlockyInstance) -> Self {
        let dialog: Self =
            glib::Object::new(&[("use-header-bar", &1), ("instance", instance)]).unwrap();

        let window = BlockyApplicationWindow::default();
        dialog.set_transient_for(Some(&window));

        dialog
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        for view in View::iter() {
            let row = view.create_list_row();
            imp.pages_list.append(&row);
        }

        self.bind_property("name", &imp.name_entry.get(), "text");
        self.bind_property("description", &imp.description_entry.get(), "text");
        self.bind_property("version", &imp.version_label.get(), "label");
        self.bind_property("uuid", &imp.uuid_label.get(), "label");
        self.bind_property("instance-path", &imp.instance_location_label.get(), "label");
        self.bind_property(
            "libraries-path",
            &imp.libraries_location_label.get(),
            "label",
        );
        self.bind_property("assets-path", &imp.assets_location_label.get(), "label");
    }

    fn setup_signals(&self) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        // Sidebar
        imp.pages_list
            .connect_row_selected(glib::clone!(@weak self as this => move |_, row| {
                if let Some(row) = row {
                    let view = View::from(row.widget_name().as_str());
                    this.set_view(view);
                }
            }));
    }

    fn update_save_button(&self) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        let sensitive = imp.name_valid.get();
        imp.save_button.set_sensitive(sensitive);
    }

    fn set_view(&self, view: View) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        // Setup for view
        match view {
            View::General => {}
            View::Java => {}
            View::Game => {}
            View::Worlds => {}
            View::Servers => {}
            View::ScreenShots => {}
            View::ResourcePacks => {}
        }

        imp.stack.set_visible_child_name(view.get_id())
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
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
    }
}

#[derive(Clone, Debug, EnumIter)]
enum View {
    General,
    Java,
    Game,
    Worlds,
    Servers,
    ScreenShots,
    ResourcePacks,
}

impl From<&str> for View {
    fn from(str: &str) -> Self {
        match str {
            "general" => Self::General,
            "java" => Self::Java,
            "game" => Self::Game,
            "worlds" => Self::Worlds,
            "servers" => Self::Servers,
            "screenshots" => Self::ScreenShots,
            "resourcepacks" => Self::ResourcePacks,
            x => {
                error!("View '{}' does not exist", x);
                Self::General
            }
        }
    }
}

impl View {
    fn create_list_row(&self) -> gtk::ListBoxRow {
        let label = gtk::Label::builder()
            .xalign(0.0)
            .label(self.get_name())
            .build();

        let row = gtk::ListBoxRow::builder()
            .css_classes(vec!["sidebar-row".to_string()])
            .name(self.get_id())
            .child(&label)
            .build();

        row
    }

    fn get_id(&self) -> &'static str {
        match self {
            View::General => "general",
            View::Java => "java",
            View::Game => "game",
            View::Worlds => "worlds",
            View::Servers => "servers",
            View::ScreenShots => "screenshots",
            View::ResourcePacks => "resourcepacks",
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            View::General => "General",
            View::Java => "Java",
            View::Game => "Game",
            View::Worlds => "Worlds",
            View::Servers => "Servers",
            View::ScreenShots => "Screenshots",
            View::ResourcePacks => "Resourcepacks",
        }
    }
}
