use crate::config;
use crate::ui::BlockyApplicationWindow;
use gettextrs::gettext;
use gtk::traits::GtkWindowExt;

pub fn show_about_dialog() {
    let main_window = BlockyApplicationWindow::default();
    let comments = gettext("Minecraft Launcher with support for multiple instances");

    let dialog = gtk::AboutDialog::builder()
        .program_name(config::APP_NAME)
        .logo_icon_name(config::APP_ID)
        .license_type(gtk::License::MitX11)
        .comments(&comments)
        .copyright("© 2022 Stefan Rupertsberger")
        // Insert your website here
        // .website("https://gitlab.gnome.org/bilelmoussaoui/blocky/")
        .version(config::VERSION)
        .transient_for(&main_window)
        .translator_credits(&gettext("translator-credits"))
        .modal(true)
        .authors(vec!["Stefan Rupertsberger".into()])
        .artists(vec!["Stefan Rupertsberger".into()])
        .build();

    dialog.present();
}
