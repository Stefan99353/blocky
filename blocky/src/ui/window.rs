use crate::managers::BlockyProfileManager;
use crate::settings::SettingKey;
use crate::ui::BlockyContentBox;
use crate::ui::BlockyInstancePage;
use crate::{config, settings, BlockyApplication};
use adw::subclass::prelude::*;
use blocky_core::gobject::GProfile;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, TemplateChild};
use std::str::FromStr;
use uuid::Uuid;

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
            BlockyInstancePage::static_type();
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

            // Update Profiles
            obj.setup_signals();
            obj.update_profiles();

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
            .expect("Failed to create BlockyApplicationWindow")
    }

    fn setup_signals(&self) {
        let imp = imp::BlockyApplicationWindow::from_instance(self);
        let profile_manager = BlockyProfileManager::default();

        // Current profile changed
        profile_manager.connect_notify_local(
            Some("current-profile"),
            glib::clone!(@weak self as this => move |_, _| {
                this.update_current_profile();
            }),
        );

        // Profile List changed
        profile_manager.profiles().connect_items_changed(
            glib::clone!(@weak self as this => move |_,_,_,_| {
                this.update_profiles();
            }),
        );

        imp.profile_combo_box.connect_changed(move |combobox| {
            if let Some(uuid) = combobox.active_id() {
                let uuid = Uuid::from_str(&uuid).unwrap();
                let profile_manager = BlockyProfileManager::default();
                profile_manager.set_current_profile_by_uuid(uuid);
            }
        });
    }

    fn update_current_profile(&self) {
        let imp = imp::BlockyApplicationWindow::from_instance(self);
        let profile_manager = BlockyProfileManager::default();

        let new_current = profile_manager
            .current_profile()
            .map(|p| p.uuid().to_string());
        let old_current = imp.profile_combo_box.active_id().map(|p| p.to_string());

        if new_current != old_current {
            // Update ComboBoxText
            match new_current {
                None => {
                    imp.profile_combo_box.set_active_id(None);
                }
                Some(id) => {
                    imp.profile_combo_box.set_active_id(Some(&id));
                }
            }
        }
    }

    fn update_profiles(&self) {
        let imp = imp::BlockyApplicationWindow::from_instance(self);
        let profile_manager = BlockyProfileManager::default();

        let profiles = profile_manager.profiles();
        imp.profile_combo_box.remove_all();

        for pos in 0..profiles.n_items() {
            let profile = profiles.item(pos).unwrap().downcast::<GProfile>().unwrap();

            let uuid = profile.uuid().to_string();
            let username = profile.username();

            imp.profile_combo_box.append(Some(&uuid), &username);
        }

        self.update_current_profile();
    }

    pub fn toast_notification(&self, msg: &str) {
        let imp = imp::BlockyApplicationWindow::from_instance(self);
        let toast = adw::Toast::new(msg);
        imp.toast_overlay.add_toast(&toast);
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
