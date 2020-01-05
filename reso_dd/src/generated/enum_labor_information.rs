// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LaborInformation Lookups](https://ddwiki.reso.org/display/DDW17/LaborInformation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LaborInformation {
    /// "[Employee License Required](https://ddwiki.reso.org/display/DDW17/Employee+License+Required)": Special licensing is required for employees.
    EmployeeLicenseRequired,

    /// "[Non-Union](https://ddwiki.reso.org/display/DDW17/Non-Union)": A labor union(s) are not currently established with the given business.
    NonUnion,

    /// "[Union](https://ddwiki.reso.org/display/DDW17/Union)": A labor union(s) are established with the given business.
    Union,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LaborInformation {
    fn from_str(s: &str) -> LaborInformation {
        match s {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LaborInformation {
        match s.as_ref() {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LaborInformation::EmployeeLicenseRequired => "Employee License Required",

            LaborInformation::NonUnion => "Non-Union",

            LaborInformation::Union => "Union",

            LaborInformation::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LaborInformation::EmployeeLicenseRequired => "Employee License Required".into(),

            LaborInformation::NonUnion => "Non-Union".into(),

            LaborInformation::Union => "Union".into(),

            LaborInformation::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LaborInformation::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LaborInformation {
    fn from(s: String) -> LaborInformation {
        match s.as_ref() {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LaborInformation {
    fn from(s: &str) -> LaborInformation {
        match s {
            "Employee License Required" => LaborInformation::EmployeeLicenseRequired,

            "Non-Union" => LaborInformation::NonUnion,

            "Union" => LaborInformation::Union,

            _ => LaborInformation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LaborInformation> for &'a str {
    fn from(s: &'a LaborInformation) -> &'a str {
        match s {
            LaborInformation::EmployeeLicenseRequired => "Employee License Required",

            LaborInformation::NonUnion => "Non-Union",

            LaborInformation::Union => "Union",

            LaborInformation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LaborInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LaborInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
