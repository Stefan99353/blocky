use super::error::{AuthenticationError, TokenKind};
use crate::consts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XboxLiveToken {
    pub token: String,
    pub exp: Option<DateTime<Utc>>,
    pub user_hash: Option<String>,
}

impl XboxLiveToken {
    pub fn authenticate(microsoft_token: &str) -> Result<Self, AuthenticationError> {
        let http_client = reqwest::blocking::Client::new();
        let response = http_client
            .post(consts::XBL_AUTH_TOKEN_URL)
            .json(&json!({
                "Properties": {
                    "AuthMethod": "RPS",
                    "SiteName": "user.auth.xboxlive.com",
                    "RpsTicket": format!("d={}", microsoft_token)
                },
                "RelyingParty": "http://auth.xboxlive.com",
                "TokenType": "JWT"
            }))
            .send()?
            .error_for_status()?
            .json::<XboxLiveTokenResponse>()?;

        Ok(response.into())
    }

    pub fn check_expired(&self) -> Result<(), AuthenticationError> {
        if let Some(exp) = &self.exp {
            let now = Utc::now();

            if exp < &now {
                return Err(AuthenticationError::ExpiredToken(TokenKind::XboxLive));
            }
        }

        Ok(())
    }
}

impl From<XboxLiveTokenResponse> for XboxLiveToken {
    fn from(resp: XboxLiveTokenResponse) -> Self {
        Self {
            token: resp.token,
            exp: Some(resp.not_after),
            user_hash: resp
                .display_claims
                .xui
                .first()
                .map(|uhs| uhs.uhs.to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveTokenResponse {
    // issue_instant: DateTime<Utc>,
    not_after: DateTime<Utc>,
    token: String,
    display_claims: DisplayClaims,
}

#[derive(Clone, Debug, Deserialize)]
struct DisplayClaims {
    xui: Vec<UserHash>,
}

#[derive(Clone, Debug, Deserialize)]
struct UserHash {
    uhs: String,
}
