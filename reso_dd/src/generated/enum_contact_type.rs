// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ContactType Lookups](https://ddwiki.reso.org/display/DDW17/ContactType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContactType {
    /// "[Business](https://ddwiki.reso.org/display/DDW17/Business)": The contact is a business relation.
    Business,

    /// "[Family](https://ddwiki.reso.org/display/DDW17/Family)": The contact is a family member.
    Family,

    /// "[Friend](https://ddwiki.reso.org/display/DDW17/Friend)": The contact is a personal friend.
    Friend,

    /// "[Lead](https://ddwiki.reso.org/display/DDW17/Lead)": The lead is a contact that may be a potential buyer or seller to the member.
    Lead,

    /// "[Prospect](https://ddwiki.reso.org/display/DDW17/Prospect)": The contact is a prospective client.
    Prospect,

    /// "[Ready to Buy](https://ddwiki.reso.org/display/DDW17/Ready+to+Buy)": The contact is a client who is ready to start a transaction.
    ReadytoBuy,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ContactType {
    fn from_str(s: &str) -> ContactType {
        match s {
            "Business" => ContactType::Business,

            "Family" => ContactType::Family,

            "Friend" => ContactType::Friend,

            "Lead" => ContactType::Lead,

            "Prospect" => ContactType::Prospect,

            "Ready to Buy" => ContactType::ReadytoBuy,

            _ => ContactType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ContactType {
        match s.as_ref() {
            "Business" => ContactType::Business,

            "Family" => ContactType::Family,

            "Friend" => ContactType::Friend,

            "Lead" => ContactType::Lead,

            "Prospect" => ContactType::Prospect,

            "Ready to Buy" => ContactType::ReadytoBuy,

            _ => ContactType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ContactType::Business => "Business",

            ContactType::Family => "Family",

            ContactType::Friend => "Friend",

            ContactType::Lead => "Lead",

            ContactType::Prospect => "Prospect",

            ContactType::ReadytoBuy => "Ready to Buy",

            ContactType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ContactType::Business => "Business".into(),

            ContactType::Family => "Family".into(),

            ContactType::Friend => "Friend".into(),

            ContactType::Lead => "Lead".into(),

            ContactType::Prospect => "Prospect".into(),

            ContactType::ReadytoBuy => "Ready to Buy".into(),

            ContactType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ContactType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ContactType {
    fn from(s: String) -> ContactType {
        match s.as_ref() {
            "Business" => ContactType::Business,

            "Family" => ContactType::Family,

            "Friend" => ContactType::Friend,

            "Lead" => ContactType::Lead,

            "Prospect" => ContactType::Prospect,

            "Ready to Buy" => ContactType::ReadytoBuy,

            _ => ContactType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ContactType {
    fn from(s: &str) -> ContactType {
        match s {
            "Business" => ContactType::Business,

            "Family" => ContactType::Family,

            "Friend" => ContactType::Friend,

            "Lead" => ContactType::Lead,

            "Prospect" => ContactType::Prospect,

            "Ready to Buy" => ContactType::ReadytoBuy,

            _ => ContactType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ContactType> for &'a str {
    fn from(s: &'a ContactType) -> &'a str {
        match s {
            ContactType::Business => "Business",

            ContactType::Family => "Family",

            ContactType::Friend => "Friend",

            ContactType::Lead => "Lead",

            ContactType::Prospect => "Prospect",

            ContactType::ReadytoBuy => "Ready to Buy",

            ContactType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ContactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ContactType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
