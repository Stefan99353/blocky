#[derive(Clone, Debug)]
pub enum InstallationUpdate {
    Library(Progress),
    Asset(Progress),
    LogConfig(Progress),
    Client(Progress),
    Cancel,
    Success,
    #[cfg(feature = "fabric")]
    FabricLibrary(Progress),
}

impl InstallationUpdate {
    pub fn resource_type(&self) -> String {
        match self {
            InstallationUpdate::Library(_) => "Library".to_string(),
            InstallationUpdate::Asset(_) => "Asset".to_string(),
            InstallationUpdate::LogConfig(_) => "LogConfig".to_string(),
            InstallationUpdate::Client(_) => "Client".to_string(),
            #[cfg(feature = "fabric")]
            InstallationUpdate::FabricLibrary(_) => "FabricLibrary".to_string(),
            _ => "Other".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Progress {
    pub total_files: usize,
    pub current_file: usize,
    pub current_file_url: String,
    pub current_file_size: Option<usize>,
}
