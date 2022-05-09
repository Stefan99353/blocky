use crate::ui::BlockyInstanceRow;
use adw::subclass::prelude::*;
use gio::ListStore;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{CompositeTemplate, TemplateChild};
use libblocky::gobject::GBlockyInstance;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/instance_group.ui")]
    pub struct BlockyInstanceGroup {
        #[template_child]
        pub listbox: TemplateChild<gtk::ListBox>,

        pub model: RefCell<Option<ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyInstanceGroup {
        const NAME: &'static str = "BlockyInstanceGroup";
        type Type = super::BlockyInstanceGroup;
        type ParentType = adw::PreferencesGroup;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyInstanceGroup {}

    impl WidgetImpl for BlockyInstanceGroup {}

    impl PreferencesGroupImpl for BlockyInstanceGroup {}
}

glib::wrapper! {
    pub struct BlockyInstanceGroup(ObjectSubclass<imp::BlockyInstanceGroup>)
        @extends gtk::Widget, adw::PreferencesGroup;
}

impl BlockyInstanceGroup {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn set_model(&self, model: ListStore) {
        let imp = imp::BlockyInstanceGroup::from_instance(self);

        imp.listbox.bind_model(Some(&model), |object| {
            let instance = object.downcast_ref::<GBlockyInstance>().unwrap();
            BlockyInstanceRow::new(instance).upcast::<gtk::Widget>()
        });

        imp.listbox.connect_row_activated(move |_, row| {
            row.activate_action("instance.edit", None)
                .expect("Failed to activate action on instance row");
        });

        *imp.model.borrow_mut() = Some(model);
    }
}
