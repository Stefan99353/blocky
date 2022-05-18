use crate::ui::BlockyVersionSummaryRow;
use blocky_core::gobject::GVersionSummary;
use blocky_core::minecraft::models::version_summary::VersionSummary;
use blocky_core::minecraft::models::version_type::VersionType;
use glib::Cast;
use gtk::SignalListItemFactory;
use itertools::Itertools;
use std::collections::HashMap;

pub fn version_list_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    // Bind
    factory.connect_bind(move |_, list_item| {
        let version_summary = list_item
            .item()
            .unwrap()
            .downcast::<GVersionSummary>()
            .unwrap();

        let row = BlockyVersionSummaryRow::new(&version_summary);
        list_item.set_child(Some(&row));
    });

    factory
}

pub fn filter_versions(
    manifest: &HashMap<String, VersionSummary>,
    releases: bool,
    snapshots: bool,
    betas: bool,
    alphas: bool,
) -> Vec<VersionSummary> {
    let versions = manifest
        .iter()
        .filter(|(_key, summary)| {
            (matches!(summary._type, VersionType::Release) && releases)
                || (matches!(summary._type, VersionType::Snapshot) && snapshots)
                || (matches!(summary._type, VersionType::OldBeta) && betas)
                || (matches!(summary._type, VersionType::OldAlpha) && alphas)
        })
        .sorted_by(|(_, a), (_, b)| Ord::cmp(&b.release_time, &a.release_time))
        .map(|(_, summary)| summary.clone())
        .collect();

    versions
}
