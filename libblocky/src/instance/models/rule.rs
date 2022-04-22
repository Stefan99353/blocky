use crate::os::{Arch, Platform};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    pub action: Action,
    pub os: Option<Os>,
    pub features: Option<Features>,
}

impl Rule {
    pub fn allows(&self) -> bool {
        if let Some(os) = &self.os {
            if let Some(platform) = &os.platform {
                if platform != &Platform::current() {
                    return !self.action.to_bool();
                }
            }
        }

        if let Some(features) = &self.features {
            if features.is_demo_user.is_some() || features.has_custom_resolution.is_some() {
                return false;
            }
        }

        self.action.to_bool()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Action {
    #[serde(alias = "allow")]
    Allow,
    #[serde(alias = "disallow")]
    Disallow,
}

impl Action {
    pub fn to_bool(&self) -> bool {
        match self {
            Action::Allow => true,
            Action::Disallow => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Os {
    #[serde(alias = "name")]
    pub platform: Option<Platform>,
    pub version: Option<String>,
    pub arch: Option<Arch>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
}
