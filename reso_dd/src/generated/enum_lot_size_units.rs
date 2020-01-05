// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LotSizeUnits Lookups](https://ddwiki.reso.org/display/DDW17/LotSizeUnits+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LotSizeUnits {
    /// "[Acres](https://ddwiki.reso.org/display/DDW17/Acres)": The value reported in the Lot Size Area field is in acres.
    Acres,

    /// "[Square Feet](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245429)": The value reported in the Lot Size Area field is in square feet.
    SquareFeet,

    /// "[Square Meters](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245430)": The value reported in the Lot Size Area field is in square meters.
    SquareMeters,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LotSizeUnits {
    fn from_str(s: &str) -> LotSizeUnits {
        match s {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LotSizeUnits {
        match s.as_ref() {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LotSizeUnits::Acres => "Acres",

            LotSizeUnits::SquareFeet => "Square Feet",

            LotSizeUnits::SquareMeters => "Square Meters",

            LotSizeUnits::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LotSizeUnits::Acres => "Acres".into(),

            LotSizeUnits::SquareFeet => "Square Feet".into(),

            LotSizeUnits::SquareMeters => "Square Meters".into(),

            LotSizeUnits::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LotSizeUnits::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LotSizeUnits {
    fn from(s: String) -> LotSizeUnits {
        match s.as_ref() {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LotSizeUnits {
    fn from(s: &str) -> LotSizeUnits {
        match s {
            "Acres" => LotSizeUnits::Acres,

            "Square Feet" => LotSizeUnits::SquareFeet,

            "Square Meters" => LotSizeUnits::SquareMeters,

            _ => LotSizeUnits::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LotSizeUnits> for &'a str {
    fn from(s: &'a LotSizeUnits) -> &'a str {
        match s {
            LotSizeUnits::Acres => "Acres",

            LotSizeUnits::SquareFeet => "Square Feet",

            LotSizeUnits::SquareMeters => "Square Meters",

            LotSizeUnits::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LotSizeUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LotSizeUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
