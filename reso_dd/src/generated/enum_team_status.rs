// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TeamStatus Lookups](https://ddwiki.reso.org/display/DDW17/TeamStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TeamStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246521)": The team is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246522)": The team is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for TeamStatus {
    fn from(s: String) -> TeamStatus {
        match s.as_ref() {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TeamStatus {
    fn from(s: &str) -> TeamStatus {
        match s {
            "Active" => TeamStatus::Active,

            "Inactive" => TeamStatus::Inactive,

            _ => TeamStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TeamStatus> for &'a str {
    fn from(s: &'a TeamStatus) -> &'a str {
        match s {
            TeamStatus::Active => "Active",

            TeamStatus::Inactive => "Inactive",

            TeamStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TeamStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TeamStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_team_status_format {
    use super::TeamStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<TeamStatus>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<TeamStatus>>, D::Error>
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
