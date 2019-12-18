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

pub(crate) mod option_vec_open_house_type_format {
    use super::OpenHouseType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OpenHouseType>>,
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
    ) -> Result<Option<Vec<OpenHouseType>>, D::Error>
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
