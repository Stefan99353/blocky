use crate::glib::subclass::{InitializingObject, InitializingType, Signal};
use crate::glib::{ParamSpec, Value};
use crate::settings;
use crate::settings::SettingKey;
use crate::ui::BlockyApplicationWindow;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, FileChooserAction, FileChooserNative, ResponseType};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/preferences_window.ui")]
    pub struct BlockyPreferencesWindow {
        // Launcher Page
        #[template_child]
        pub default_instances_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub default_instances_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub default_libraries_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub default_libraries_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub default_assets_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub default_assets_dir_label: TemplateChild<gtk::Label>,

        // Minecraft Page
        #[template_child]
        pub default_fullscreen_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub default_width_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub default_height_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub default_min_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub default_max_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub default_java_exec_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub default_java_exec_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub default_use_jvm_args_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub default_jvm_args_text: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyPreferencesWindow {
        const NAME: &'static str = "BlockyPreferencesWindow";
        type Type = super::BlockyPreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyPreferencesWindow {
        fn constructed(&self, obj: &Self::Type) {
            let main_window = BlockyApplicationWindow::default();
            obj.set_transient_for(Some(&main_window));

            obj.setup_widgets();
            obj.setup_signals();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyPreferencesWindow {}

    impl WindowImpl for BlockyPreferencesWindow {}

    impl AdwWindowImpl for BlockyPreferencesWindow {}

    impl PreferencesWindowImpl for BlockyPreferencesWindow {}
}

glib::wrapper! {
    pub struct BlockyPreferencesWindow(ObjectSubclass<imp::BlockyPreferencesWindow>)
    @extends gtk::Widget, gtk::Window, adw::Window, adw::PreferencesWindow;
}

impl BlockyPreferencesWindow {
    pub fn new() -> Self {
        glib::Object::new::<BlockyPreferencesWindow>(&[]).unwrap()
    }

    fn setup_widgets(&self) {}

    fn setup_signals(&self) {
        let imp = imp::BlockyPreferencesWindow::from_instance(self);

        // Launcher Page
        // Default instance dir
        settings::bind_property(
            SettingKey::DefaultInstancesDir,
            &*imp.default_instances_dir_label,
            "label",
        );
        imp.default_instances_dir_button.connect_clicked(glib::clone!(@weak self as this => move |_| {
            this.folder_chooser(&gettext("Select Instance Location"), SettingKey::DefaultInstancesDir);
        }));
        // Default libraries dir
        settings::bind_property(
            SettingKey::DefaultLibrariesDir,
            &*imp.default_libraries_dir_label,
            "label",
        );
        imp.default_libraries_dir_button.connect_clicked(glib::clone!(@weak self as this => move |_| {
            this.folder_chooser(&gettext("Select Libraries Location"), SettingKey::DefaultLibrariesDir);
        }));
        // Default assets dir
        settings::bind_property(
            SettingKey::DefaultAssetsDir,
            &*imp.default_assets_dir_label,
            "label",
        );
        imp.default_assets_dir_button.connect_clicked(glib::clone!(@weak self as this => move |_| {
            this.folder_chooser(&gettext("Select Assets Location"), SettingKey::DefaultAssetsDir);
        }));

        // Minecraft Page
        // Default fullscreen
        settings::bind_property(
            SettingKey::DefaultFullscreen,
            &*imp.default_fullscreen_switch,
            "state",
        );
        // Default width
        settings::bind_property(
            SettingKey::DefaultWidth,
            &*imp.default_width_spinbutton,
            "value",
        );
        // Default height
        settings::bind_property(
            SettingKey::DefaultHeight,
            &*imp.default_height_spinbutton,
            "value",
        );
        // Default minimum memory
        settings::bind_property(
            SettingKey::DefaultMinMemory,
            &*imp.default_min_memory_spinbutton,
            "value",
        );
        // Default maximum memory
        settings::bind_property(
            SettingKey::DefaultMaxMemory,
            &*imp.default_max_memory_spinbutton,
            "value",
        );
        // Default java exec
        settings::bind_property(
            SettingKey::DefaultJavaExec,
            &*imp.default_java_exec_label,
            "label",
        );
        imp.default_java_exec_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.file_chooser(&gettext("Select Java Executable"), SettingKey::DefaultJavaExec);
            }));
        // Default use jvm args
        settings::bind_property(
            SettingKey::DefaultUseJvmArgs,
            &*imp.default_use_jvm_args_switch,
            "state",
        );
        // Default jvm args
        settings::bind_property(
            SettingKey::DefaultJvmArgs,
            &*imp.default_jvm_args_text,
            "text",
        )
    }

    fn folder_chooser(&self, title: &str, key: SettingKey) {
        let dialog = FileChooserNative::new(
            Some(title),
            Some(self),
            FileChooserAction::SelectFolder,
            Some(&gettext("Select")),
            Some(&gettext("Cancel")),
        );

        dialog.connect_response(
            glib::clone!(@strong dialog, @weak self as this, @strong key => move |_, resp| {
                dialog.destroy();
                if resp == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        if let Some(path) = folder.path() {
                            if let Some(path_string) = path.to_str() {
                                debug!("Selected directory: {}", path_string);
                                settings::set_string(key.clone(), path_string);
                            }
                        }
                    }
                }
            }),
        );

        dialog.show();
    }

    fn file_chooser(&self, title: &str, key: SettingKey) {
        let dialog = FileChooserNative::new(
            Some(title),
            Some(self),
            FileChooserAction::Open,
            Some(&gettext("Select")),
            Some(&gettext("Cancel")),
        );

        dialog.connect_response(
            glib::clone!(@strong dialog, @weak self as this, @strong key => move |_, resp| {
                dialog.destroy();
                if resp == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            if let Some(path_string) = path.to_str() {
                                debug!("Selected file: {}", path_string);
                                settings::set_string(key.clone(), path_string);
                            }
                        }
                    }
                }
            }),
        );

        dialog.show();
    }
}
