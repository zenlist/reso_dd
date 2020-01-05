// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OccupantType Lookups](https://ddwiki.reso.org/display/DDW17/OccupantType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OccupantType {
    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245646)": The occupant is the owner.
    Owner,

    /// "[Tenant](https://ddwiki.reso.org/display/DDW17/Tenant)": The occupant is a tenant.
    Tenant,

    /// "[Vacant](https://ddwiki.reso.org/display/DDW17/Vacant)": The property is vacant.
    Vacant,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OccupantType {
    fn from_str(s: &str) -> OccupantType {
        match s {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OccupantType {
        match s.as_ref() {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OccupantType::Owner => "Owner",

            OccupantType::Tenant => "Tenant",

            OccupantType::Vacant => "Vacant",

            OccupantType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OccupantType::Owner => "Owner".into(),

            OccupantType::Tenant => "Tenant".into(),

            OccupantType::Vacant => "Vacant".into(),

            OccupantType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OccupantType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OccupantType {
    fn from(s: String) -> OccupantType {
        match s.as_ref() {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OccupantType {
    fn from(s: &str) -> OccupantType {
        match s {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OccupantType> for &'a str {
    fn from(s: &'a OccupantType) -> &'a str {
        match s {
            OccupantType::Owner => "Owner",

            OccupantType::Tenant => "Tenant",

            OccupantType::Vacant => "Vacant",

            OccupantType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OccupantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OccupantType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
