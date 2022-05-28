use blocky_core::gobject::fabric::fabric_loader_summary::GFabricLoaderSummary;
use glib::Cast;
use gtk::SignalListItemFactory;

pub fn fabric_loader_list_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    // Bind
    factory.connect_bind(move |_, list_item| {
        let summary = list_item
            .item()
            .unwrap()
            .downcast::<GFabricLoaderSummary>()
            .unwrap();

        let text = match summary.stable() {
            true => format!("★ {} ★", &summary.version()),
            false => format!("{}", &summary.version()),
        };
        let label = gtk::Label::new(Some(&text));
        list_item.set_child(Some(&label));
    });

    factory
}
