// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Concessions Lookups](https://ddwiki.reso.org/display/DDW17/Concessions+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Concessions {
    /// "[Call Listing Agent](https://ddwiki.reso.org/display/DDW17/Call+Listing+Agent)": Call the listing agent for information about concessions made/offered by the seller.
    CallListingAgent,

    /// "[No](https://ddwiki.reso.org/display/DDW17/No)": There are no concessions included with this listing.
    No,

    /// "[Yes](https://ddwiki.reso.org/display/DDW17/Yes)": There are concessions that are part of the listing/sale.
    Yes,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Concessions {
    fn from(s: String) -> Concessions {
        match s.as_ref() {
            "Call Listing Agent" => Concessions::CallListingAgent,

            "No" => Concessions::No,

            "Yes" => Concessions::Yes,

            _ => Concessions::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Concessions {
    fn from(s: &str) -> Concessions {
        match s {
            "Call Listing Agent" => Concessions::CallListingAgent,

            "No" => Concessions::No,

            "Yes" => Concessions::Yes,

            _ => Concessions::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Concessions> for &'a str {
    fn from(s: &'a Concessions) -> &'a str {
        match s {
            Concessions::CallListingAgent => "Call Listing Agent",

            Concessions::No => "No",

            Concessions::Yes => "Yes",

            Concessions::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Concessions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Concessions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_concessions_format {
    use super::Concessions;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Concessions>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Concessions>>, D::Error>
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
