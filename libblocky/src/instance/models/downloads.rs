use super::File;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Downloads {
    pub client: File,
    pub client_mappings: File,
    pub server: File,
    pub server_mappings: File,
}
