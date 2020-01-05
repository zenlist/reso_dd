// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [DeviceType Lookups](https://ddwiki.reso.org/display/DDW17/DeviceType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeviceType {
    /// "[Desktop](https://ddwiki.reso.org/display/DDW17/Desktop)": The Actor's device has been identified as a desktop device by the source
    Desktop,

    /// "[Mobile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244595)": The Actor's device has been identified as a mobile device by the source
    Mobile,

    /// "[Tablet](https://ddwiki.reso.org/display/DDW17/Tablet)": The Actor's device has been identified as a tablet device by the source
    Tablet,

    /// "[Unknown](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244597)": The Actor's device could not be identified by the source
    Unknown,

    /// "[Wearable](https://ddwiki.reso.org/display/DDW17/Wearable)": The Actor's device has been identified as a wearable device by the source
    Wearable,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for DeviceType {
    fn from_str(s: &str) -> DeviceType {
        match s {
            "Desktop" => DeviceType::Desktop,

            "Mobile" => DeviceType::Mobile,

            "Tablet" => DeviceType::Tablet,

            "Unknown" => DeviceType::Unknown,

            "Wearable" => DeviceType::Wearable,

            _ => DeviceType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> DeviceType {
        match s.as_ref() {
            "Desktop" => DeviceType::Desktop,

            "Mobile" => DeviceType::Mobile,

            "Tablet" => DeviceType::Tablet,

            "Unknown" => DeviceType::Unknown,

            "Wearable" => DeviceType::Wearable,

            _ => DeviceType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            DeviceType::Desktop => "Desktop",

            DeviceType::Mobile => "Mobile",

            DeviceType::Tablet => "Tablet",

            DeviceType::Unknown => "Unknown",

            DeviceType::Wearable => "Wearable",

            DeviceType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            DeviceType::Desktop => "Desktop".into(),

            DeviceType::Mobile => "Mobile".into(),

            DeviceType::Tablet => "Tablet".into(),

            DeviceType::Unknown => "Unknown".into(),

            DeviceType::Wearable => "Wearable".into(),

            DeviceType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            DeviceType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for DeviceType {
    fn from(s: String) -> DeviceType {
        match s.as_ref() {
            "Desktop" => DeviceType::Desktop,

            "Mobile" => DeviceType::Mobile,

            "Tablet" => DeviceType::Tablet,

            "Unknown" => DeviceType::Unknown,

            "Wearable" => DeviceType::Wearable,

            _ => DeviceType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for DeviceType {
    fn from(s: &str) -> DeviceType {
        match s {
            "Desktop" => DeviceType::Desktop,

            "Mobile" => DeviceType::Mobile,

            "Tablet" => DeviceType::Tablet,

            "Unknown" => DeviceType::Unknown,

            "Wearable" => DeviceType::Wearable,

            _ => DeviceType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a DeviceType> for &'a str {
    fn from(s: &'a DeviceType) -> &'a str {
        match s {
            DeviceType::Desktop => "Desktop",

            DeviceType::Mobile => "Mobile",

            DeviceType::Tablet => "Tablet",

            DeviceType::Unknown => "Unknown",

            DeviceType::Wearable => "Wearable",

            DeviceType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for DeviceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeviceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
