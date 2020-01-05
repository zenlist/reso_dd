// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ContactListingPreference Lookups](https://ddwiki.reso.org/display/DDW17/ContactListingPreference+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContactListingPreference {
    /// "[Discard](https://ddwiki.reso.org/display/DDW17/Discard)": The contact has flagged to discard the given listing. The contact is not interested in the given listing.
    Discard,

    /// "[Favorite](https://ddwiki.reso.org/display/DDW17/Favorite)": The contact has flagged the given listing as a favorite. The contact is interested in the given listing.
    Favorite,

    /// "[Possibility](https://ddwiki.reso.org/display/DDW17/Possibility)": The contact has flagged the given listing as a possibility. The contact might be interested in the given listing.
    Possibility,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ContactListingPreference {
    fn from_str(s: &str) -> ContactListingPreference {
        match s {
            "Discard" => ContactListingPreference::Discard,

            "Favorite" => ContactListingPreference::Favorite,

            "Possibility" => ContactListingPreference::Possibility,

            _ => ContactListingPreference::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ContactListingPreference {
        match s.as_ref() {
            "Discard" => ContactListingPreference::Discard,

            "Favorite" => ContactListingPreference::Favorite,

            "Possibility" => ContactListingPreference::Possibility,

            _ => ContactListingPreference::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ContactListingPreference::Discard => "Discard",

            ContactListingPreference::Favorite => "Favorite",

            ContactListingPreference::Possibility => "Possibility",

            ContactListingPreference::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ContactListingPreference::Discard => "Discard".into(),

            ContactListingPreference::Favorite => "Favorite".into(),

            ContactListingPreference::Possibility => "Possibility".into(),

            ContactListingPreference::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ContactListingPreference::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ContactListingPreference {
    fn from(s: String) -> ContactListingPreference {
        match s.as_ref() {
            "Discard" => ContactListingPreference::Discard,

            "Favorite" => ContactListingPreference::Favorite,

            "Possibility" => ContactListingPreference::Possibility,

            _ => ContactListingPreference::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ContactListingPreference {
    fn from(s: &str) -> ContactListingPreference {
        match s {
            "Discard" => ContactListingPreference::Discard,

            "Favorite" => ContactListingPreference::Favorite,

            "Possibility" => ContactListingPreference::Possibility,

            _ => ContactListingPreference::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ContactListingPreference> for &'a str {
    fn from(s: &'a ContactListingPreference) -> &'a str {
        match s {
            ContactListingPreference::Discard => "Discard",

            ContactListingPreference::Favorite => "Favorite",

            ContactListingPreference::Possibility => "Possibility",

            ContactListingPreference::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ContactListingPreference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ContactListingPreference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
