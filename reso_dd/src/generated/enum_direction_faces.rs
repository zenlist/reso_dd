// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [DirectionFaces Lookups](https://ddwiki.reso.org/display/DDW17/DirectionFaces+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DirectionFaces {
    /// "[East](https://ddwiki.reso.org/display/DDW17/East)": The front of the structure faces east.
    East,

    /// "[North](https://ddwiki.reso.org/display/DDW17/North)": The front of the structure faces north.
    North,

    /// "[Northeast](https://ddwiki.reso.org/display/DDW17/Northeast)": The front of the structure faces northeast.
    Northeast,

    /// "[Northwest](https://ddwiki.reso.org/display/DDW17/Northwest)": The front of the structure faces northwest.
    Northwest,

    /// "[South](https://ddwiki.reso.org/display/DDW17/South)": The front of the structure faces south.
    South,

    /// "[Southeast](https://ddwiki.reso.org/display/DDW17/Southeast)": The front of the structure faces southeast.
    Southeast,

    /// "[Southwest](https://ddwiki.reso.org/display/DDW17/Southwest)": The front of the structure faces southwest.
    Southwest,

    /// "[West](https://ddwiki.reso.org/display/DDW17/West)": The front of the structure faces west.
    West,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for DirectionFaces {
    fn from_str(s: &str) -> DirectionFaces {
        match s {
            "East" => DirectionFaces::East,

            "North" => DirectionFaces::North,

            "Northeast" => DirectionFaces::Northeast,

            "Northwest" => DirectionFaces::Northwest,

            "South" => DirectionFaces::South,

            "Southeast" => DirectionFaces::Southeast,

            "Southwest" => DirectionFaces::Southwest,

            "West" => DirectionFaces::West,

            _ => DirectionFaces::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> DirectionFaces {
        match s.as_ref() {
            "East" => DirectionFaces::East,

            "North" => DirectionFaces::North,

            "Northeast" => DirectionFaces::Northeast,

            "Northwest" => DirectionFaces::Northwest,

            "South" => DirectionFaces::South,

            "Southeast" => DirectionFaces::Southeast,

            "Southwest" => DirectionFaces::Southwest,

            "West" => DirectionFaces::West,

            _ => DirectionFaces::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            DirectionFaces::East => "East",

            DirectionFaces::North => "North",

            DirectionFaces::Northeast => "Northeast",

            DirectionFaces::Northwest => "Northwest",

            DirectionFaces::South => "South",

            DirectionFaces::Southeast => "Southeast",

            DirectionFaces::Southwest => "Southwest",

            DirectionFaces::West => "West",

            DirectionFaces::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            DirectionFaces::East => "East".into(),

            DirectionFaces::North => "North".into(),

            DirectionFaces::Northeast => "Northeast".into(),

            DirectionFaces::Northwest => "Northwest".into(),

            DirectionFaces::South => "South".into(),

            DirectionFaces::Southeast => "Southeast".into(),

            DirectionFaces::Southwest => "Southwest".into(),

            DirectionFaces::West => "West".into(),

            DirectionFaces::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            DirectionFaces::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for DirectionFaces {
    fn from(s: String) -> DirectionFaces {
        match s.as_ref() {
            "East" => DirectionFaces::East,

            "North" => DirectionFaces::North,

            "Northeast" => DirectionFaces::Northeast,

            "Northwest" => DirectionFaces::Northwest,

            "South" => DirectionFaces::South,

            "Southeast" => DirectionFaces::Southeast,

            "Southwest" => DirectionFaces::Southwest,

            "West" => DirectionFaces::West,

            _ => DirectionFaces::OpenEnumeration(s),
        }
    }
}

impl From<&str> for DirectionFaces {
    fn from(s: &str) -> DirectionFaces {
        match s {
            "East" => DirectionFaces::East,

            "North" => DirectionFaces::North,

            "Northeast" => DirectionFaces::Northeast,

            "Northwest" => DirectionFaces::Northwest,

            "South" => DirectionFaces::South,

            "Southeast" => DirectionFaces::Southeast,

            "Southwest" => DirectionFaces::Southwest,

            "West" => DirectionFaces::West,

            _ => DirectionFaces::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a DirectionFaces> for &'a str {
    fn from(s: &'a DirectionFaces) -> &'a str {
        match s {
            DirectionFaces::East => "East",

            DirectionFaces::North => "North",

            DirectionFaces::Northeast => "Northeast",

            DirectionFaces::Northwest => "Northwest",

            DirectionFaces::South => "South",

            DirectionFaces::Southeast => "Southeast",

            DirectionFaces::Southwest => "Southwest",

            DirectionFaces::West => "West",

            DirectionFaces::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for DirectionFaces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DirectionFaces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
