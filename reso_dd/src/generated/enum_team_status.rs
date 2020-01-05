// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TeamStatus Lookups](https://ddwiki.reso.org/display/DDW17/TeamStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TeamStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246521)": The team is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246522)": The team is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for TeamStatus {
    fn from_str(s: &str) -> TeamStatus {
        match s {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> TeamStatus {
        match s.as_ref() {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            TeamStatus::Active => "Active",

            TeamStatus::Inactive => "Inactive",

            TeamStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            TeamStatus::Active => "Active".into(),

            TeamStatus::Inactive => "Inactive".into(),

            TeamStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            TeamStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for TeamStatus {
    fn from(s: String) -> TeamStatus {
        match s.as_ref() {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TeamStatus {
    fn from(s: &str) -> TeamStatus {
        match s {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TeamStatus> for &'a str {
    fn from(s: &'a TeamStatus) -> &'a str {
        match s {
            TeamStatus::Active => "Active",

            TeamStatus::Inactive => "Inactive",

            TeamStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TeamStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TeamStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
