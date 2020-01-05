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

impl crate::ResoEnumeration for Concessions {
    fn from_str(s: &str) -> Concessions {
        match s {
            "Call Listing Agent" => Concessions::CallListingAgent,

            "No" => Concessions::No,

            "Yes" => Concessions::Yes,

            _ => Concessions::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Concessions {
        match s.as_ref() {
            "Call Listing Agent" => Concessions::CallListingAgent,

            "No" => Concessions::No,

            "Yes" => Concessions::Yes,

            _ => Concessions::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Concessions::CallListingAgent => "Call Listing Agent",

            Concessions::No => "No",

            Concessions::Yes => "Yes",

            Concessions::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Concessions::CallListingAgent => "Call Listing Agent".into(),

            Concessions::No => "No".into(),

            Concessions::Yes => "Yes".into(),

            Concessions::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Concessions::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
