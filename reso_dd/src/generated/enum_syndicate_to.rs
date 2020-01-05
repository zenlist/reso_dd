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

impl crate::ResoEnumeration for SyndicateTo {
    fn from_str(s: &str) -> SyndicateTo {
        match s {
            "Homes.com" => SyndicateTo::Homescom,

            "ListHub" => SyndicateTo::ListHub,

            "Realtor.com" => SyndicateTo::Realtorcom,

            "Zillow/Trulia" => SyndicateTo::ZillowTrulia,

            _ => SyndicateTo::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> SyndicateTo {
        match s.as_ref() {
            "Homes.com" => SyndicateTo::Homescom,

            "ListHub" => SyndicateTo::ListHub,

            "Realtor.com" => SyndicateTo::Realtorcom,

            "Zillow/Trulia" => SyndicateTo::ZillowTrulia,

            _ => SyndicateTo::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            SyndicateTo::Homescom => "Homes.com",

            SyndicateTo::ListHub => "ListHub",

            SyndicateTo::Realtorcom => "Realtor.com",

            SyndicateTo::ZillowTrulia => "Zillow/Trulia",

            SyndicateTo::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            SyndicateTo::Homescom => "Homes.com".into(),

            SyndicateTo::ListHub => "ListHub".into(),

            SyndicateTo::Realtorcom => "Realtor.com".into(),

            SyndicateTo::ZillowTrulia => "Zillow/Trulia".into(),

            SyndicateTo::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            SyndicateTo::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
