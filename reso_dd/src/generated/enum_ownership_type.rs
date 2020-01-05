// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OwnershipType Lookups](https://ddwiki.reso.org/display/DDW17/OwnershipType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OwnershipType {
    /// "[Corporation](https://ddwiki.reso.org/display/DDW17/Corporation)": The ownership type of the business being sold is a corporation.
    Corporation,

    /// "[LLC](https://ddwiki.reso.org/display/DDW17/LLC)": The ownership type of the business being sold is a limited liability corporation.
    LLC,

    /// "[Partnership](https://ddwiki.reso.org/display/DDW17/Partnership)": The ownership type of the business being sold is a partnership.
    Partnership,

    /// "[Sole Proprietor](https://ddwiki.reso.org/display/DDW17/Sole+Proprietor)": The ownership type of the business being sold is a sole proprietor.
    SoleProprietor,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OwnershipType {
    fn from_str(s: &str) -> OwnershipType {
        match s {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OwnershipType {
        match s.as_ref() {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OwnershipType::Corporation => "Corporation",

            OwnershipType::LLC => "LLC",

            OwnershipType::Partnership => "Partnership",

            OwnershipType::SoleProprietor => "Sole Proprietor",

            OwnershipType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OwnershipType::Corporation => "Corporation".into(),

            OwnershipType::LLC => "LLC".into(),

            OwnershipType::Partnership => "Partnership".into(),

            OwnershipType::SoleProprietor => "Sole Proprietor".into(),

            OwnershipType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OwnershipType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OwnershipType {
    fn from(s: String) -> OwnershipType {
        match s.as_ref() {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OwnershipType {
    fn from(s: &str) -> OwnershipType {
        match s {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OwnershipType> for &'a str {
    fn from(s: &'a OwnershipType) -> &'a str {
        match s {
            OwnershipType::Corporation => "Corporation",

            OwnershipType::LLC => "LLC",

            OwnershipType::Partnership => "Partnership",

            OwnershipType::SoleProprietor => "Sole Proprietor",

            OwnershipType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OwnershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OwnershipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
