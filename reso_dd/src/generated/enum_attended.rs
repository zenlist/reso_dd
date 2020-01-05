// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Attended Lookups](https://ddwiki.reso.org/display/DDW17/Attended+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Attended {
    /// "[Agent](https://ddwiki.reso.org/display/DDW17/Agent)": A licensed real estate agent will be present at the open house event.
    Agent,

    /// "[Seller](https://ddwiki.reso.org/display/DDW17/Seller)": A licensed real estate agent will not be present and the property owner will be present at the open house event.
    Seller,

    /// "[Unattended](https://ddwiki.reso.org/display/DDW17/Unattended)": The open house event will not be attended.  Access will normally be via lockbox or other pre-arranged means.
    Unattended,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Attended {
    fn from_str(s: &str) -> Attended {
        match s {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Attended {
        match s.as_ref() {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Attended::Agent => "Agent",

            Attended::Seller => "Seller",

            Attended::Unattended => "Unattended",

            Attended::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Attended::Agent => "Agent".into(),

            Attended::Seller => "Seller".into(),

            Attended::Unattended => "Unattended".into(),

            Attended::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Attended::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Attended {
    fn from(s: String) -> Attended {
        match s.as_ref() {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Attended {
    fn from(s: &str) -> Attended {
        match s {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Attended> for &'a str {
    fn from(s: &'a Attended) -> &'a str {
        match s {
            Attended::Agent => "Agent",

            Attended::Seller => "Seller",

            Attended::Unattended => "Unattended",

            Attended::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Attended {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Attended {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
