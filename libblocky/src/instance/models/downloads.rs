use super::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Downloads {
    pub client: File,
    pub client_mappings: Option<File>,
    pub server: Option<File>,
    pub server_mappings: Option<File>,
}
