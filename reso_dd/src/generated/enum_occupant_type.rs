// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OccupantType Lookups](https://ddwiki.reso.org/display/DDW17/OccupantType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OccupantType {
    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245646)": The occupant is the owner.
    Owner,

    /// "[Tenant](https://ddwiki.reso.org/display/DDW17/Tenant)": The occupant is a tenant.
    Tenant,

    /// "[Vacant](https://ddwiki.reso.org/display/DDW17/Vacant)": The property is vacant.
    Vacant,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OccupantType {
    fn from(s: String) -> OccupantType {
        match s.as_ref() {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OccupantType {
    fn from(s: &str) -> OccupantType {
        match s {
            "Owner" => OccupantType::Owner,

            "Tenant" => OccupantType::Tenant,

            "Vacant" => OccupantType::Vacant,

            _ => OccupantType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OccupantType> for &'a str {
    fn from(s: &'a OccupantType) -> &'a str {
        match s {
            OccupantType::Owner => "Owner",

            OccupantType::Tenant => "Tenant",

            OccupantType::Vacant => "Vacant",

            OccupantType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OccupantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OccupantType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_occupant_type_format {
    use super::OccupantType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OccupantType>>,
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
    ) -> Result<Option<Vec<OccupantType>>, D::Error>
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
