// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OfficeBranchType Lookups](https://ddwiki.reso.org/display/DDW17/OfficeBranchType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OfficeBranchType {
    /// "[Branch](https://ddwiki.reso.org/display/DDW17/Branch)": This office is a branch office.
    Branch,

    /// "[Main](https://ddwiki.reso.org/display/DDW17/Main)": This office is the broker's main office.
    Main,

    /// "[Stand Alone](https://ddwiki.reso.org/display/DDW17/Stand+Alone)": This office is a stand alone, or single office brokerage.
    StandAlone,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OfficeBranchType {
    fn from_str(s: &str) -> OfficeBranchType {
        match s {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OfficeBranchType {
        match s.as_ref() {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OfficeBranchType::Branch => "Branch",

            OfficeBranchType::Main => "Main",

            OfficeBranchType::StandAlone => "Stand Alone",

            OfficeBranchType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OfficeBranchType::Branch => "Branch".into(),

            OfficeBranchType::Main => "Main".into(),

            OfficeBranchType::StandAlone => "Stand Alone".into(),

            OfficeBranchType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OfficeBranchType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OfficeBranchType {
    fn from(s: String) -> OfficeBranchType {
        match s.as_ref() {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OfficeBranchType {
    fn from(s: &str) -> OfficeBranchType {
        match s {
            "Branch" => OfficeBranchType::Branch,

            "Main" => OfficeBranchType::Main,

            "Stand Alone" => OfficeBranchType::StandAlone,

            _ => OfficeBranchType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OfficeBranchType> for &'a str {
    fn from(s: &'a OfficeBranchType) -> &'a str {
        match s {
            OfficeBranchType::Branch => "Branch",

            OfficeBranchType::Main => "Main",

            OfficeBranchType::StandAlone => "Stand Alone",

            OfficeBranchType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OfficeBranchType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OfficeBranchType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
