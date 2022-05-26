use crate::managers::BlockyInstanceManager;
use crate::ui::BlockyApplicationWindow;
use adw::prelude::*;
use blocky_core::gobject::instance;
use blocky_core::gobject::GInstance;
use blocky_core::instance::Instance;
use gettextrs::gettext;
use glib::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, FileChooserAction, FileChooserNative, ResponseType};
use once_cell::sync::{Lazy, OnceCell};
use std::cell::Cell;
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
        pub instance_path_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub libraries_path_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub assets_path_label: TemplateChild<gtk::Label>,

        // Java
        #[template_child]
        pub override_memory_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub min_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub max_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub override_java_exec_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub java_exec_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub java_exec_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub override_jvm_args_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub jvm_args_entry: TemplateChild<gtk::Entry>,

        // Game
        #[template_child]
        pub fullscreen_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub override_window_size_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub window_width_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub window_height_spinbutton: TemplateChild<gtk::SpinButton>,

        // Fabric
        #[template_child]
        pub use_fabric_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub fabric_install_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub fabric_version_label: TemplateChild<gtk::Label>,

        pub instance: OnceCell<GInstance>,
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
                    GInstance::static_type(),
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
        let g_instance = self.instance();
        info!("Saving instance");

        // Check Fabric version
        if g_instance
            .property::<String>(instance::FABRIC_VERSION)
            .is_empty()
        {
            g_instance.set_property(instance::USE_FABRIC, false);
        }

        let instance = Instance::from(g_instance);
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
    pub fn new(instance: &GInstance) -> Self {
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

        // General
        self.bind_property(instance::NAME, &imp.name_entry.get(), "text");
        self.bind_property(instance::DESCRIPTION, &imp.description_entry.get(), "text");
        self.bind_property(instance::VERSION, &imp.version_label.get(), "label");
        self.bind_property(instance::UUID, &imp.uuid_label.get(), "label");
        self.bind_property(
            instance::INSTANCE_PATH,
            &imp.instance_path_label.get(),
            "label",
        );
        self.bind_property(
            instance::LIBRARIES_PATH,
            &imp.libraries_path_label.get(),
            "label",
        );
        self.bind_property(instance::ASSETS_PATH, &imp.assets_path_label.get(), "label");

        // Java
        self.bind_property(
            instance::ENABLE_MEMORY,
            &imp.override_memory_expander.get(),
            "enable-expansion",
        );
        self.bind_property(
            instance::MIN_MEMORY,
            &imp.min_memory_spinbutton.get(),
            "value",
        );
        self.bind_property(
            instance::MAX_MEMORY,
            &imp.max_memory_spinbutton.get(),
            "value",
        );
        self.bind_property(
            instance::ENABLE_JAVA_EXEC,
            &imp.override_java_exec_expander.get(),
            "enable-expansion",
        );
        self.bind_property(instance::JAVA_EXEC, &imp.java_exec_label.get(), "label");
        self.bind_property(
            instance::ENABLE_JVM_ARGS,
            &imp.override_jvm_args_expander.get(),
            "enable-expansion",
        );
        self.bind_property(instance::JVM_ARGS, &imp.jvm_args_entry.get(), "text");

        // Game
        self.bind_property(
            instance::USE_FULLSCREEN,
            &imp.fullscreen_switch.get(),
            "state",
        );
        self.bind_property(
            instance::ENABLE_WINDOW_SIZE,
            &imp.override_window_size_expander.get(),
            "enable-expansion",
        );
        self.bind_property(
            instance::WINDOW_WIDTH,
            &imp.window_width_spinbutton.get(),
            "value",
        );
        self.bind_property(
            instance::WINDOW_HEIGHT,
            &imp.window_height_spinbutton.get(),
            "value",
        );

        // Fabric
        self.bind_property(
            instance::USE_FABRIC,
            &imp.use_fabric_expander.get(),
            "enable-expansion",
        );
        self.bind_property(
            instance::FABRIC_VERSION,
            &imp.fabric_version_label.get(),
            "label",
        );
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

        // Java executable
        let (java_sender, java_receiver) =
            glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);
        imp.java_exec_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.file_chooser(&gettext("Select Java Executable"), java_sender.clone());
            }));
        java_receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |path| {
                    let imp = imp::BlockyEditInstanceDialog::from_instance(&this);
                    imp.java_exec_label.set_label(&path);
                    glib::Continue(true)
                }
            ),
        );

        // Install Fabric
        imp.fabric_install_button.connect_clicked(move |_| {
            debug!("Installing fabric");
        });
    }

    fn update_save_button(&self) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        let sensitive = imp.name_valid.get();
        imp.save_button.set_sensitive(sensitive);
    }

    fn file_chooser(&self, title: &str, sender: glib::Sender<String>) {
        let dialog = FileChooserNative::new(
            Some(title),
            Some(self),
            FileChooserAction::Open,
            Some(&gettext("Select")),
            Some(&gettext("Cancel")),
        );

        dialog.connect_response(
            glib::clone!(@strong dialog, @weak self as this => move |_, resp| {
                dialog.destroy();
                if resp == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            if let Some(path_string) = path.to_str() {
                                debug!("Selected file: {}", path_string);
                                sender.send(path_string.to_string()).expect("Could not send path through channel");
                            }
                        }
                    }
                }
            }),
        );

        dialog.show();
    }

    fn set_view(&self, view: View) {
        let imp = imp::BlockyEditInstanceDialog::from_instance(self);

        // Setup for view
        match view {
            View::General => {}
            View::Java => {}
            View::Game => {}
            View::Fabric => {}
            View::Saves => {}
            View::Servers => {}
            View::ScreenShots => {}
            View::ResourcePacks => {}
            View::Logs => {}
        }

        imp.stack.set_visible_child_name(view.get_id())
    }

    pub fn instance(&self) -> GInstance {
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
    Fabric,
    Saves,
    Servers,
    ScreenShots,
    ResourcePacks,
    Logs,
}

impl From<&str> for View {
    fn from(str: &str) -> Self {
        match str {
            "general" => Self::General,
            "java" => Self::Java,
            "game" => Self::Game,
            "fabric" => Self::Fabric,
            "saves" => Self::Saves,
            "servers" => Self::Servers,
            "screenshots" => Self::ScreenShots,
            "resourcepacks" => Self::ResourcePacks,
            "logs" => Self::Logs,
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
            View::Fabric => "fabric",
            View::Saves => "saves",
            View::Servers => "servers",
            View::ScreenShots => "screenshots",
            View::ResourcePacks => "resourcepacks",
            View::Logs => "logs",
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            View::General => "General",
            View::Java => "Java",
            View::Game => "Game",
            View::Fabric => "Fabric",
            View::Saves => "Saves",
            View::Servers => "Servers",
            View::ScreenShots => "Screenshots",
            View::ResourcePacks => "Resourcepacks",
            View::Logs => "Logs",
        }
    }
}
