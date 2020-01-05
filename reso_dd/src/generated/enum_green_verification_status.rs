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

impl crate::ResoEnumeration for GreenVerificationStatus {
    fn from_str(s: &str) -> GreenVerificationStatus {
        match s {
            "Complete" => GreenVerificationStatus::Complete,

            "In Process" => GreenVerificationStatus::InProcess,

            _ => GreenVerificationStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> GreenVerificationStatus {
        match s.as_ref() {
            "Complete" => GreenVerificationStatus::Complete,

            "In Process" => GreenVerificationStatus::InProcess,

            _ => GreenVerificationStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            GreenVerificationStatus::Complete => "Complete",

            GreenVerificationStatus::InProcess => "In Process",

            GreenVerificationStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            GreenVerificationStatus::Complete => "Complete".into(),

            GreenVerificationStatus::InProcess => "In Process".into(),

            GreenVerificationStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            GreenVerificationStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
