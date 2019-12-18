// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ContactStatus Lookups](https://ddwiki.reso.org/display/DDW17/ContactStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContactStatus {
    /// "[Active](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244403)": The contact is active.
    Active,

    /// "[Deleted](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244404)": The contact has been deleted.
    Deleted,

    /// "[Inactive](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244405)": The contact is no longer active.
    Inactive,

    /// "[On Vacation](https://ddwiki.reso.org/display/DDW17/On+Vacation)": The contact is on vacation.
    OnVacation,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ContactStatus {
    fn from(s: String) -> ContactStatus {
        match s.as_ref() {
            "Active" => ContactStatus::Active,

            "Deleted" => ContactStatus::Deleted,

            "Inactive" => ContactStatus::Inactive,

            "On Vacation" => ContactStatus::OnVacation,

            _ => ContactStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ContactStatus {
    fn from(s: &str) -> ContactStatus {
        match s {
            "Active" => ContactStatus::Active,

            "Deleted" => ContactStatus::Deleted,

            "Inactive" => ContactStatus::Inactive,

            "On Vacation" => ContactStatus::OnVacation,

            _ => ContactStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ContactStatus> for &'a str {
    fn from(s: &'a ContactStatus) -> &'a str {
        match s {
            ContactStatus::Active => "Active",

            ContactStatus::Deleted => "Deleted",

            ContactStatus::Inactive => "Inactive",

            ContactStatus::OnVacation => "On Vacation",

            ContactStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ContactStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ContactStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_contact_status_format {
    use super::ContactStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ContactStatus>>,
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
    ) -> Result<Option<Vec<ContactStatus>>, D::Error>
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
