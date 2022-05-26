use crate::managers::BlockyInstanceManager;
use crate::ui::BlockyApplicationWindow;
use blocky_core::minecraft::installation_update::InstallationUpdate;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::str::FromStr;
use uuid::Uuid;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/install_progress_dialog.ui")]
    pub struct BlockyInstallProgressDialog {
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub status_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstallProgressDialog {
        const NAME: &'static str = "BlockyInstallProgressDialog";
        type Type = super::BlockyInstallProgressDialog;
        type ParentType = gtk::Dialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyInstallProgressDialog {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_signals();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyInstallProgressDialog {}

    impl WindowImpl for BlockyInstallProgressDialog {}

    impl DialogImpl for BlockyInstallProgressDialog {}
}

glib::wrapper! {
    pub struct BlockyInstallProgressDialog(ObjectSubclass<imp::BlockyInstallProgressDialog>)
    @extends gtk::Widget, gtk::Window, adw::Window, gtk::Dialog;
}

impl BlockyInstallProgressDialog {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let dialog: Self = glib::Object::new(&[("use-header-bar", &1)]).unwrap();
        let window = BlockyApplicationWindow::default();
        dialog.set_transient_for(Some(&window));
        dialog
    }

    fn setup_widgets(&self) {}

    fn setup_signals(&self) {
        let actions = gio::SimpleActionGroup::new();
        self.insert_action_group("install", Some(&actions));

        // install.cancel
        let cancel_action = gio::SimpleAction::new("cancel", None);
        cancel_action.connect_activate(glib::clone!(@weak self as this => move |_, _| {
            let instance_manager = BlockyInstanceManager::default();
            instance_manager.cancel_current_installation();
            this.close();
        }));
        cancel_action.set_enabled(true);
        actions.add_action(&cancel_action);
    }

    pub fn update_widgets(&self, update: InstallationUpdate) {
        let imp = imp::BlockyInstallProgressDialog::from_instance(self);

        let progress = match &update {
            InstallationUpdate::Library(progress) => progress,
            InstallationUpdate::Asset(progress) => progress,
            InstallationUpdate::LogConfig(progress) => progress,
            InstallationUpdate::Client(progress) => progress,
            InstallationUpdate::FabricLibrary(progress) => progress,
            _ => unreachable!("Unknown resource installed"),
        };

        let fraction = progress.current_file as f64 / progress.total_files as f64;
        imp.progress_bar.set_fraction(fraction);
        imp.spinner.start();

        imp.status_label.set_label(&format!(
            "Installing {}: {} of {}",
            &update.resource_type(),
            progress.current_file,
            progress.total_files
        ));
    }

    pub fn uuid(&self) -> Uuid {
        let uuid = self.property::<String>("uuid");
        Uuid::from_str(&uuid).unwrap()
    }
}
