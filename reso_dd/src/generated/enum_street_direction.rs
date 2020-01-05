// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [StreetDirection Lookups](https://ddwiki.reso.org/display/DDW17/StreetDirection+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StreetDirection {
    /// "[E](https://ddwiki.reso.org/display/DDW17/E)": The street suffix or prefix direction is East.
    E,

    /// "[N](https://ddwiki.reso.org/display/DDW17/N)": The street suffix or prefix direction is North.
    N,

    /// "[NE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246400)": The street suffix or prefix direction is North East.
    NE,

    /// "[NW](https://ddwiki.reso.org/display/DDW17/NW)": The street suffix or prefix direction is North West.
    NW,

    /// "[S](https://ddwiki.reso.org/display/DDW17/S)": The street suffix or prefix direction is South.
    S,

    /// "[SE](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246403)": The street suffix or prefix direction is South East.
    SE,

    /// "[SW](https://ddwiki.reso.org/display/DDW17/SW)": The street suffix or prefix direction is South West.
    SW,

    /// "[W](https://ddwiki.reso.org/display/DDW17/W)": The street suffix or prefix direction is West.
    W,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for StreetDirection {
    fn from_str(s: &str) -> StreetDirection {
        match s {
            "E" => StreetDirection::E,

            "N" => StreetDirection::N,

            "NE" => StreetDirection::NE,

            "NW" => StreetDirection::NW,

            "S" => StreetDirection::S,

            "SE" => StreetDirection::SE,

            "SW" => StreetDirection::SW,

            "W" => StreetDirection::W,

            _ => StreetDirection::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> StreetDirection {
        match s.as_ref() {
            "E" => StreetDirection::E,

            "N" => StreetDirection::N,

            "NE" => StreetDirection::NE,

            "NW" => StreetDirection::NW,

            "S" => StreetDirection::S,

            "SE" => StreetDirection::SE,

            "SW" => StreetDirection::SW,

            "W" => StreetDirection::W,

            _ => StreetDirection::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            StreetDirection::E => "E",

            StreetDirection::N => "N",

            StreetDirection::NE => "NE",

            StreetDirection::NW => "NW",

            StreetDirection::S => "S",

            StreetDirection::SE => "SE",

            StreetDirection::SW => "SW",

            StreetDirection::W => "W",

            StreetDirection::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            StreetDirection::E => "E".into(),

            StreetDirection::N => "N".into(),

            StreetDirection::NE => "NE".into(),

            StreetDirection::NW => "NW".into(),

            StreetDirection::S => "S".into(),

            StreetDirection::SE => "SE".into(),

            StreetDirection::SW => "SW".into(),

            StreetDirection::W => "W".into(),

            StreetDirection::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            StreetDirection::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for StreetDirection {
    fn from(s: String) -> StreetDirection {
        match s.as_ref() {
            "E" => StreetDirection::E,

            "N" => StreetDirection::N,

            "NE" => StreetDirection::NE,

            "NW" => StreetDirection::NW,

            "S" => StreetDirection::S,

            "SE" => StreetDirection::SE,

            "SW" => StreetDirection::SW,

            "W" => StreetDirection::W,

            _ => StreetDirection::OpenEnumeration(s),
        }
    }
}

impl From<&str> for StreetDirection {
    fn from(s: &str) -> StreetDirection {
        match s {
            "E" => StreetDirection::E,

            "N" => StreetDirection::N,

            "NE" => StreetDirection::NE,

            "NW" => StreetDirection::NW,

            "S" => StreetDirection::S,

            "SE" => StreetDirection::SE,

            "SW" => StreetDirection::SW,

            "W" => StreetDirection::W,

            _ => StreetDirection::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a StreetDirection> for &'a str {
    fn from(s: &'a StreetDirection) -> &'a str {
        match s {
            StreetDirection::E => "E",

            StreetDirection::N => "N",

            StreetDirection::NE => "NE",

            StreetDirection::NW => "NW",

            StreetDirection::S => "S",

            StreetDirection::SE => "SE",

            StreetDirection::SW => "SW",

            StreetDirection::W => "W",

            StreetDirection::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for StreetDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for StreetDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
