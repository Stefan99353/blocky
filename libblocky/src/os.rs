use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Platform {
    Linux,
    MacOs,
    Windows,
    Other,
}

impl Platform {
    pub fn current() -> Self {
        match std::env::consts::OS {
            "linux" => Self::Linux,
            "macos" => Self::MacOs,
            "windows" => Self::Windows,
            _ => Self::Other,
        }
    }

    pub fn matches_current(&self) -> bool {
        self == &Self::current()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Arch {
    #[serde(alias = "x86")]
    I386,
    #[serde(alias = "x86_64")]
    AMD64,
}

impl Arch {
    pub fn current() -> Self {
        if cfg!(target_pointer_width = "32") {
            Self::I386
        } else {
            Self::AMD64
        }
    }

    pub fn get_bits(&self) -> u8 {
        match self {
            Arch::I386 => 32,
            Arch::AMD64 => 64,
        }
    }
}
