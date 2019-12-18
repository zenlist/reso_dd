// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OwnershipType Lookups](https://ddwiki.reso.org/display/DDW17/OwnershipType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OwnershipType {
    /// "[Corporation](https://ddwiki.reso.org/display/DDW17/Corporation)": The ownership type of the business being sold is a corporation.
    Corporation,

    /// "[LLC](https://ddwiki.reso.org/display/DDW17/LLC)": The ownership type of the business being sold is a limited liability corporation.
    LLC,

    /// "[Partnership](https://ddwiki.reso.org/display/DDW17/Partnership)": The ownership type of the business being sold is a partnership.
    Partnership,

    /// "[Sole Proprietor](https://ddwiki.reso.org/display/DDW17/Sole+Proprietor)": The ownership type of the business being sold is a sole proprietor.
    SoleProprietor,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OwnershipType {
    fn from(s: String) -> OwnershipType {
        match s.as_ref() {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OwnershipType {
    fn from(s: &str) -> OwnershipType {
        match s {
            "Corporation" => OwnershipType::Corporation,

            "LLC" => OwnershipType::LLC,

            "Partnership" => OwnershipType::Partnership,

            "Sole Proprietor" => OwnershipType::SoleProprietor,

            _ => OwnershipType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OwnershipType> for &'a str {
    fn from(s: &'a OwnershipType) -> &'a str {
        match s {
            OwnershipType::Corporation => "Corporation",

            OwnershipType::LLC => "LLC",

            OwnershipType::Partnership => "Partnership",

            OwnershipType::SoleProprietor => "Sole Proprietor",

            OwnershipType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OwnershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OwnershipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_ownership_type_format {
    use super::OwnershipType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OwnershipType>>,
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
    ) -> Result<Option<Vec<OwnershipType>>, D::Error>
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
