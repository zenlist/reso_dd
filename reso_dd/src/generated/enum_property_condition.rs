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

pub(crate) mod option_vec_property_condition_format {
    use super::PropertyCondition;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PropertyCondition>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match items {
            None => return serializer.serialize_none(),
            Some(ref vec) if vec.len() == 0 => serializer.serialize_str(""),
            Some(ref vec) => {
                let items: Vec<&str> = vec.iter().map(|item| item.into()).collect();
                let joined = items.join(",");
                serializer.serialize_str(&joined)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<PropertyCondition>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "" {
            return Ok(Some(vec![]));
        }

        let items = s.split(",").map(|i| From::<&str>::from(i)).collect();
        Ok(Some(items))
    }
}
