use crate::managers::BlockyProfileManager;
use crate::ui::{BlockyApplicationWindow, BlockyNewProfileWindow, BlockyPreferencesWindow};
use crate::{config, settings};
use adw::subclass::prelude::*;
use gio::subclass::prelude::{ApplicationImpl, ApplicationImplExt};
use glib::subclass::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value, WeakRef};
use gtk::gdk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::{Lazy, OnceCell};

mod imp {
    use super::*;

    pub struct BlockyApplication {
        pub profile_manager: BlockyProfileManager,

        pub window: OnceCell<WeakRef<BlockyApplicationWindow>>,
        pub settings: gio::Settings,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyApplication {
        const NAME: &'static str = "BlockyApplication";
        type Type = super::BlockyApplication;
        type ParentType = gtk::Application;

        fn new() -> Self {
            let profile_manager = BlockyProfileManager::new();

            Self {
                profile_manager,
                window: OnceCell::new(),
                settings: settings::get_settings(),
            }
        }
    }

    impl ObjectImpl for BlockyApplication {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::new(
                    "profile-manager",
                    "Profile Manager",
                    "Profile Manager",
                    BlockyProfileManager::static_type(),
                    ParamFlags::READABLE,
                )]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "profile-manager" => self.profile_manager.to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl ApplicationImpl for BlockyApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("Activate Application");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                debug!("Application window presented");
                return;
            }

            debug!("Initializing managers");
            app.init_profile_manager();

            debug!("Create new application window");
            let window = app.create_window();
            window.present();
            self.window
                .set(window.downgrade())
                .expect("Window already set.");
        }

        fn startup(&self, app: &Self::Type) {
            debug!("Start Application");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(config::APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for BlockyApplication {}

    impl AdwApplicationImpl for BlockyApplication {}
}

glib::wrapper! {
    pub struct BlockyApplication(ObjectSubclass<imp::BlockyApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl BlockyApplication {
    pub fn run() {
        info!("{} ({})", config::APP_NAME, config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKG_DATADIR);

        let app = glib::Object::new::<BlockyApplication>(&[
            ("application-id", &Some(config::APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/at/stefan99353/Blocky/")),
        ])
        .expect("Failed to initialize app");

        app.set_default();

        // Start Application
        ApplicationExtManual::run(&app);
    }

    fn create_window(&self) -> BlockyApplicationWindow {
        BlockyApplicationWindow::new(self)
    }

    fn setup_gactions(&self) {
        // app.add-instance
        let action_add_instance = gio::SimpleAction::new("add-instance", None);
        action_add_instance.connect_activate(move |_, _| {
            debug!("Show add-instance window");
        });
        self.add_action(&action_add_instance);

        // app.add-profile
        let action_add_profile = gio::SimpleAction::new("add-profile", None);
        action_add_profile.connect_activate(move |_, _| {
            debug!("Show add-profile window");
            BlockyNewProfileWindow::default().show();
        });
        self.add_action(&action_add_profile);

        // app.preferences
        let action_preferences = gio::SimpleAction::new("preferences", None);
        action_preferences.connect_activate(move |_, _| {
            debug!("Show preferences window");
            BlockyPreferencesWindow::default().show();
        });
        self.add_action(&action_preferences);

        // app.about
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(move |_, _| {
            debug!("Show about dialog");
            crate::ui::about::show_about_dialog();
        });
        self.add_action(&action_about);

        // app.quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(move |_, _| {
            debug!("Closing application");
            let window = BlockyApplicationWindow::default();
            window.close();
        });
        self.add_action(&action_quit);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.add-instance", &["<primary>plus"]);
        self.set_accels_for_action("app.preferences", &["<primary>comma"]);
        self.set_accels_for_action("app.quit", &["<primary>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/at/stefan99353/Blocky/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn profile_manager(&self) -> BlockyProfileManager {
        self.property("profile-manager")
    }

    fn init_profile_manager(&self) {
        let profile_manager = BlockyProfileManager::default();
        profile_manager.initialize()
    }
}

impl Default for BlockyApplication {
    fn default() -> Self {
        gio::Application::default()
            .expect("Could not get default application")
            .downcast()
            .unwrap()
    }
}
