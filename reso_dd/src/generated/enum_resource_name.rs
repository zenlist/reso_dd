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

pub(crate) mod option_vec_resource_name_format {
    use super::ResourceName;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ResourceName>>,
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
    ) -> Result<Option<Vec<ResourceName>>, D::Error>
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
