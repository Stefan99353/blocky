use crate::managers::BlockyInstanceManager;
use crate::ui::edit_instance_dialog::BlockyEditInstanceDialog;
use crate::ui::{BlockyApplicationWindow, BlockyInstallProgressDialog};
use blocky_core::gobject::GInstance;
use blocky_core::instance::Instance;
use blocky_core::minecraft::installation_update::InstallationUpdate;
use gettextrs::gettext;
use glib::subclass::InitializingObject;
use glib::ToValue;
use glib::{IsA, ObjectExt, ParamSpec, Value};
use glib::{ParamFlags, ParamSpecObject, StaticType};
use gtk::gdk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/instance_row.ui")]
    pub struct BlockyInstanceRow {
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,

        #[template_child]
        pub launch_button: TemplateChild<gtk::Button>,

        pub popover_menu: OnceCell<gtk::PopoverMenu>,
        pub instance: OnceCell<GInstance>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstanceRow {
        const NAME: &'static str = "BlockyInstanceRow";
        type Type = super::BlockyInstanceRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyInstanceRow {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecObject::new(
                    "instance",
                    "Instance",
                    "Instance",
                    GInstance::static_type(),
                    ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                )]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "instance" => self.instance.set(value.get().unwrap()).unwrap(),
                x => {
                    error!("Property {} not a member of BlockyInstanceRow", x);
                    unimplemented!()
                }
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "instance" => self.instance.get().to_value(),
                x => {
                    error!("Property {} not a member of BlockyInstanceRow", x);
                    unimplemented!()
                }
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_signals();

            install_actions(&obj.instance(), obj.clone().upcast());

            self.parent_constructed(obj);
        }

        fn dispose(&self, _obj: &Self::Type) {
            self.popover_menu.get().unwrap().unparent();
        }
    }

    impl WidgetImpl for BlockyInstanceRow {}

    impl ListBoxRowImpl for BlockyInstanceRow {}
}

glib::wrapper! {
    pub struct BlockyInstanceRow(ObjectSubclass<imp::BlockyInstanceRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl BlockyInstanceRow {
    pub fn new(instance: &GInstance) -> Self {
        glib::Object::new(&[("instance", instance)]).unwrap()
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyInstanceRow::from_instance(self);

        self.bind_property("name", &imp.name_label.get(), "label");
        self.bind_property("description", &imp.description_label.get(), "label");
        self.bind_property("version", &imp.version_label.get(), "label");

        // Popover
        let builder = gtk::Builder::from_resource("/at/stefan99353/Blocky/ui/instance_menu.ui");
        let menu: gio::MenuModel = builder.object("instance_menu").unwrap();
        let popover_menu = gtk::PopoverMenu::from_model(Some(&menu));
        popover_menu.set_parent(self);
        imp.popover_menu.set(popover_menu).unwrap();
    }

    fn setup_signals(&self) {
        // Right click menu
        let controller = gtk::GestureClick::new();
        controller.set_button(gdk::BUTTON_SECONDARY);
        controller.connect_pressed(glib::clone!(@weak self as this => move |c, _, x, y| this.show_context_menu(Some(c), x, y)));
        self.add_controller(&controller);
    }

    fn show_context_menu<G>(&self, controller: Option<&G>, x: f64, y: f64)
    where
        G: IsA<gtk::Gesture>,
    {
        let imp = imp::BlockyInstanceRow::from_instance(self);

        if let Some(controller) = controller {
            controller.set_state(gtk::EventSequenceState::Claimed);
        }

        let coordinates = gdk::Rectangle::new(x as i32, y as i32, 0, 0);

        imp.popover_menu
            .get()
            .unwrap()
            .set_pointing_to(Some(&coordinates));
        imp.popover_menu.get().unwrap().popup();
    }

    pub fn instance(&self) -> GInstance {
        self.property("instance")
    }

    fn bind_property<T: IsA<gtk::Widget>>(
        &self,
        prop_name: &str,
        widget: &T,
        widget_prop_name: &str,
    ) {
        self.instance()
            .bind_property(prop_name, widget, widget_prop_name)
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
    }
}

fn install_actions(instance: &GInstance, widget: gtk::Widget) {
    let actions = gio::SimpleActionGroup::new();
    widget.insert_action_group("instance", Some(&actions));
    let window = BlockyApplicationWindow::default();
    let instance_manager = BlockyInstanceManager::default();

    // instance.launch
    let launch_action = gio::SimpleAction::new("launch", None);
    launch_action.connect_activate(
        glib::clone!(@weak instance, @weak instance_manager, @weak window => move |_, _| {
            window.toast_notification(&gettext("Launching instance."));
            instance_manager.launch_instance(instance.uuid());
        }),
    );
    launch_action.set_enabled(false);
    actions.add_action(&launch_action);

    instance_manager.check_instance_installed(instance.uuid()).attach(
        None,
        glib::clone!(@weak launch_action => @default-return glib::Continue(false), move |status| {
            launch_action.set_enabled(status);
            glib::Continue(true)
        }
    ));

    // instance.install
    let install_action = gio::SimpleAction::new("install", None);
    install_action.connect_activate(glib::clone!(@weak instance, @weak launch_action, @weak instance_manager => move |_, _| {
        let dialog = BlockyInstallProgressDialog::new();
        dialog.show();

        let receiver = instance_manager.install_instance(instance.uuid());
        receiver.attach(
            None,
            glib::clone!(@weak dialog, @weak launch_action => @default-return glib::Continue(false), move |update| {
                process_install_update(update, dialog.upcast(), launch_action);
                glib::Continue(true)
            }
        ));
    }));
    actions.add_action(&install_action);

    // instance.edit
    let edit_action = gio::SimpleAction::new("edit", None);
    edit_action.connect_activate(glib::clone!(@weak instance => move |_, _| {
        let instance = Instance::from(instance);
        let dialog = BlockyEditInstanceDialog::new(&instance.into());
        dialog.show();
    }));
    actions.add_action(&edit_action);

    // instance.remove
    let remove_action = gio::SimpleAction::new("remove", None);
    remove_action.connect_activate(
        glib::clone!(@weak instance, @weak instance_manager, @weak window => move |_, _| {
            let uuid = instance.uuid();
            window.toast_notification(&gettext("Instance removed."));
            instance_manager.remove_instance_by_uuid(uuid);
        }),
    );
    actions.add_action(&remove_action);
}

fn process_install_update(
    update: InstallationUpdate,
    dialog: gtk::Dialog,
    launch_action: gio::SimpleAction,
) {
    match update {
        InstallationUpdate::Success => {
            // Update finished
            let window = BlockyApplicationWindow::default();
            launch_action.set_enabled(true);
            dialog.close();
            window.toast_notification(&gettext("Installation finished."));
        }
        InstallationUpdate::Cancel => {
            // Update finished
            dialog.close();
        }
        update => {
            let dialog = dialog.downcast::<BlockyInstallProgressDialog>().unwrap();
            dialog.update_widgets(update);
        }
    }
}
