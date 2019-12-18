// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [MemberStatus Lookups](https://ddwiki.reso.org/display/DDW17/MemberStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemberStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245584)": The member's account is active.
    Active,

    /// "[Inactive](https://ddwiki.reso.org/display/DDW17/Inactive)": the member's account is not active.
    Inactive,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for MemberStatus {
    fn from(s: String) -> MemberStatus {
        match s.as_ref() {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for MemberStatus {
    fn from(s: &str) -> MemberStatus {
        match s {
            "Active" => MemberStatus::Active,

            "Inactive" => MemberStatus::Inactive,

            _ => MemberStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a MemberStatus> for &'a str {
    fn from(s: &'a MemberStatus) -> &'a str {
        match s {
            MemberStatus::Active => "Active",

            MemberStatus::Inactive => "Inactive",

            MemberStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for MemberStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for MemberStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_member_status_format {
    use super::MemberStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<MemberStatus>>,
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
    ) -> Result<Option<Vec<MemberStatus>>, D::Error>
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
