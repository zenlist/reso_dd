// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OfficeStatus Lookups](https://ddwiki.reso.org/display/DDW17/OfficeStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OfficeStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245650)": The member office's account is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245651)": The member office's account is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OfficeStatus {
    fn from(s: String) -> OfficeStatus {
        match s.as_ref() {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OfficeStatus {
    fn from(s: &str) -> OfficeStatus {
        match s {
            "Active" => OfficeStatus::Active,

            "Inactive" => OfficeStatus::Inactive,

            _ => OfficeStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OfficeStatus> for &'a str {
    fn from(s: &'a OfficeStatus) -> &'a str {
        match s {
            OfficeStatus::Active => "Active",

            OfficeStatus::Inactive => "Inactive",

            OfficeStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OfficeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OfficeStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_office_status_format {
    use super::OfficeStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OfficeStatus>>,
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
    ) -> Result<Option<Vec<OfficeStatus>>, D::Error>
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
