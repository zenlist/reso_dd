// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ShowingContactType Lookups](https://ddwiki.reso.org/display/DDW17/ShowingContactType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShowingContactType {
    /// "[Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246270)": The showing contact is a licensed agent.
    Agent,

    /// "[Occupant](https://ddwiki.reso.org/display/DDW17/Occupant)": The showing contact is the occupant.
    Occupant,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246272)": The showing contact is the owner.
    Owner,

    /// "[Property Manager](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246273)": The showing contact is the property manager.
    PropertyManager,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ShowingContactType {
    fn from_str(s: &str) -> ShowingContactType {
        match s {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ShowingContactType {
        match s.as_ref() {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ShowingContactType::Agent => "Agent",

            ShowingContactType::Occupant => "Occupant",

            ShowingContactType::Owner => "Owner",

            ShowingContactType::PropertyManager => "Property Manager",

            ShowingContactType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ShowingContactType::Agent => "Agent".into(),

            ShowingContactType::Occupant => "Occupant".into(),

            ShowingContactType::Owner => "Owner".into(),

            ShowingContactType::PropertyManager => "Property Manager".into(),

            ShowingContactType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ShowingContactType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ShowingContactType {
    fn from(s: String) -> ShowingContactType {
        match s.as_ref() {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ShowingContactType {
    fn from(s: &str) -> ShowingContactType {
        match s {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ShowingContactType> for &'a str {
    fn from(s: &'a ShowingContactType) -> &'a str {
        match s {
            ShowingContactType::Agent => "Agent",

            ShowingContactType::Occupant => "Occupant",

            ShowingContactType::Owner => "Owner",

            ShowingContactType::PropertyManager => "Property Manager",

            ShowingContactType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ShowingContactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ShowingContactType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
