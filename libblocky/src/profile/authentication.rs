use super::AuthenticationError;
use super::Profile;
use crate::consts;
use crate::profile::entitlements::Entitlements;
use crate::profile::error::TokenKind;
use crate::profile::microsoft::MicrosoftToken;
use crate::profile::minecraft::{MinecraftProfile, MinecraftToken};
use crate::profile::xbox_live::XboxLiveToken;
use crate::profile::xbox_live_security::XboxLiveSecurityToken;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use reqwest::Url;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use uuid::Uuid;

impl Profile {
    pub fn authenticate_microsoft(
        client_id: &str,
        client_secret: &str,
    ) -> Result<Self, crate::error::Error> {
        debug!("Authenticate with Microsoft");

        trace!("Setting up TCP listener");
        let tcp_listener =
            TcpListener::bind(("127.0.0.1", 0)).map_err(AuthenticationError::TcpListener)?;
        let listen_address = tcp_listener
            .local_addr()
            .map_err(AuthenticationError::TcpListener)?;
        let listen_port = listen_address.port();

        trace!("Setting up OAuth client");
        let client_id = ClientId::new(client_id.to_string());
        let client_secret = ClientSecret::new(client_secret.to_string());
        let auth_url = AuthUrl::new(consts::MS_AUTH_CODE_URL.to_string()).unwrap();
        let token_url = TokenUrl::new(consts::MS_AUTH_TOKEN_URL.to_string()).unwrap();
        let redirect_url =
            RedirectUrl::new(format!("http://localhost:{}/blocky_auth", listen_port)).unwrap();

        let oauth_client =
            BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_auth_type(AuthType::RequestBody)
                .set_redirect_uri(redirect_url);
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (login_url, _) = oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("XboxLive.signin".to_string()))
            .add_scope(Scope::new("XboxLive.offline_access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        debug!("Open login URL in default browser");
        if let Err(err) = webbrowser::open(login_url.as_str()) {
            error!("Could not open login URL in default browser");
            error!("{}", err);
            info!("Login URL: {}", login_url);
        }

        debug!("Waiting for redirect on {}", listen_address);
        // TODO: Mechanism to cancel after N seconds
        let authorization_code = receive_authorization_code(tcp_listener)?;

        debug!("Exchanging authorization code for token");
        let token_response = oauth_client
            .exchange_code(authorization_code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)
            .map_err(|_| AuthenticationError::CodeExchange)?;

        let profile = Self {
            uuid: Uuid::new_v4(),
            microsoft: MicrosoftToken::from_token_response(token_response),
            xbox_live: None,
            xbox_live_security: None,
            minecraft: None,
            entitlements: None,
            minecraft_profile: None,
        };

        Ok(profile)
    }

    pub fn authenticate_xbox_live(&mut self) -> Result<(), crate::error::Error> {
        debug!("Authenticate with XBox Live");

        self.microsoft.check_expired()?;

        let token = XboxLiveToken::authenticate(&self.microsoft.token)?;
        self.xbox_live = Some(token);

        Ok(())
    }

    pub fn authenticate_xbox_live_security(&mut self) -> Result<(), crate::error::Error> {
        debug!("Authenticate with XBox Live Security");

        match &self.xbox_live {
            None => Err(AuthenticationError::NotAuthenticated(TokenKind::XboxLive).into()),
            Some(xbox_live) => {
                xbox_live.check_expired()?;

                let token = XboxLiveSecurityToken::authenticate(&xbox_live.token)?;
                self.xbox_live_security = Some(token);

                Ok(())
            }
        }
    }

    pub fn authenticate_minecraft(&mut self) -> Result<(), crate::error::Error> {
        debug!("Authenticate with Minecraft");

        match &self.xbox_live_security {
            None => Err(AuthenticationError::NotAuthenticated(TokenKind::XboxLiveSecret).into()),
            Some(xbox_live_security) => {
                xbox_live_security.check_expired()?;

                let user_hash = xbox_live_security
                    .user_hash
                    .as_ref()
                    .ok_or(AuthenticationError::UserHash)?;

                let token = MinecraftToken::authenticate(&xbox_live_security.token, user_hash)?;
                self.minecraft = Some(token);

                Ok(())
            }
        }
    }

    pub fn set_entitlements(&mut self) -> Result<(), crate::error::Error> {
        debug!("Get Minecraft entitlements");

        match &self.minecraft {
            None => Err(AuthenticationError::NotAuthenticated(TokenKind::Minecraft).into()),
            Some(minecraft) => {
                minecraft.check_expired()?;

                let entitlements = Entitlements::get_entitlements(&minecraft.token)?;
                self.entitlements = Some(entitlements);

                Ok(())
            }
        }
    }

    pub fn set_profile(&mut self) -> Result<(), crate::error::Error> {
        debug!("Get Minecraft profile");

        match &self.minecraft {
            None => Err(AuthenticationError::NotAuthenticated(TokenKind::Minecraft).into()),
            Some(minecraft) => {
                minecraft.check_expired()?;

                // Check entitlements
                match &self.entitlements {
                    None => {
                        return Err(AuthenticationError::MissingEntitlements.into());
                    }
                    Some(entitlements) => {
                        if !entitlements.owns_minecraft() {
                            return Err(AuthenticationError::NotEntitled.into());
                        }
                    }
                }

                let profile = MinecraftProfile::get_profile(&minecraft.token)?;
                self.minecraft_profile = Some(profile);

                Ok(())
            }
        }
    }
}

fn receive_authorization_code(
    listener: TcpListener,
) -> Result<AuthorizationCode, AuthenticationError> {
    let (mut stream, _) = listener
        .accept()
        .map_err(AuthenticationError::TcpListener)?;
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    reader
        .read_line(&mut request_line)
        .map_err(AuthenticationError::TcpListener)?;

    let redirect_url = request_line
        .split_whitespace()
        .nth(1)
        .ok_or(AuthenticationError::OAuthResponse)?;
    let url = Url::parse(&format!("http://localhost{}", redirect_url))
        .map_err(|_| AuthenticationError::OAuthResponse)?;

    let auth_code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| AuthorizationCode::new(value.into_owned()))
        .ok_or(AuthenticationError::OAuthResponse)?;

    stream
        .write_all(&success_message())
        .map_err(AuthenticationError::TcpListener)?;

    Ok(auth_code)
}

fn success_message() -> Vec<u8> {
    let message = "This window can be closed";
    format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    )
    .into_bytes()
}
