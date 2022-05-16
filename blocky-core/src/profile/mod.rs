mod authentication;
mod entitlements;
pub(crate) mod error;
mod microsoft;
mod minecraft;
mod xbox_live;
mod xbox_live_security;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub uuid: Uuid,
    pub microsoft: microsoft::MicrosoftToken,
    pub xbox_live: Option<xbox_live::XboxLiveToken>,
    pub xbox_live_security: Option<xbox_live_security::XboxLiveSecurityToken>,
    pub minecraft: Option<minecraft::MinecraftToken>,
    pub entitlements: Option<entitlements::Entitlements>,
    pub minecraft_profile: Option<minecraft::MinecraftProfile>,
}
