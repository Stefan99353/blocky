#[macro_use]
extern crate log;

mod application;
#[rustfmt::skip]
mod config;
mod managers;
mod paths;
mod settings;
mod ui;
pub(crate) mod utils;

use application::BlockyApplication;

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    if cfg!(debug_assertions) {
        warn!("===== This is a debug build =====");
    }

    // Initialize GTK and libadwaita
    gtk::init().expect("Failed to initialize GTK");
    adw::init();

    // Initialize paths (data, config and cache)
    paths::init().expect("Failed to create directories");
    paths::set_defaults();

    // Prepare i18n
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(config::PKG_NAME, config::LOCALEDIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(config::PKG_NAME).expect("Unable to switch to the text domain");

    // Load gresources
    let res = gio::Resource::load(config::RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    glib::set_application_name(config::APP_NAME);

    BlockyApplication::run();
}
