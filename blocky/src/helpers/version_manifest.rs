use crate::helpers::error::error_dialog;
use blocky_core::minecraft::models::version_manifest::VersionManifest;
use blocky_core::minecraft::models::version_summary::VersionSummary;
use gtk_macros::send;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

pub fn get_manifest() -> (
    glib::Receiver<HashMap<String, VersionSummary>>,
    JoinHandle<()>,
) {
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let handle = thread::spawn(move || match VersionManifest::get() {
        Ok(manifest) => {
            send!(sender, manifest.versions);
        }
        Err(err) => {
            error_dialog(err);
            send!(sender, HashMap::default());
        }
    });

    (receiver, handle)
}
