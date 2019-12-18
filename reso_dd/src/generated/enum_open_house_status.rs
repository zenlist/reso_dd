// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OpenHouseStatus Lookups](https://ddwiki.reso.org/display/DDW17/OpenHouseStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenHouseStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245665)": The open house is active and continuing as scheduled.
    Active,

    /// "[Canceled](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245666)": The open house has been canceled.
    Canceled,

    /// "[Ended](https://ddwiki.reso.org/display/DDW17/Ended)": The open house has ended and is past the scheduled open house date and time.
    Ended,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OpenHouseStatus {
    fn from(s: String) -> OpenHouseStatus {
        match s.as_ref() {
            "Active" => OpenHouseStatus::Active,

            "Canceled" => OpenHouseStatus::Canceled,

            "Ended" => OpenHouseStatus::Ended,

            _ => OpenHouseStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OpenHouseStatus {
    fn from(s: &str) -> OpenHouseStatus {
        match s {
            "Active" => OpenHouseStatus::Active,

            "Canceled" => OpenHouseStatus::Canceled,

            "Ended" => OpenHouseStatus::Ended,

            _ => OpenHouseStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OpenHouseStatus> for &'a str {
    fn from(s: &'a OpenHouseStatus) -> &'a str {
        match s {
            OpenHouseStatus::Active => "Active",

            OpenHouseStatus::Canceled => "Canceled",

            OpenHouseStatus::Ended => "Ended",

            OpenHouseStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OpenHouseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OpenHouseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_open_house_status_format {
    use super::OpenHouseStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OpenHouseStatus>>,
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
    ) -> Result<Option<Vec<OpenHouseStatus>>, D::Error>
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
