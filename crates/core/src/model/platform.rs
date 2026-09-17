use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// The target platform for a build, package, or publish operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Android,
    #[serde(rename = "ios")]
    IOS,
    #[serde(rename = "macos")]
    MacOS,
    Windows,
    Linux,
    Web,
    #[serde(rename = "ohos")]
    Ohos,
}

impl Platform {
    /// Return the canonical string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::IOS => "ios",
            Self::MacOS => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Web => "web",
            Self::Ohos => "ohos",
        }
    }

    /// Return all known platform variants.
    pub const fn all() -> &'static [Platform] {
        &[
            Self::Android,
            Self::IOS,
            Self::MacOS,
            Self::Windows,
            Self::Linux,
            Self::Web,
            Self::Ohos,
        ]
    }

    /// The current host operating system as a `Platform`.
    ///
    /// Returns `None` when the host OS does not match any known platform.
    pub fn current() -> Option<Self> {
        if cfg!(target_os = "android") {
            Some(Self::Android)
        } else if cfg!(target_os = "ios") {
            Some(Self::IOS)
        } else if cfg!(target_os = "macos") {
            Some(Self::MacOS)
        } else if cfg!(target_os = "windows") {
            Some(Self::Windows)
        } else if cfg!(target_os = "linux") {
            Some(Self::Linux)
        } else {
            None
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::IOS),
            "macos" => Ok(Self::MacOS),
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            "web" => Ok(Self::Web),
            "ohos" => Ok(Self::Ohos),
            other => Err(format!("invalid platform: `{other}`")),
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_str() {
        for p in Platform::all() {
            assert_eq!(Platform::from_str(p.as_str()).unwrap(), *p);
        }
    }

    #[test]
    fn case_insensitive_parse() {
        assert_eq!("Android".parse::<Platform>().unwrap(), Platform::Android);
        assert_eq!("IOS".parse::<Platform>().unwrap(), Platform::IOS);
        assert_eq!("macOS".parse::<Platform>().unwrap(), Platform::MacOS);
    }

    #[test]
    fn invalid_platform() {
        assert!("tizen".parse::<Platform>().is_err());
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&Platform::MacOS).unwrap();
        assert_eq!(json, "\"macos\"");
        let back: Platform = serde_json::from_str(&json).unwrap();
        assert_eq!(back, Platform::MacOS);
    }

    #[test]
    fn serde_ios() {
        let json = "\"ios\"";
        let p: Platform = serde_json::from_str(json).unwrap();
        assert_eq!(p, Platform::IOS);
    }

    #[test]
    fn display_matches_as_str() {
        for p in Platform::all() {
            assert_eq!(format!("{p}"), p.as_str());
        }
    }

    #[test]
    fn current_matches_cfg() {
        let current = Platform::current();
        #[cfg(target_os = "macos")]
        assert_eq!(current, Some(Platform::MacOS));
        #[cfg(target_os = "linux")]
        assert_eq!(current, Some(Platform::Linux));
        #[cfg(target_os = "windows")]
        assert_eq!(current, Some(Platform::Windows));
    }
}
