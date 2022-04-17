use crate::config;
use adw::glib;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/at/stefan99353/Blocky/ui/content_box.ui")]
    pub struct BlockyContentBox {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub ready_status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BlockyContentBox {
        const NAME: &'static str = "BlockyContentBox";
        type ParentType = gtk::Box;
        type Type = super::BlockyContentBox;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BlockyContentBox {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_widgets();

            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for BlockyContentBox {}
    impl BoxImpl for BlockyContentBox {}
}

glib::wrapper! {
    pub struct BlockyContentBox(ObjectSubclass<imp::BlockyContentBox>)
        @extends gtk::Widget, gtk::Box;
}

impl BlockyContentBox {
    fn setup_widgets(&self) {
        let imp = imp::BlockyContentBox::from_instance(self);

        // Set icon on ready page
        imp.ready_status_page.set_icon_name(Some(config::APP_ID));

        // Init View
        self.set_view(View::Ready)
    }

    fn set_view(&self, view: View) {
        let imp = imp::BlockyContentBox::from_instance(self);
        imp.spinner.set_spinning(false);

        // Setup for view
        match view {
            View::Ready => {}
            View::Instances => {}
            View::Loading => {
                imp.spinner.set_spinning(true);
            }
        }

        imp.stack.set_visible_child_name(view.get_name())
    }
}

enum View {
    Ready,
    Instances,
    Loading,
}

impl View {
    pub fn get_name(&self) -> &'static str {
        match self {
            View::Ready => "ready",
            View::Instances => "instances",
            View::Loading => "loading",
        }
    }
}