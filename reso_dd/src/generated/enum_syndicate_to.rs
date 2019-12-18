// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [SyndicateTo Lookups](https://ddwiki.reso.org/display/DDW17/SyndicateTo+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SyndicateTo {
    /// "[Homes.com](https://ddwiki.reso.org/display/DDW17/Homes.com)": The Broker, or Member if permitted by the Broker, is allowing their listings to be sent to Homes.com.
    Homescom,

    /// "[ListHub](https://ddwiki.reso.org/display/DDW17/ListHub)": The Broker, or Member if permitted by the Broker, is allowing their listings to be sent to ListHub.com.
    ListHub,

    /// "[Realtor.com](https://ddwiki.reso.org/display/DDW17/Realtor.com)": The Broker, or Member if permitted by the Broker, is allowing their listings to be sent to Realtor.com.
    Realtorcom,

    /// "[Zillow/Trulia](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246431)": The Broker, or Member if permitted by the Broker, is allowing their listings to be sent to Zillow and Trulia.
    ZillowTrulia,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for SyndicateTo {
    fn from(s: String) -> SyndicateTo {
        match s.as_ref() {
            "Homes.com" => SyndicateTo::Homescom,

            "ListHub" => SyndicateTo::ListHub,

            "Realtor.com" => SyndicateTo::Realtorcom,

            "Zillow/Trulia" => SyndicateTo::ZillowTrulia,

            _ => SyndicateTo::OpenEnumeration(s),
        }
    }
}

impl From<&str> for SyndicateTo {
    fn from(s: &str) -> SyndicateTo {
        match s {
            "Homes.com" => SyndicateTo::Homescom,

            "ListHub" => SyndicateTo::ListHub,

            "Realtor.com" => SyndicateTo::Realtorcom,

            "Zillow/Trulia" => SyndicateTo::ZillowTrulia,

            _ => SyndicateTo::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a SyndicateTo> for &'a str {
    fn from(s: &'a SyndicateTo) -> &'a str {
        match s {
            SyndicateTo::Homescom => "Homes.com",

            SyndicateTo::ListHub => "ListHub",

            SyndicateTo::Realtorcom => "Realtor.com",

            SyndicateTo::ZillowTrulia => "Zillow/Trulia",

            SyndicateTo::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for SyndicateTo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SyndicateTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_syndicate_to_format {
    use super::SyndicateTo;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<SyndicateTo>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<SyndicateTo>>, D::Error>
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
