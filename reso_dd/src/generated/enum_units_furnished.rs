// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [UnitsFurnished Lookups](https://ddwiki.reso.org/display/DDW17/UnitsFurnished+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitsFurnished {
    /// "[All Units](https://ddwiki.reso.org/display/DDW17/All+Units)": All of the units in the listed income property are furnished.
    AllUnits,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246595)": None of the units in the listed income property are furnished.
    None,

    /// "[Varies By Unit](https://ddwiki.reso.org/display/DDW17/Varies+By+Unit)": Some of the units in the listing income property are furnished.
    VariesByUnit,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for UnitsFurnished {
    fn from_str(s: &str) -> UnitsFurnished {
        match s {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> UnitsFurnished {
        match s.as_ref() {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            UnitsFurnished::AllUnits => "All Units",

            UnitsFurnished::None => "None",

            UnitsFurnished::VariesByUnit => "Varies By Unit",

            UnitsFurnished::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            UnitsFurnished::AllUnits => "All Units".into(),

            UnitsFurnished::None => "None".into(),

            UnitsFurnished::VariesByUnit => "Varies By Unit".into(),

            UnitsFurnished::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            UnitsFurnished::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for UnitsFurnished {
    fn from(s: String) -> UnitsFurnished {
        match s.as_ref() {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s),
        }
    }
}

impl From<&str> for UnitsFurnished {
    fn from(s: &str) -> UnitsFurnished {
        match s {
            "All Units" => UnitsFurnished::AllUnits,

            "None" => UnitsFurnished::None,

            "Varies By Unit" => UnitsFurnished::VariesByUnit,

            _ => UnitsFurnished::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a UnitsFurnished> for &'a str {
    fn from(s: &'a UnitsFurnished) -> &'a str {
        match s {
            UnitsFurnished::AllUnits => "All Units",

            UnitsFurnished::None => "None",

            UnitsFurnished::VariesByUnit => "Varies By Unit",

            UnitsFurnished::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for UnitsFurnished {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for UnitsFurnished {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
