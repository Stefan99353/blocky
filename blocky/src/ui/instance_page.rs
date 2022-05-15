use crate::managers::BlockyInstanceManager;
use crate::ui::BlockyInstanceGroup;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/instance_page.ui")]
    pub struct BlockyInstancePage {}

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstancePage {
        const NAME: &'static str = "BlockyInstancePage";
        type Type = super::BlockyInstancePage;
        type ParentType = adw::PreferencesPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass)
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyInstancePage {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();
            obj.setup_signals();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyInstancePage {}

    impl PreferencesPageImpl for BlockyInstancePage {}
}

glib::wrapper! {
    pub struct BlockyInstancePage(ObjectSubclass<imp::BlockyInstancePage>)
        @extends gtk::Widget, adw::PreferencesPage;
}

impl BlockyInstancePage {
    fn setup_widgets(&self) {
        let instance_manager = BlockyInstanceManager::default();

        let default_group = BlockyInstanceGroup::new();
        default_group.set_model(instance_manager.instances());

        self.add(&default_group);
    }

    fn setup_signals(&self) {}
}
