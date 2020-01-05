// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ListingService Lookups](https://ddwiki.reso.org/display/DDW17/ListingService+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ListingService {
    /// "[Entry Only](https://ddwiki.reso.org/display/DDW17/Entry+Only)": The only service provided by the brokerage is the input of the listing into the MLS system.
    EntryOnly,

    /// "[Full Service](https://ddwiki.reso.org/display/DDW17/Full+Service)": A full set of services offered by a brokerage.
    FullService,

    /// "[Limited Service](https://ddwiki.reso.org/display/DDW17/Limited+Service)": A limited set of services offered by a brokerage
    LimitedService,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ListingService {
    fn from_str(s: &str) -> ListingService {
        match s {
            "Entry Only" => ListingService::EntryOnly,

            "Full Service" => ListingService::FullService,

            "Limited Service" => ListingService::LimitedService,

            _ => ListingService::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ListingService {
        match s.as_ref() {
            "Entry Only" => ListingService::EntryOnly,

            "Full Service" => ListingService::FullService,

            "Limited Service" => ListingService::LimitedService,

            _ => ListingService::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ListingService::EntryOnly => "Entry Only",

            ListingService::FullService => "Full Service",

            ListingService::LimitedService => "Limited Service",

            ListingService::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ListingService::EntryOnly => "Entry Only".into(),

            ListingService::FullService => "Full Service".into(),

            ListingService::LimitedService => "Limited Service".into(),

            ListingService::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ListingService::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ListingService {
    fn from(s: String) -> ListingService {
        match s.as_ref() {
            "Entry Only" => ListingService::EntryOnly,

            "Full Service" => ListingService::FullService,

            "Limited Service" => ListingService::LimitedService,

            _ => ListingService::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ListingService {
    fn from(s: &str) -> ListingService {
        match s {
            "Entry Only" => ListingService::EntryOnly,

            "Full Service" => ListingService::FullService,

            "Limited Service" => ListingService::LimitedService,

            _ => ListingService::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ListingService> for &'a str {
    fn from(s: &'a ListingService) -> &'a str {
        match s {
            ListingService::EntryOnly => "Entry Only",

            ListingService::FullService => "Full Service",

            ListingService::LimitedService => "Limited Service",

            ListingService::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ListingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ListingService {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
