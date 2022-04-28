mod authentication;
mod entitlements;
mod error;
mod microsoft;
pub(crate) mod minecraft;
mod xbox_live;
mod xbox_live_security;

use entitlements::Entitlements;
pub use error::AuthenticationError;
use microsoft::MicrosoftToken;
use minecraft::{MinecraftProfile, MinecraftToken};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xbox_live::XboxLiveToken;
use xbox_live_security::XboxLiveSecurityToken;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub uuid: Uuid,
    pub microsoft: MicrosoftToken,
    pub xbox_live: Option<XboxLiveToken>,
    pub xbox_live_security: Option<XboxLiveSecurityToken>,
    pub minecraft: Option<MinecraftToken>,
    pub entitlements: Option<Entitlements>,
    pub minecraft_profile: Option<MinecraftProfile>,
}
