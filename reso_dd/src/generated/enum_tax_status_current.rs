// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [TaxStatusCurrent Lookups](https://ddwiki.reso.org/display/DDW17/TaxStatusCurrent+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TaxStatusCurrent {
    /// "[Personal](https://ddwiki.reso.org/display/DDW17/Personal)": The tax is based on personal property.
    Personal,

    /// "[Personal And Real](https://ddwiki.reso.org/display/DDW17/Personal+And+Real)": The tax is based on both personal and real property.
    PersonalAndReal,

    /// "[Real](https://ddwiki.reso.org/display/DDW17/Real)": The tax is based on real property.
    Real,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for TaxStatusCurrent {
    fn from_str(s: &str) -> TaxStatusCurrent {
        match s {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> TaxStatusCurrent {
        match s.as_ref() {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            TaxStatusCurrent::Personal => "Personal",

            TaxStatusCurrent::PersonalAndReal => "Personal And Real",

            TaxStatusCurrent::Real => "Real",

            TaxStatusCurrent::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            TaxStatusCurrent::Personal => "Personal".into(),

            TaxStatusCurrent::PersonalAndReal => "Personal And Real".into(),

            TaxStatusCurrent::Real => "Real".into(),

            TaxStatusCurrent::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            TaxStatusCurrent::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for TaxStatusCurrent {
    fn from(s: String) -> TaxStatusCurrent {
        match s.as_ref() {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s),
        }
    }
}

impl From<&str> for TaxStatusCurrent {
    fn from(s: &str) -> TaxStatusCurrent {
        match s {
            "Personal" => TaxStatusCurrent::Personal,

            "Personal And Real" => TaxStatusCurrent::PersonalAndReal,

            "Real" => TaxStatusCurrent::Real,

            _ => TaxStatusCurrent::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a TaxStatusCurrent> for &'a str {
    fn from(s: &'a TaxStatusCurrent) -> &'a str {
        match s {
            TaxStatusCurrent::Personal => "Personal",

            TaxStatusCurrent::PersonalAndReal => "Personal And Real",

            TaxStatusCurrent::Real => "Real",

            TaxStatusCurrent::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for TaxStatusCurrent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for TaxStatusCurrent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
