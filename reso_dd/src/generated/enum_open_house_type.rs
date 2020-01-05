// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OpenHouseType Lookups](https://ddwiki.reso.org/display/DDW17/OpenHouseType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenHouseType {
    /// "[Broker](https://ddwiki.reso.org/display/DDW17/Broker)": The open house is only open to brokers, and at times agents.
    Broker,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245670)": The open house is only open to the members of a particular office(s).
    Office,

    /// "[Public](https://ddwiki.reso.org/display/DDW17/Public)": The open house is open to the general public.
    Public,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OpenHouseType {
    fn from_str(s: &str) -> OpenHouseType {
        match s {
            "Broker" => OpenHouseType::Broker,

            "Office" => OpenHouseType::Office,

            "Public" => OpenHouseType::Public,

            _ => OpenHouseType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OpenHouseType {
        match s.as_ref() {
            "Broker" => OpenHouseType::Broker,

            "Office" => OpenHouseType::Office,

            "Public" => OpenHouseType::Public,

            _ => OpenHouseType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OpenHouseType::Broker => "Broker",

            OpenHouseType::Office => "Office",

            OpenHouseType::Public => "Public",

            OpenHouseType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OpenHouseType::Broker => "Broker".into(),

            OpenHouseType::Office => "Office".into(),

            OpenHouseType::Public => "Public".into(),

            OpenHouseType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OpenHouseType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OpenHouseType {
    fn from(s: String) -> OpenHouseType {
        match s.as_ref() {
            "Broker" => OpenHouseType::Broker,

            "Office" => OpenHouseType::Office,

            "Public" => OpenHouseType::Public,

            _ => OpenHouseType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OpenHouseType {
    fn from(s: &str) -> OpenHouseType {
        match s {
            "Broker" => OpenHouseType::Broker,

            "Office" => OpenHouseType::Office,

            "Public" => OpenHouseType::Public,

            _ => OpenHouseType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OpenHouseType> for &'a str {
    fn from(s: &'a OpenHouseType) -> &'a str {
        match s {
            OpenHouseType::Broker => "Broker",

            OpenHouseType::Office => "Office",

            OpenHouseType::Public => "Public",

            OpenHouseType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OpenHouseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OpenHouseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
