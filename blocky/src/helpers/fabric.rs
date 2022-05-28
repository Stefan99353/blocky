use crate::helpers::error::error_dialog;
use blocky_core::minecraft::fabric::loader_manifest::FabricLoaderSummary;
use gtk_macros::send;
use std::thread;
use std::thread::JoinHandle;

pub fn get_loader_manifest(
    game_version: String,
) -> (glib::Receiver<Vec<FabricLoaderSummary>>, JoinHandle<()>) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let handle = thread::spawn(move || match FabricLoaderSummary::get(&game_version) {
        Ok(manifest) => {
            send!(sender, manifest);
        }
        Err(err) => {
            error_dialog(err);
            send!(sender, Vec::default());
        }
    });

    (receiver, handle)
}
