use crate::minecraft::models::version_type::VersionType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionSummary {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: VersionType,
    pub url: String,
    pub time: DateTime<Utc>,
    #[serde(alias = "releaseTime")]
    pub release_time: DateTime<Utc>,
}
