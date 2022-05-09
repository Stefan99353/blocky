use crate::settings;
use crate::settings::SettingKey;
use crate::ui::BlockyApplicationWindow;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, FileChooserAction, FileChooserNative, ResponseType, TemplateChild};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/preferences_window.ui")]
    pub struct BlockyPreferencesWindow {
        // Launcher Page
        #[template_child]
        pub instances_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub instances_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub libraries_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub libraries_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub assets_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub assets_dir_label: TemplateChild<gtk::Label>,

        // Minecraft Page
        #[template_child]
        pub fullscreen_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub enable_window_size_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub window_width_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub window_height_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub enable_memory_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub min_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub max_memory_spinbutton: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub java_exec_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub java_exec_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub enable_jvm_args_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub jvm_args_entry: TemplateChild<gtk::Entry>,
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
    fn setup_widgets(&self) {}

    fn setup_signals(&self) {
        let imp = imp::BlockyPreferencesWindow::from_instance(self);

        // Launcher Page
        // Default instance dir
        settings::bind_property(SettingKey::InstancesDir, &*imp.instances_dir_label, "label");
        imp.instances_dir_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.folder_chooser(&gettext("Select Instance Location"), SettingKey::InstancesDir);
            }));
        // Default libraries dir
        settings::bind_property(SettingKey::LibrariesDir, &*imp.libraries_dir_label, "label");
        imp.libraries_dir_button.connect_clicked(glib::clone!(@weak self as this => move |_| {
            this.folder_chooser(&gettext("Select Libraries Location"), SettingKey::LibrariesDir);
        }));
        // Default assets dir
        settings::bind_property(SettingKey::AssetsDir, &*imp.assets_dir_label, "label");
        imp.assets_dir_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.folder_chooser(&gettext("Select Assets Location"), SettingKey::AssetsDir);
            }));

        // Minecraft Page
        // Fullscreen
        settings::bind_property(SettingKey::UseFullscreen, &*imp.fullscreen_switch, "state");
        // Enable window size
        settings::bind_property(
            SettingKey::EnableWindowSize,
            &*imp.enable_window_size_expander,
            "enable-expansion",
        );
        // Width
        settings::bind_property(
            SettingKey::GameWindowWidth,
            &*imp.window_width_spinbutton,
            "value",
        );
        // Height
        settings::bind_property(
            SettingKey::GameWindowHeight,
            &*imp.window_height_spinbutton,
            "value",
        );
        // Enable memory
        settings::bind_property(
            SettingKey::EnableMemory,
            &*imp.enable_memory_expander,
            "enable-expansion",
        );
        // Minimum memory
        settings::bind_property(SettingKey::MinMemory, &*imp.min_memory_spinbutton, "value");
        // Maximum memory
        settings::bind_property(SettingKey::MaxMemory, &*imp.max_memory_spinbutton, "value");
        // Java exec
        settings::bind_property(SettingKey::JavaExec, &*imp.java_exec_label, "label");
        imp.java_exec_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.file_chooser(&gettext("Select Java Executable"), SettingKey::JavaExec);
            }));
        // Enable JVM args
        settings::bind_property(
            SettingKey::EnableJvmArgs,
            &*imp.enable_jvm_args_expander,
            "enable-expansion",
        );
        // JVM args
        settings::bind_property(SettingKey::JvmArgs, &*imp.jvm_args_entry, "text")
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

impl Default for BlockyPreferencesWindow {
    fn default() -> Self {
        glib::Object::new::<BlockyPreferencesWindow>(&[]).unwrap()
    }
}
