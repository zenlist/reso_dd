// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [WaterfrontFeatures Lookups](https://ddwiki.reso.org/display/DDW17/WaterfrontFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WaterfrontFeatures {
    /// "[Beach Access](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246624)": The property has access to the beach.
    BeachAccess,

    /// "[Beach Front](https://ddwiki.reso.org/display/DDW17/Beach+Front)": The property is on the beach front.
    BeachFront,

    /// "[Canal Access](https://ddwiki.reso.org/display/DDW17/Canal+Access)": The property has access to the canal(s).
    CanalAccess,

    /// "[Canal Front](https://ddwiki.reso.org/display/DDW17/Canal+Front)": The property is located on the canal.
    CanalFront,

    /// "[Creek](https://ddwiki.reso.org/display/DDW17/Creek)": The property is either on or near a creek.
    Creek,

    /// "[Lagoon](https://ddwiki.reso.org/display/DDW17/Lagoon)": The property is either on or near a lagoon.
    Lagoon,

    /// "[Lake](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246630)": The property is either on or near a lake.
    Lake,

    /// "[Lake Front](https://ddwiki.reso.org/display/DDW17/Lake+Front)": The property is on the lake front.
    LakeFront,

    /// "[Lake Privileges](https://ddwiki.reso.org/display/DDW17/Lake+Privileges)": The property includes rights to access the lake.
    LakePrivileges,

    /// "[Navigable Water](https://ddwiki.reso.org/display/DDW17/Navigable+Water)": The water wide, slow and deep enough for water vessels.
    NavigableWater,

    /// "[Ocean Access](https://ddwiki.reso.org/display/DDW17/Ocean+Access)": The property has access to the ocean.
    OceanAccess,

    /// "[Ocean Front](https://ddwiki.reso.org/display/DDW17/Ocean+Front)": The property is on the ocean front.
    OceanFront,

    /// "[Pond](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246636)": The property is on or near a pond.
    Pond,

    /// "[River Access](https://ddwiki.reso.org/display/DDW17/River+Access)": The property has access to the river.
    RiverAccess,

    /// "[River Front](https://ddwiki.reso.org/display/DDW17/River+Front)": The property is located on the river front.
    RiverFront,

    /// "[Seawall](https://ddwiki.reso.org/display/DDW17/Seawall)": The property is protected by a sea wall or barrier.
    Seawall,

    /// "[Stream](https://ddwiki.reso.org/display/DDW17/Stream)": The property is on our near a stream.
    Stream,

    /// "[Waterfront](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246641)": The property is located on the waterfront.
    Waterfront,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for WaterfrontFeatures {
    fn from_str(s: &str) -> WaterfrontFeatures {
        match s {
            "Beach Access" => WaterfrontFeatures::BeachAccess,

            "Beach Front" => WaterfrontFeatures::BeachFront,

            "Canal Access" => WaterfrontFeatures::CanalAccess,

            "Canal Front" => WaterfrontFeatures::CanalFront,

            "Creek" => WaterfrontFeatures::Creek,

            "Lagoon" => WaterfrontFeatures::Lagoon,

            "Lake" => WaterfrontFeatures::Lake,

            "Lake Front" => WaterfrontFeatures::LakeFront,

            "Lake Privileges" => WaterfrontFeatures::LakePrivileges,

            "Navigable Water" => WaterfrontFeatures::NavigableWater,

            "Ocean Access" => WaterfrontFeatures::OceanAccess,

            "Ocean Front" => WaterfrontFeatures::OceanFront,

            "Pond" => WaterfrontFeatures::Pond,

            "River Access" => WaterfrontFeatures::RiverAccess,

            "River Front" => WaterfrontFeatures::RiverFront,

            "Seawall" => WaterfrontFeatures::Seawall,

            "Stream" => WaterfrontFeatures::Stream,

            "Waterfront" => WaterfrontFeatures::Waterfront,

            _ => WaterfrontFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> WaterfrontFeatures {
        match s.as_ref() {
            "Beach Access" => WaterfrontFeatures::BeachAccess,

            "Beach Front" => WaterfrontFeatures::BeachFront,

            "Canal Access" => WaterfrontFeatures::CanalAccess,

            "Canal Front" => WaterfrontFeatures::CanalFront,

            "Creek" => WaterfrontFeatures::Creek,

            "Lagoon" => WaterfrontFeatures::Lagoon,

            "Lake" => WaterfrontFeatures::Lake,

            "Lake Front" => WaterfrontFeatures::LakeFront,

            "Lake Privileges" => WaterfrontFeatures::LakePrivileges,

            "Navigable Water" => WaterfrontFeatures::NavigableWater,

            "Ocean Access" => WaterfrontFeatures::OceanAccess,

            "Ocean Front" => WaterfrontFeatures::OceanFront,

            "Pond" => WaterfrontFeatures::Pond,

            "River Access" => WaterfrontFeatures::RiverAccess,

            "River Front" => WaterfrontFeatures::RiverFront,

            "Seawall" => WaterfrontFeatures::Seawall,

            "Stream" => WaterfrontFeatures::Stream,

            "Waterfront" => WaterfrontFeatures::Waterfront,

            _ => WaterfrontFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            WaterfrontFeatures::BeachAccess => "Beach Access",

            WaterfrontFeatures::BeachFront => "Beach Front",

            WaterfrontFeatures::CanalAccess => "Canal Access",

            WaterfrontFeatures::CanalFront => "Canal Front",

            WaterfrontFeatures::Creek => "Creek",

            WaterfrontFeatures::Lagoon => "Lagoon",

            WaterfrontFeatures::Lake => "Lake",

            WaterfrontFeatures::LakeFront => "Lake Front",

            WaterfrontFeatures::LakePrivileges => "Lake Privileges",

            WaterfrontFeatures::NavigableWater => "Navigable Water",

            WaterfrontFeatures::OceanAccess => "Ocean Access",

            WaterfrontFeatures::OceanFront => "Ocean Front",

            WaterfrontFeatures::Pond => "Pond",

            WaterfrontFeatures::RiverAccess => "River Access",

            WaterfrontFeatures::RiverFront => "River Front",

            WaterfrontFeatures::Seawall => "Seawall",

            WaterfrontFeatures::Stream => "Stream",

            WaterfrontFeatures::Waterfront => "Waterfront",

            WaterfrontFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            WaterfrontFeatures::BeachAccess => "Beach Access".into(),

            WaterfrontFeatures::BeachFront => "Beach Front".into(),

            WaterfrontFeatures::CanalAccess => "Canal Access".into(),

            WaterfrontFeatures::CanalFront => "Canal Front".into(),

            WaterfrontFeatures::Creek => "Creek".into(),

            WaterfrontFeatures::Lagoon => "Lagoon".into(),

            WaterfrontFeatures::Lake => "Lake".into(),

            WaterfrontFeatures::LakeFront => "Lake Front".into(),

            WaterfrontFeatures::LakePrivileges => "Lake Privileges".into(),

            WaterfrontFeatures::NavigableWater => "Navigable Water".into(),

            WaterfrontFeatures::OceanAccess => "Ocean Access".into(),

            WaterfrontFeatures::OceanFront => "Ocean Front".into(),

            WaterfrontFeatures::Pond => "Pond".into(),

            WaterfrontFeatures::RiverAccess => "River Access".into(),

            WaterfrontFeatures::RiverFront => "River Front".into(),

            WaterfrontFeatures::Seawall => "Seawall".into(),

            WaterfrontFeatures::Stream => "Stream".into(),

            WaterfrontFeatures::Waterfront => "Waterfront".into(),

            WaterfrontFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            WaterfrontFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for WaterfrontFeatures {
    fn from(s: String) -> WaterfrontFeatures {
        match s.as_ref() {
            "Beach Access" => WaterfrontFeatures::BeachAccess,

            "Beach Front" => WaterfrontFeatures::BeachFront,

            "Canal Access" => WaterfrontFeatures::CanalAccess,

            "Canal Front" => WaterfrontFeatures::CanalFront,

            "Creek" => WaterfrontFeatures::Creek,

            "Lagoon" => WaterfrontFeatures::Lagoon,

            "Lake" => WaterfrontFeatures::Lake,

            "Lake Front" => WaterfrontFeatures::LakeFront,

            "Lake Privileges" => WaterfrontFeatures::LakePrivileges,

            "Navigable Water" => WaterfrontFeatures::NavigableWater,

            "Ocean Access" => WaterfrontFeatures::OceanAccess,

            "Ocean Front" => WaterfrontFeatures::OceanFront,

            "Pond" => WaterfrontFeatures::Pond,

            "River Access" => WaterfrontFeatures::RiverAccess,

            "River Front" => WaterfrontFeatures::RiverFront,

            "Seawall" => WaterfrontFeatures::Seawall,

            "Stream" => WaterfrontFeatures::Stream,

            "Waterfront" => WaterfrontFeatures::Waterfront,

            _ => WaterfrontFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for WaterfrontFeatures {
    fn from(s: &str) -> WaterfrontFeatures {
        match s {
            "Beach Access" => WaterfrontFeatures::BeachAccess,

            "Beach Front" => WaterfrontFeatures::BeachFront,

            "Canal Access" => WaterfrontFeatures::CanalAccess,

            "Canal Front" => WaterfrontFeatures::CanalFront,

            "Creek" => WaterfrontFeatures::Creek,

            "Lagoon" => WaterfrontFeatures::Lagoon,

            "Lake" => WaterfrontFeatures::Lake,

            "Lake Front" => WaterfrontFeatures::LakeFront,

            "Lake Privileges" => WaterfrontFeatures::LakePrivileges,

            "Navigable Water" => WaterfrontFeatures::NavigableWater,

            "Ocean Access" => WaterfrontFeatures::OceanAccess,

            "Ocean Front" => WaterfrontFeatures::OceanFront,

            "Pond" => WaterfrontFeatures::Pond,

            "River Access" => WaterfrontFeatures::RiverAccess,

            "River Front" => WaterfrontFeatures::RiverFront,

            "Seawall" => WaterfrontFeatures::Seawall,

            "Stream" => WaterfrontFeatures::Stream,

            "Waterfront" => WaterfrontFeatures::Waterfront,

            _ => WaterfrontFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a WaterfrontFeatures> for &'a str {
    fn from(s: &'a WaterfrontFeatures) -> &'a str {
        match s {
            WaterfrontFeatures::BeachAccess => "Beach Access",

            WaterfrontFeatures::BeachFront => "Beach Front",

            WaterfrontFeatures::CanalAccess => "Canal Access",

            WaterfrontFeatures::CanalFront => "Canal Front",

            WaterfrontFeatures::Creek => "Creek",

            WaterfrontFeatures::Lagoon => "Lagoon",

            WaterfrontFeatures::Lake => "Lake",

            WaterfrontFeatures::LakeFront => "Lake Front",

            WaterfrontFeatures::LakePrivileges => "Lake Privileges",

            WaterfrontFeatures::NavigableWater => "Navigable Water",

            WaterfrontFeatures::OceanAccess => "Ocean Access",

            WaterfrontFeatures::OceanFront => "Ocean Front",

            WaterfrontFeatures::Pond => "Pond",

            WaterfrontFeatures::RiverAccess => "River Access",

            WaterfrontFeatures::RiverFront => "River Front",

            WaterfrontFeatures::Seawall => "Seawall",

            WaterfrontFeatures::Stream => "Stream",

            WaterfrontFeatures::Waterfront => "Waterfront",

            WaterfrontFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for WaterfrontFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for WaterfrontFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
