// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PreferredAddress Lookups](https://ddwiki.reso.org/display/DDW17/PreferredAddress+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PreferredAddress {
    /// "[Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245953)": The contact prefers the use of their home address.
    Home,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245954)": The contact prefers the use of their other address.
    Other,

    /// "[Work](https://ddwiki.reso.org/display/DDW17/Work)": The contact prefers the use of their work address.
    Work,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PreferredAddress {
    fn from_str(s: &str) -> PreferredAddress {
        match s {
            "Home" => PreferredAddress::Home,

            "Other" => PreferredAddress::Other,

            "Work" => PreferredAddress::Work,

            _ => PreferredAddress::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PreferredAddress {
        match s.as_ref() {
            "Home" => PreferredAddress::Home,

            "Other" => PreferredAddress::Other,

            "Work" => PreferredAddress::Work,

            _ => PreferredAddress::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PreferredAddress::Home => "Home",

            PreferredAddress::Other => "Other",

            PreferredAddress::Work => "Work",

            PreferredAddress::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PreferredAddress::Home => "Home".into(),

            PreferredAddress::Other => "Other".into(),

            PreferredAddress::Work => "Work".into(),

            PreferredAddress::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PreferredAddress::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PreferredAddress {
    fn from(s: String) -> PreferredAddress {
        match s.as_ref() {
            "Home" => PreferredAddress::Home,

            "Other" => PreferredAddress::Other,

            "Work" => PreferredAddress::Work,

            _ => PreferredAddress::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PreferredAddress {
    fn from(s: &str) -> PreferredAddress {
        match s {
            "Home" => PreferredAddress::Home,

            "Other" => PreferredAddress::Other,

            "Work" => PreferredAddress::Work,

            _ => PreferredAddress::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PreferredAddress> for &'a str {
    fn from(s: &'a PreferredAddress) -> &'a str {
        match s {
            PreferredAddress::Home => "Home",

            PreferredAddress::Other => "Other",

            PreferredAddress::Work => "Work",

            PreferredAddress::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PreferredAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PreferredAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
