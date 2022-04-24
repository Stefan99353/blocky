use crate::settings;
use crate::settings::SettingKey;
use crate::ui::{BlockyApplicationWindow, BlockyVersionSummaryRow};
use adw::prelude::*;
use gettextrs::gettext;
use gio::ListStore;
use glib::subclass::prelude::*;
use glib::subclass::{InitializingObject, InitializingType, Signal};
use glib::{ParamSpec, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    CompositeTemplate, FileChooserAction, FileChooserNative, Label, ListView, ResponseType,
    SignalListItemFactory, SingleSelection, TemplateChild,
};
use itertools::Itertools;
use libblocky::gobject::{GBlockyInstance, GBlockyVersionSummary};
use libblocky::helpers::HelperError;
use libblocky::instance::models::{VersionSummary, VersionType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use uuid::Uuid;

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/new_instance_dialog.ui")]
    pub struct BlockyNewInstanceDialog {
        #[template_child]
        pub add_button: TemplateChild<gtk::Button>,

        // General
        #[template_child]
        pub name_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub description_entry: TemplateChild<gtk::Entry>,

        // Version
        #[template_child]
        pub version_expander: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub version_list: TemplateChild<gtk::ListView>,
        #[template_child]
        pub releases_filter_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub snapshots_filter_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub betas_filter_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub alphas_filter_switch: TemplateChild<gtk::Switch>,

        // Advanced
        #[template_child]
        pub instance_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub instance_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub libraries_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub libraries_dir_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub assets_dir_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub assets_dir_label: TemplateChild<gtk::Label>,

        pub uuid: Uuid,
        pub manifest: RefCell<HashMap<String, VersionSummary>>,
        pub version_list_store: ListStore,
        pub version_selection_model: SingleSelection,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyNewInstanceDialog {
        const NAME: &'static str = "BlockyNewInstanceDialog";
        type Type = super::BlockyNewInstanceDialog;
        type ParentType = gtk::Dialog;

        fn new() -> Self {
            let list_store = gio::ListStore::new(GBlockyVersionSummary::static_type());

            let selection = SingleSelection::builder()
                .autoselect(false)
                .model(&list_store)
                .build();

            Self {
                add_button: Default::default(),
                name_entry: Default::default(),
                description_entry: Default::default(),
                version_expander: Default::default(),
                version_list: Default::default(),
                releases_filter_switch: Default::default(),
                snapshots_filter_switch: Default::default(),
                betas_filter_switch: Default::default(),
                alphas_filter_switch: Default::default(),
                instance_dir_button: Default::default(),
                instance_dir_label: Default::default(),
                libraries_dir_button: Default::default(),
                libraries_dir_label: Default::default(),
                assets_dir_button: Default::default(),
                assets_dir_label: Default::default(),
                uuid: Uuid::new_v4(),
                manifest: Default::default(),
                version_list_store: list_store,
                version_selection_model: selection,
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyNewInstanceDialog {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_signals();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyNewInstanceDialog {}

    impl WindowImpl for BlockyNewInstanceDialog {}

    impl DialogImpl for BlockyNewInstanceDialog {}
}

glib::wrapper! {
    pub struct BlockyNewInstanceDialog(ObjectSubclass<imp::BlockyNewInstanceDialog>)
    @extends gtk::Widget, gtk::Window, adw::Window, gtk::Dialog;
}

#[gtk::template_callbacks]
impl BlockyNewInstanceDialog {
    #[template_callback]
    fn add_button_clicked(&self) {
        debug!("Add button clicked");
    }
}

impl BlockyNewInstanceDialog {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let dialog: Self = glib::Object::new(&[("use-header-bar", &1)]).unwrap();

        let window = BlockyApplicationWindow::default();
        dialog.set_transient_for(Some(&window));

        dialog
    }

    fn setup_widgets(&self) {
        let imp = imp::BlockyNewInstanceDialog::from_instance(self);

        imp.version_list
            .set_factory(Some(&self.version_list_factory()));
        imp.version_list
            .set_model(Some(&imp.version_selection_model));

        let mut instance_path =
            PathBuf::from(settings::get_string(SettingKey::DefaultInstancesDir));
        instance_path.push(imp.uuid.to_string());
        let libraries_path = settings::get_string(SettingKey::DefaultLibrariesDir);
        let assets_path = settings::get_string(SettingKey::DefaultAssetsDir);

        imp.instance_dir_label
            .set_text(&instance_path.to_string_lossy().to_string());
        imp.libraries_dir_label.set_text(&libraries_path);
        imp.assets_dir_label.set_text(&assets_path);

        self.fetch_manifest();
    }

    fn setup_signals(&self) {
        let imp = imp::BlockyNewInstanceDialog::from_instance(self);

        imp.version_selection_model.connect_selected_notify(
            glib::clone!(@weak self as this => move |selection| {
                let imp = imp::BlockyNewInstanceDialog::from_instance(&this);

                if let Some(item) = selection.selected_item() {
                    let summary = item
                        .downcast::<GBlockyVersionSummary>()
                        .unwrap();

                    imp.version_expander.set_subtitle(&summary.id());
                }else {
                    imp.version_expander.set_subtitle("");
                }
            }),
        );

        // Instance folder
        let (instance_sender, instance_receiver) =
            glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);
        imp.instance_dir_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.folder_chooser(&gettext("Select Instance Folder"), instance_sender.clone());
            }));
        instance_receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |path| {
                    let imp = imp::BlockyNewInstanceDialog::from_instance(&this);
                    imp.instance_dir_label.set_text(&path);
                    Continue(true)
                }
            ),
        );

        // Libraries folder
        let (libraries_sender, libraries_receiver) =
            glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);
        imp.libraries_dir_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.folder_chooser(&gettext("Select Libraries Folder"), libraries_sender.clone());
            }));
        libraries_receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |path| {
                    let imp = imp::BlockyNewInstanceDialog::from_instance(&this);
                    imp.libraries_dir_label.set_text(&path);
                    Continue(true)
                }
            ),
        );

        // Assets folder
        let (assets_sender, assets_receiver) =
            glib::MainContext::channel::<String>(glib::PRIORITY_DEFAULT);
        imp.assets_dir_button
            .connect_clicked(glib::clone!(@weak self as this => move |_| {
                this.folder_chooser(&gettext("Select Assets Folder"), assets_sender.clone());
            }));
        assets_receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |path| {
                    let imp = imp::BlockyNewInstanceDialog::from_instance(&this);
                    imp.assets_dir_label.set_text(&path);
                    Continue(true)
                }
            ),
        );

        imp.releases_filter_switch.connect_state_notify(
            glib::clone!(@weak self as this => move |_| {
                this.refresh_version_list();
            }),
        );
        imp.snapshots_filter_switch.connect_state_notify(
            glib::clone!(@weak self as this => move |_| {
                this.refresh_version_list();
            }),
        );
        imp.betas_filter_switch
            .connect_state_notify(glib::clone!(@weak self as this => move |_| {
                this.refresh_version_list();
            }));
        imp.alphas_filter_switch.connect_state_notify(
            glib::clone!(@weak self as this => move |_| {
                this.refresh_version_list();
            }),
        );
    }

    fn fetch_manifest(&self) {
        // Get Version Manifest
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || match libblocky::helpers::get_manifest() {
            Ok(manifest) => {
                sender
                    .send(manifest)
                    .expect("Could not send version manifest through channel");
            }
            Err(err) => {
                error!("Error while getting version manifest: {}", err);
                sender
                    .send(HashMap::new())
                    .expect("Could not send version manifest through channel");
            }
        });

        receiver.attach(
            None,
            glib::clone!(@weak self as this => @default-return glib::Continue(false),
                move |manifest| {
                    let imp = imp::BlockyNewInstanceDialog::from_instance(&this);
                    *imp.manifest.borrow_mut() = manifest;
                    this.refresh_version_list();
                    glib::Continue(true)
                }
            ),
        );
    }

    fn filter_versions(&self) -> Vec<VersionSummary> {
        let imp = imp::BlockyNewInstanceDialog::from_instance(self);

        let show_releases = imp.releases_filter_switch.state();
        let show_snapshots = imp.snapshots_filter_switch.state();
        let show_betas = imp.betas_filter_switch.state();
        let show_alphas = imp.alphas_filter_switch.state();

        let versions = imp
            .manifest
            .borrow()
            .iter()
            .filter(|(key, summary)| {
                (matches!(summary._type, VersionType::Release) && show_releases)
                    || (matches!(summary._type, VersionType::Snapshot) && show_snapshots)
                    || (matches!(summary._type, VersionType::OldBeta) && show_betas)
                    || (matches!(summary._type, VersionType::OldAlpha) && show_alphas)
            })
            .sorted_by(|(_, a), (_, b)| Ord::cmp(&b.release_time, &a.release_time))
            .map(|(_, summary)| summary.clone())
            .collect();

        versions
    }

    fn version_list_factory(&self) -> SignalListItemFactory {
        let factory = SignalListItemFactory::new();

        // Bind
        factory.connect_bind(move |_, list_item| {
            let version_summary = list_item
                .item()
                .unwrap()
                .downcast::<GBlockyVersionSummary>()
                .unwrap();

            let row = BlockyVersionSummaryRow::new(&version_summary);
            list_item.set_child(Some(&row));
        });

        factory
    }

    fn refresh_version_list(&self) {
        let imp = imp::BlockyNewInstanceDialog::from_instance(self);

        let versions = self
            .filter_versions()
            .into_iter()
            .map(GBlockyVersionSummary::from)
            .collect::<Vec<GBlockyVersionSummary>>();

        imp.version_list_store
            .splice(0, imp.version_list_store.n_items(), &versions);
        if !versions.is_empty() {
            imp.version_selection_model.set_selected(0);
        }

        if let Some(selected) = imp.version_selection_model.selected_item() {
            let summary = selected.downcast::<GBlockyVersionSummary>().unwrap();

            imp.version_expander.set_subtitle(&summary.id());
        }
    }

    fn folder_chooser(&self, title: &str, sender: glib::Sender<String>) {
        let dialog = FileChooserNative::new(
            Some(title),
            Some(self),
            FileChooserAction::SelectFolder,
            Some(&gettext("Select")),
            Some(&gettext("Cancel")),
        );

        dialog.connect_response(
            glib::clone!(@strong dialog, @weak self as this => move |_, resp| {
                dialog.destroy();
                if resp == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        if let Some(path) = folder.path() {
                            if let Some(path_string) = path.to_str() {
                                debug!("Selected directory: {}", path_string);
                                sender.send(path_string.to_string()).expect("Could not send path through channel");
                            }
                        }
                    }
                }
            }),
        );

        dialog.show();
    }
}
