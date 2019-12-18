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

pub(crate) mod option_vec_door_features_format {
    use super::DoorFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<DoorFeatures>>,
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
    ) -> Result<Option<Vec<DoorFeatures>>, D::Error>
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
