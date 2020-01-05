// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LinearUnits Lookups](https://ddwiki.reso.org/display/DDW17/LinearUnits+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinearUnits {
    /// "[Feet](https://ddwiki.reso.org/display/DDW17/Feet)": The elevation of the property is measured in feet.
    Feet,

    /// "[Meters](https://ddwiki.reso.org/display/DDW17/Meters)": The elevation of the property is measured in meters.
    Meters,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LinearUnits {
    fn from_str(s: &str) -> LinearUnits {
        match s {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LinearUnits {
        match s.as_ref() {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LinearUnits::Feet => "Feet",

            LinearUnits::Meters => "Meters",

            LinearUnits::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LinearUnits::Feet => "Feet".into(),

            LinearUnits::Meters => "Meters".into(),

            LinearUnits::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LinearUnits::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LinearUnits {
    fn from(s: String) -> LinearUnits {
        match s.as_ref() {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LinearUnits {
    fn from(s: &str) -> LinearUnits {
        match s {
            "Feet" => LinearUnits::Feet,

            "Meters" => LinearUnits::Meters,

            _ => LinearUnits::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LinearUnits> for &'a str {
    fn from(s: &'a LinearUnits) -> &'a str {
        match s {
            LinearUnits::Feet => "Feet",

            LinearUnits::Meters => "Meters",

            LinearUnits::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LinearUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LinearUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
