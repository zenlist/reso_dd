// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenVerificationStatus Lookups](https://ddwiki.reso.org/display/DDW17/GreenVerificationStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenVerificationStatus {
    /// "[Complete](https://ddwiki.reso.org/display/DDW17/Complete)": Indicates that verification process is complete.  All requirements are complete and official verification documentation is on file or published; or more than 12 months of occupancy.
    Complete,

    /// "[In Process](https://ddwiki.reso.org/display/DDW17/In+Process)": Indicates that verification process is underway, but not complete.  Application, plans testing or specifications in process; or may be less than 12 months of occupancy.
    InProcess,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenVerificationStatus {
    fn from(s: String) -> GreenVerificationStatus {
        match s.as_ref() {
            "Complete" => GreenVerificationStatus::Complete,

            "In Process" => GreenVerificationStatus::InProcess,

            _ => GreenVerificationStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenVerificationStatus {
    fn from(s: &str) -> GreenVerificationStatus {
        match s {
            "Complete" => GreenVerificationStatus::Complete,

            "In Process" => GreenVerificationStatus::InProcess,

            _ => GreenVerificationStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenVerificationStatus> for &'a str {
    fn from(s: &'a GreenVerificationStatus) -> &'a str {
        match s {
            GreenVerificationStatus::Complete => "Complete",

            GreenVerificationStatus::InProcess => "In Process",

            GreenVerificationStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenVerificationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_verification_status_format {
    use super::GreenVerificationStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenVerificationStatus>>,
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
    ) -> Result<Option<Vec<GreenVerificationStatus>>, D::Error>
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
