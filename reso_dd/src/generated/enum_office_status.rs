// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OfficeStatus Lookups](https://ddwiki.reso.org/display/DDW17/OfficeStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OfficeStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245650)": The member office's account is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245651)": The member office's account is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OfficeStatus {
    fn from_str(s: &str) -> OfficeStatus {
        match s {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OfficeStatus {
        match s.as_ref() {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OfficeStatus::Active => "Active",

            OfficeStatus::Inactive => "Inactive",

            OfficeStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OfficeStatus::Active => "Active".into(),

            OfficeStatus::Inactive => "Inactive".into(),

            OfficeStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OfficeStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OfficeStatus {
    fn from(s: String) -> OfficeStatus {
        match s.as_ref() {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OfficeStatus {
    fn from(s: &str) -> OfficeStatus {
        match s {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OfficeStatus> for &'a str {
    fn from(s: &'a OfficeStatus) -> &'a str {
        match s {
            OfficeStatus::Active => "Active",

            OfficeStatus::Inactive => "Inactive",

            OfficeStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OfficeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OfficeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
