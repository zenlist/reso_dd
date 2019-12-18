// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Attended Lookups](https://ddwiki.reso.org/display/DDW17/Attended+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Attended {
    /// "[Agent](https://ddwiki.reso.org/display/DDW17/Agent)": A licensed real estate agent will be present at the open house event.
    Agent,

    /// "[Seller](https://ddwiki.reso.org/display/DDW17/Seller)": A licensed real estate agent will not be present and the property owner will be present at the open house event.
    Seller,

    /// "[Unattended](https://ddwiki.reso.org/display/DDW17/Unattended)": The open house event will not be attended.  Access will normally be via lockbox or other pre-arranged means.
    Unattended,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Attended {
    fn from(s: String) -> Attended {
        match s.as_ref() {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Attended {
    fn from(s: &str) -> Attended {
        match s {
            "Agent" => Attended::Agent,

            "Seller" => Attended::Seller,

            "Unattended" => Attended::Unattended,

            _ => Attended::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Attended> for &'a str {
    fn from(s: &'a Attended) -> &'a str {
        match s {
            Attended::Agent => "Agent",

            Attended::Seller => "Seller",

            Attended::Unattended => "Unattended",

            Attended::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Attended {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Attended {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_attended_format {
    use super::Attended;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Attended>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Attended>>, D::Error>
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
