use super::error::AuthenticationError;
use crate::consts;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entitlements {
    pub items: Vec<Entitlement>,
    pub signature: String,
    pub key_id: String,
}

impl Entitlements {
    pub fn get_entitlements(minecraft_token: &str) -> Result<Self, AuthenticationError> {
        let http_client = reqwest::blocking::Client::new();
        let response = http_client
            .get(consts::MC_ENTITLEMENTS_URL)
            .bearer_auth(minecraft_token)
            .send()?
            .error_for_status()?
            .json::<EntitlementsResponse>()?;

        Ok(response.into())
    }

    pub fn owns_minecraft(&self) -> bool {
        !self.items.is_empty()
    }
}

impl From<EntitlementsResponse> for Entitlements {
    fn from(resp: EntitlementsResponse) -> Self {
        Self {
            items: resp.items,
            signature: resp.signature,
            key_id: resp.key_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entitlement {
    pub name: String,
    pub signature: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntitlementsResponse {
    items: Vec<Entitlement>,
    signature: String,
    key_id: String,
}
