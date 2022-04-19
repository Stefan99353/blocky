use crate::settings::SettingKey;
use crate::ui::BlockyContentBox;
use crate::{config, settings, BlockyApplication};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, TemplateChild};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/window.ui")]
    pub struct BlockyApplicationWindow {
        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub profile_combo_box: TemplateChild<gtk::ComboBoxText>,
        #[template_child]
        pub app_menu_button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub content_box: TemplateChild<BlockyContentBox>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyApplicationWindow {
        const NAME: &'static str = "BlockyApplicationWindow";
        type Type = super::BlockyApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            // Devel Profile
            if config::PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyApplicationWindow {}

    impl WindowImpl for BlockyApplicationWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            window.save_window_size();

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for BlockyApplicationWindow {}

    impl AdwApplicationWindowImpl for BlockyApplicationWindow {}
}

glib::wrapper! {
    pub struct BlockyApplicationWindow(ObjectSubclass<imp::BlockyApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl BlockyApplicationWindow {
    pub fn new(app: &BlockyApplication) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create ExampleApplicationWindow")
    }

    fn save_window_size(&self) {
        debug!("Saving window state");
        let (width, height) = self.default_size();

        settings::set_integer(SettingKey::WindowWidth, width);
        settings::set_integer(SettingKey::WindowHeight, height);

        let is_maximized = self.is_maximized();
        settings::set_bool(SettingKey::IsMaximized, is_maximized);
    }

    fn load_window_size(&self) {
        debug!("Loading window state");
        let width = settings::get_integer(SettingKey::WindowWidth);
        let height = settings::get_integer(SettingKey::WindowHeight);
        self.set_default_size(width, height);

        if settings::get_bool(SettingKey::IsMaximized) {
            self.maximize();
        }
    }
}

impl Default for BlockyApplicationWindow {
    fn default() -> Self {
        BlockyApplication::default()
            .active_window()
            .unwrap()
            .downcast()
            .unwrap()
    }
}
