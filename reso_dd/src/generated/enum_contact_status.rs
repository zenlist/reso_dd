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

impl crate::ResoEnumeration for ContactStatus {
    fn from_str(s: &str) -> ContactStatus {
        match s {
            "Active" => ContactStatus::Active,

            "Deleted" => ContactStatus::Deleted,

            "Inactive" => ContactStatus::Inactive,

            "On Vacation" => ContactStatus::OnVacation,

            _ => ContactStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ContactStatus {
        match s.as_ref() {
            "Active" => ContactStatus::Active,

            "Deleted" => ContactStatus::Deleted,

            "Inactive" => ContactStatus::Inactive,

            "On Vacation" => ContactStatus::OnVacation,

            _ => ContactStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ContactStatus::Active => "Active",

            ContactStatus::Deleted => "Deleted",

            ContactStatus::Inactive => "Inactive",

            ContactStatus::OnVacation => "On Vacation",

            ContactStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ContactStatus::Active => "Active".into(),

            ContactStatus::Deleted => "Deleted".into(),

            ContactStatus::Inactive => "Inactive".into(),

            ContactStatus::OnVacation => "On Vacation".into(),

            ContactStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ContactStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
