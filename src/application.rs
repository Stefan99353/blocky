use gettextrs::gettext;

use crate::config;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gdk, gio, glib};
use glib::clone;
use gtk::subclass::application::GtkApplicationImpl;

use crate::window::ExampleApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    pub struct BlockyApplication {
        pub window: OnceCell<WeakRef<ExampleApplicationWindow>>,
        pub settings: gio::Settings,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyApplication {
        const NAME: &'static str = "BlockyApplication";
        type Type = super::BlockyApplication;
        type ParentType = gtk::Application;

        fn new() -> Self {
            Self {
                window: OnceCell::new(),
                settings: gio::Settings::new(config::APP_ID),
            }
        }
    }

    impl ObjectImpl for BlockyApplication {}

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
        @extends gio::Application, gtk::Application,
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

    fn create_window(&self) -> ExampleApplicationWindow {
        ExampleApplicationWindow::new(&self)
    }

    fn main_window(&self) -> ExampleApplicationWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
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

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(config::APP_ID)
            .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/blocky/")
            .version(config::VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Stefan Rupertsberger".into()])
            .artists(vec!["Stefan Rupertsberger".into()])
            .build();

        dialog.present();
    }
}
