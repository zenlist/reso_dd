// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PropertyCondition Lookups](https://ddwiki.reso.org/display/DDW17/PropertyCondition+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PropertyCondition {
    /// "[Fixer](https://ddwiki.reso.org/display/DDW17/Fixer)": The property is a "fixer" or property in need of moderate or extensive repair.
    Fixer,

    /// "[New Construction](https://ddwiki.reso.org/display/DDW17/New+Construction)": The property is newly built.
    NewConstruction,

    /// "[Under Construction](https://ddwiki.reso.org/display/DDW17/Under+Construction)": The property is still under construction and building has not been completed.
    UnderConstruction,

    /// "[Updated/Remodeled](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245870)": The property has been remolded or updated is some fashion.
    UpdatedRemodeled,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PropertyCondition {
    fn from_str(s: &str) -> PropertyCondition {
        match s {
            "Fixer" => PropertyCondition::Fixer,

            "New Construction" => PropertyCondition::NewConstruction,

            "Under Construction" => PropertyCondition::UnderConstruction,

            "Updated/Remodeled" => PropertyCondition::UpdatedRemodeled,

            _ => PropertyCondition::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PropertyCondition {
        match s.as_ref() {
            "Fixer" => PropertyCondition::Fixer,

            "New Construction" => PropertyCondition::NewConstruction,

            "Under Construction" => PropertyCondition::UnderConstruction,

            "Updated/Remodeled" => PropertyCondition::UpdatedRemodeled,

            _ => PropertyCondition::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PropertyCondition::Fixer => "Fixer",

            PropertyCondition::NewConstruction => "New Construction",

            PropertyCondition::UnderConstruction => "Under Construction",

            PropertyCondition::UpdatedRemodeled => "Updated/Remodeled",

            PropertyCondition::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PropertyCondition::Fixer => "Fixer".into(),

            PropertyCondition::NewConstruction => "New Construction".into(),

            PropertyCondition::UnderConstruction => "Under Construction".into(),

            PropertyCondition::UpdatedRemodeled => "Updated/Remodeled".into(),

            PropertyCondition::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PropertyCondition::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PropertyCondition {
    fn from(s: String) -> PropertyCondition {
        match s.as_ref() {
            "Fixer" => PropertyCondition::Fixer,

            "New Construction" => PropertyCondition::NewConstruction,

            "Under Construction" => PropertyCondition::UnderConstruction,

            "Updated/Remodeled" => PropertyCondition::UpdatedRemodeled,

            _ => PropertyCondition::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PropertyCondition {
    fn from(s: &str) -> PropertyCondition {
        match s {
            "Fixer" => PropertyCondition::Fixer,

            "New Construction" => PropertyCondition::NewConstruction,

            "Under Construction" => PropertyCondition::UnderConstruction,

            "Updated/Remodeled" => PropertyCondition::UpdatedRemodeled,

            _ => PropertyCondition::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PropertyCondition> for &'a str {
    fn from(s: &'a PropertyCondition) -> &'a str {
        match s {
            PropertyCondition::Fixer => "Fixer",

            PropertyCondition::NewConstruction => "New Construction",

            PropertyCondition::UnderConstruction => "Under Construction",

            PropertyCondition::UpdatedRemodeled => "Updated/Remodeled",

            PropertyCondition::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PropertyCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PropertyCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
