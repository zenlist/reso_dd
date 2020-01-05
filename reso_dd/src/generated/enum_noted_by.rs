// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [NotedBy Lookups](https://ddwiki.reso.org/display/DDW17/NotedBy+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NotedBy {
    /// "[Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246727)": The agent has written the given note about the given listing.
    Agent,

    /// "[Contact](https://ddwiki.reso.org/display/DDW17/Contact)": The contact has written the given note about the given listing.
    Contact,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for NotedBy {
    fn from_str(s: &str) -> NotedBy {
        match s {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> NotedBy {
        match s.as_ref() {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            NotedBy::Agent => "Agent",

            NotedBy::Contact => "Contact",

            NotedBy::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            NotedBy::Agent => "Agent".into(),

            NotedBy::Contact => "Contact".into(),

            NotedBy::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            NotedBy::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for NotedBy {
    fn from(s: String) -> NotedBy {
        match s.as_ref() {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s),
        }
    }
}

impl From<&str> for NotedBy {
    fn from(s: &str) -> NotedBy {
        match s {
            "Agent" => NotedBy::Agent,

            "Contact" => NotedBy::Contact,

            _ => NotedBy::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a NotedBy> for &'a str {
    fn from(s: &'a NotedBy) -> &'a str {
        match s {
            NotedBy::Agent => "Agent",

            NotedBy::Contact => "Contact",

            NotedBy::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for NotedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for NotedBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
