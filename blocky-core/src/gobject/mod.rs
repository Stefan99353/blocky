#[cfg(feature = "fabric")]
pub mod fabric;
pub mod instance;
pub mod profile;
pub mod version_summary;

pub use instance::GInstance;
pub use profile::GProfile;
pub use version_summary::GVersionSummary;
