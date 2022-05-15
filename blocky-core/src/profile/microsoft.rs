use super::error::{AuthenticationError, TokenKind};
use chrono::{DateTime, Duration, Utc};
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MicrosoftToken {
    pub token: String,
    pub refresh_token: Option<String>,
    pub exp: Option<DateTime<Utc>>,
}

impl MicrosoftToken {
    pub fn from_token_response(btr: BasicTokenResponse) -> Self {
        let exp = btr
            .expires_in()
            .map(|d| Utc::now() - Duration::seconds(60) + Duration::seconds(d.as_secs() as i64));

        Self {
            token: btr.access_token().secret().to_string(),
            refresh_token: btr.refresh_token().map(|t| t.secret().to_string()),
            exp,
        }
    }

    pub fn check_expired(&self) -> Result<(), AuthenticationError> {
        if let Some(exp) = &self.exp {
            let now = Utc::now();

            if exp < &now {
                return Err(AuthenticationError::ExpiredToken(TokenKind::Microsoft));
            }
        }

        Ok(())
    }
}
