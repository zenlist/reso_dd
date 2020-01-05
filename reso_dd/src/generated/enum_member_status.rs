// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MemberStatus Lookups](https://ddwiki.reso.org/display/DDW17/MemberStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245584)": The member's account is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/display/DDW17/Inactive)": the member's account is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for MemberStatus {
    fn from_str(s: &str) -> MemberStatus {
        match s {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> MemberStatus {
        match s.as_ref() {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            MemberStatus::Active => "Active",

            MemberStatus::Inactive => "Inactive",

            MemberStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            MemberStatus::Active => "Active".into(),

            MemberStatus::Inactive => "Inactive".into(),

            MemberStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            MemberStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for MemberStatus {
    fn from(s: String) -> MemberStatus {
        match s.as_ref() {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MemberStatus {
    fn from(s: &str) -> MemberStatus {
        match s {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MemberStatus> for &'a str {
    fn from(s: &'a MemberStatus) -> &'a str {
        match s {
            MemberStatus::Active => "Active",

            MemberStatus::Inactive => "Inactive",

            MemberStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MemberStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MemberStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
