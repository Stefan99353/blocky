use crate::config;
use crate::managers::BlockyProfileManager;
use crate::ui::BlockyApplicationWindow;
use crate::utils::update;
use blocky_core::profile::Profile;
use gettextrs::gettext;
use glib::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, TemplateChild};
use std::thread;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/new_profile_dialog.ui")]
    pub struct BlockyNewProfileDialog {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub start_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub status_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyNewProfileDialog {
        const NAME: &'static str = "BlockyNewProfileDialog";
        type Type = super::BlockyNewProfileDialog;
        type ParentType = gtk::Dialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyNewProfileDialog {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyNewProfileDialog {}

    impl WindowImpl for BlockyNewProfileDialog {}

    impl DialogImpl for BlockyNewProfileDialog {}
}

glib::wrapper! {
    pub struct BlockyNewProfileDialog(ObjectSubclass<imp::BlockyNewProfileDialog>)
    @extends gtk::Widget, gtk::Window, adw::Window, gtk::Dialog;
}

#[gtk::template_callbacks]
impl BlockyNewProfileDialog {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let dialog: Self = glib::Object::new(&[("use-header-bar", &1)]).unwrap();

        let window = BlockyApplicationWindow::default();
        dialog.set_transient_for(Some(&window));

        dialog
    }

    #[template_callback]
    fn start_button_clicked(&self) {
        debug!("Start authentication sequence");
        self.set_view(View::Loading);

        let (sender, receiver) = glib::MainContext::channel::<update::StatusUpdate<String, Profile>>(
            glib::PRIORITY_DEFAULT,
        );

        thread::spawn(move || {
            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Authenticating at Microsoft",
                )))
                .expect("Could not send through channel");
            let mut profile =
                Profile::authenticate_microsoft(config::MS_GRAPH_ID, config::MS_GRAPH_SECRET)
                    .unwrap();

            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Authenticating at XBox Live",
                )))
                .expect("Could not send through channel");
            profile.authenticate_xbox_live().unwrap();

            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Authenticating at XBox Live Security",
                )))
                .expect("Could not send through channel");
            profile.authenticate_xbox_live_security().unwrap();

            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Authenticating at Minecraft",
                )))
                .expect("Could not send through channel");
            profile.authenticate_minecraft().unwrap();

            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Getting Minecraft entitlements",
                )))
                .expect("Could not send through channel");
            profile.set_entitlements().unwrap();

            sender
                .send(update::StatusUpdate::Update(gettext(
                    "Getting Minecraft profile",
                )))
                .expect("Could not send through channel");
            profile.set_profile().unwrap();

            sender
                .send(update::StatusUpdate::Finish(profile))
                .expect("Could not send through channel");
        });

        receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false), move |update| {
                let imp = imp::BlockyNewProfileDialog::from_instance(&this);

                match update {
                    update::StatusUpdate::Update(msg) => {
                        imp.spinner.set_spinning(true);
                        imp.status_label.set_label(&msg);
                    }
                    update::StatusUpdate::Finish(profile) => {
                        this.add_profile(profile);
                    },
                }

                glib::Continue(true)
            }),
        );
    }

    fn add_profile(&self, profile: Profile) {
        let profile_manager = BlockyProfileManager::default();
        profile_manager.add_profile(profile);
        self.close();
    }

    fn setup_widgets(&self) {
        // Init View
        self.set_view(View::Ready)
    }

    fn set_view(&self, view: View) {
        let imp = imp::BlockyNewProfileDialog::from_instance(self);
        imp.spinner.set_spinning(false);

        match view {
            View::Ready => {}
            View::Loading => {
                imp.spinner.set_spinning(true);
                imp.start_button.set_sensitive(false);
            }
        }

        imp.stack.set_visible_child_name(view.get_name())
    }
}

enum View {
    Ready,
    Loading,
}

impl View {
    pub fn get_name(&self) -> &'static str {
        match self {
            View::Ready => "ready",
            View::Loading => "loading",
        }
    }
}
