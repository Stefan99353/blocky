#[derive(Clone, Debug)]
pub struct ResourceInstallationUpdate {
    pub resource_type: ResourceType,
    pub url: String,
    pub total: usize,
    pub n: usize,
    pub size: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum ResourceType {
    VersionManifest,
    VersionData,
    Library,
    AssetIndex,
    Asset,
    LogConfig,
    Client,
}
