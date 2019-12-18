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

pub(crate) mod option_vec_preferred_address_format {
    use super::PreferredAddress;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PreferredAddress>>,
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
    ) -> Result<Option<Vec<PreferredAddress>>, D::Error>
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
