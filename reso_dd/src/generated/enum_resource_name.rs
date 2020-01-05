// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ResourceName Lookups](https://ddwiki.reso.org/display/DDW17/ResourceName+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResourceName {
    /// "[Contacts](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246107)": This record is related to another record in the Contacts resource.
    Contacts,

    /// "[Member](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246108)": This record is related to another record in the Member resource.
    Member,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246109)": This record is related to another record in the Office resource.
    Office,

    /// "[Property](https://ddwiki.reso.org/display/DDW17/Property)": This record is related to another record in the Property resource.
    Property,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ResourceName {
    fn from_str(s: &str) -> ResourceName {
        match s {
            "Contacts" => ResourceName::Contacts,

            "Member" => ResourceName::Member,

            "Office" => ResourceName::Office,

            "Property" => ResourceName::Property,

            _ => ResourceName::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ResourceName {
        match s.as_ref() {
            "Contacts" => ResourceName::Contacts,

            "Member" => ResourceName::Member,

            "Office" => ResourceName::Office,

            "Property" => ResourceName::Property,

            _ => ResourceName::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ResourceName::Contacts => "Contacts",

            ResourceName::Member => "Member",

            ResourceName::Office => "Office",

            ResourceName::Property => "Property",

            ResourceName::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ResourceName::Contacts => "Contacts".into(),

            ResourceName::Member => "Member".into(),

            ResourceName::Office => "Office".into(),

            ResourceName::Property => "Property".into(),

            ResourceName::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ResourceName::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ResourceName {
    fn from(s: String) -> ResourceName {
        match s.as_ref() {
            "Contacts" => ResourceName::Contacts,

            "Member" => ResourceName::Member,

            "Office" => ResourceName::Office,

            "Property" => ResourceName::Property,

            _ => ResourceName::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ResourceName {
    fn from(s: &str) -> ResourceName {
        match s {
            "Contacts" => ResourceName::Contacts,

            "Member" => ResourceName::Member,

            "Office" => ResourceName::Office,

            "Property" => ResourceName::Property,

            _ => ResourceName::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ResourceName> for &'a str {
    fn from(s: &'a ResourceName) -> &'a str {
        match s {
            ResourceName::Contacts => "Contacts",

            ResourceName::Member => "Member",

            ResourceName::Office => "Office",

            ResourceName::Property => "Property",

            ResourceName::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ResourceName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ResourceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
