use super::error::{AuthenticationError, TokenKind};
use crate::consts;
use crate::consts::MC_PROFILE_URL;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinecraftProfile {
    id: String,
    name: String,
    capes: Vec<serde_json::Value>,
    skins: Vec<serde_json::Value>,
}

impl MinecraftProfile {
    pub fn get_profile(minecraft_token: &str) -> Result<Self, AuthenticationError> {
        let http_client = reqwest::blocking::Client::new();
        let response = http_client
            .get(MC_PROFILE_URL)
            .bearer_auth(minecraft_token)
            .send()?
            .error_for_status()?
            .json::<Self>()?;

        Ok(response)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinecraftToken {
    pub username: String,
    pub token: String,
    pub exp: Option<DateTime<Utc>>,
}

impl MinecraftToken {
    pub fn authenticate(
        xbox_live_security_token: &str,
        user_hash: &str,
    ) -> Result<Self, AuthenticationError> {
        let http_client = reqwest::blocking::Client::new();
        let response = http_client
            .post(consts::MC_AUTH_TOKEN_URL)
            .json(&json!({
                "identityToken": format!("XBL3.0 x={};{}", user_hash, xbox_live_security_token)
            }))
            .send()?
            .error_for_status()?
            .json::<MinecraftTokenResponse>()?;

        Ok(response.into())
    }

    pub fn check_expired(&self) -> Result<(), AuthenticationError> {
        if let Some(exp) = &self.exp {
            let now = Utc::now();

            if exp < &now {
                return Err(AuthenticationError::ExpiredToken(TokenKind::Minecraft));
            }
        }

        Ok(())
    }
}

impl From<MinecraftTokenResponse> for MinecraftToken {
    fn from(resp: MinecraftTokenResponse) -> Self {
        let exp = Utc::now() - Duration::seconds(60) + Duration::seconds(resp.expires_in as i64);

        Self {
            username: resp.username,
            token: resp.access_token,
            exp: Some(exp),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct MinecraftTokenResponse {
    username: String,
    roles: Vec<serde_json::Value>,
    access_token: String,
    token_type: String,
    expires_in: u64,
}
