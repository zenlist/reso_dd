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

pub(crate) mod option_vec_listing_service_format {
    use super::ListingService;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ListingService>>,
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
    ) -> Result<Option<Vec<ListingService>>, D::Error>
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
