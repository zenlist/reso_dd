// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [DoorFeatures Lookups](https://ddwiki.reso.org/display/DDW17/DoorFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DoorFeatures {
    /// "[ENERGY STAR Qualified Doors](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Doors)": The property has ENERGY STAR qualified door(s).
    ENERGYSTARQualifiedDoors,

    /// "[French Doors](https://ddwiki.reso.org/display/DDW17/French+Doors)": The property has doors with glass panes throughout the length of the door.
    FrenchDoors,

    /// "[Mirrored Closet Door(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244602)": The property has one or more closet doors that have a mirrored surface.
    MirroredClosetDoors,

    /// "[Sliding Doors](https://ddwiki.reso.org/display/DDW17/Sliding+Doors)": The property has sliding doors.
    SlidingDoors,

    /// "[Storm Door(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244604)": The property has one or more storm doors.
    StormDoors,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for DoorFeatures {
    fn from_str(s: &str) -> DoorFeatures {
        match s {
            "ENERGY STAR Qualified Doors" => DoorFeatures::ENERGYSTARQualifiedDoors,

            "French Doors" => DoorFeatures::FrenchDoors,

            "Mirrored Closet Door(s)" => DoorFeatures::MirroredClosetDoors,

            "Sliding Doors" => DoorFeatures::SlidingDoors,

            "Storm Door(s)" => DoorFeatures::StormDoors,

            _ => DoorFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> DoorFeatures {
        match s.as_ref() {
            "ENERGY STAR Qualified Doors" => DoorFeatures::ENERGYSTARQualifiedDoors,

            "French Doors" => DoorFeatures::FrenchDoors,

            "Mirrored Closet Door(s)" => DoorFeatures::MirroredClosetDoors,

            "Sliding Doors" => DoorFeatures::SlidingDoors,

            "Storm Door(s)" => DoorFeatures::StormDoors,

            _ => DoorFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            DoorFeatures::ENERGYSTARQualifiedDoors => "ENERGY STAR Qualified Doors",

            DoorFeatures::FrenchDoors => "French Doors",

            DoorFeatures::MirroredClosetDoors => "Mirrored Closet Door(s)",

            DoorFeatures::SlidingDoors => "Sliding Doors",

            DoorFeatures::StormDoors => "Storm Door(s)",

            DoorFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            DoorFeatures::ENERGYSTARQualifiedDoors => "ENERGY STAR Qualified Doors".into(),

            DoorFeatures::FrenchDoors => "French Doors".into(),

            DoorFeatures::MirroredClosetDoors => "Mirrored Closet Door(s)".into(),

            DoorFeatures::SlidingDoors => "Sliding Doors".into(),

            DoorFeatures::StormDoors => "Storm Door(s)".into(),

            DoorFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            DoorFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for DoorFeatures {
    fn from(s: String) -> DoorFeatures {
        match s.as_ref() {
            "ENERGY STAR Qualified Doors" => DoorFeatures::ENERGYSTARQualifiedDoors,

            "French Doors" => DoorFeatures::FrenchDoors,

            "Mirrored Closet Door(s)" => DoorFeatures::MirroredClosetDoors,

            "Sliding Doors" => DoorFeatures::SlidingDoors,

            "Storm Door(s)" => DoorFeatures::StormDoors,

            _ => DoorFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for DoorFeatures {
    fn from(s: &str) -> DoorFeatures {
        match s {
            "ENERGY STAR Qualified Doors" => DoorFeatures::ENERGYSTARQualifiedDoors,

            "French Doors" => DoorFeatures::FrenchDoors,

            "Mirrored Closet Door(s)" => DoorFeatures::MirroredClosetDoors,

            "Sliding Doors" => DoorFeatures::SlidingDoors,

            "Storm Door(s)" => DoorFeatures::StormDoors,

            _ => DoorFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a DoorFeatures> for &'a str {
    fn from(s: &'a DoorFeatures) -> &'a str {
        match s {
            DoorFeatures::ENERGYSTARQualifiedDoors => "ENERGY STAR Qualified Doors",

            DoorFeatures::FrenchDoors => "French Doors",

            DoorFeatures::MirroredClosetDoors => "Mirrored Closet Door(s)",

            DoorFeatures::SlidingDoors => "Sliding Doors",

            DoorFeatures::StormDoors => "Storm Door(s)",

            DoorFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for DoorFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DoorFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
