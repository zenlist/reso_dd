// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ShowingContactType Lookups](https://ddwiki.reso.org/display/DDW17/ShowingContactType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShowingContactType {
    /// "[Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246270)": The showing contact is a licensed agent.
    Agent,

    /// "[Occupant](https://ddwiki.reso.org/display/DDW17/Occupant)": The showing contact is the occupant.
    Occupant,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246272)": The showing contact is the owner.
    Owner,

    /// "[Property Manager](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246273)": The showing contact is the property manager.
    PropertyManager,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ShowingContactType {
    fn from(s: String) -> ShowingContactType {
        match s.as_ref() {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ShowingContactType {
    fn from(s: &str) -> ShowingContactType {
        match s {
            "Agent" => ShowingContactType::Agent,

            "Occupant" => ShowingContactType::Occupant,

            "Owner" => ShowingContactType::Owner,

            "Property Manager" => ShowingContactType::PropertyManager,

            _ => ShowingContactType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ShowingContactType> for &'a str {
    fn from(s: &'a ShowingContactType) -> &'a str {
        match s {
            ShowingContactType::Agent => "Agent",

            ShowingContactType::Occupant => "Occupant",

            ShowingContactType::Owner => "Owner",

            ShowingContactType::PropertyManager => "Property Manager",

            ShowingContactType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ShowingContactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ShowingContactType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_showing_contact_type_format {
    use super::ShowingContactType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ShowingContactType>>,
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
    ) -> Result<Option<Vec<ShowingContactType>>, D::Error>
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
