// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RoadSurfaceType Lookups](https://ddwiki.reso.org/display/DDW17/RoadSurfaceType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoadSurfaceType {
    /// "[Alley Paved](https://ddwiki.reso.org/display/DDW17/Alley+Paved)": The property's road is a paved alley.
    AlleyPaved,

    /// "[Asphalt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246217)": The property's road is asphalt.
    Asphalt,

    /// "[Chip And Seal](https://ddwiki.reso.org/display/DDW17/Chip+And+Seal)": The property's road is chip and seal.
    ChipAndSeal,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246219)": The property's road is concrete.
    Concrete,

    /// "[Dirt](https://ddwiki.reso.org/display/DDW17/Dirt)": The property's road is dirt.
    Dirt,

    /// "[Gravel](https://ddwiki.reso.org/display/DDW17/Gravel)": The property's road is gravel.
    Gravel,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246222)": The property has no road.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246223)": The surface type of the property's road is something other than those in this list.
    Other,

    /// "[Paved](https://ddwiki.reso.org/display/DDW17/Paved)": The property's road is paved.
    Paved,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246225)": See the Public or Private Remarks for details on the road surface type.
    SeeRemarks,

    /// "[Unimproved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246226)": The property's road is unimproved.
    Unimproved,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for RoadSurfaceType {
    fn from_str(s: &str) -> RoadSurfaceType {
        match s {
            "Alley Paved" => RoadSurfaceType::AlleyPaved,

            "Asphalt" => RoadSurfaceType::Asphalt,

            "Chip And Seal" => RoadSurfaceType::ChipAndSeal,

            "Concrete" => RoadSurfaceType::Concrete,

            "Dirt" => RoadSurfaceType::Dirt,

            "Gravel" => RoadSurfaceType::Gravel,

            "None" => RoadSurfaceType::None,

            "Other" => RoadSurfaceType::Other,

            "Paved" => RoadSurfaceType::Paved,

            "See Remarks" => RoadSurfaceType::SeeRemarks,

            "Unimproved" => RoadSurfaceType::Unimproved,

            _ => RoadSurfaceType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> RoadSurfaceType {
        match s.as_ref() {
            "Alley Paved" => RoadSurfaceType::AlleyPaved,

            "Asphalt" => RoadSurfaceType::Asphalt,

            "Chip And Seal" => RoadSurfaceType::ChipAndSeal,

            "Concrete" => RoadSurfaceType::Concrete,

            "Dirt" => RoadSurfaceType::Dirt,

            "Gravel" => RoadSurfaceType::Gravel,

            "None" => RoadSurfaceType::None,

            "Other" => RoadSurfaceType::Other,

            "Paved" => RoadSurfaceType::Paved,

            "See Remarks" => RoadSurfaceType::SeeRemarks,

            "Unimproved" => RoadSurfaceType::Unimproved,

            _ => RoadSurfaceType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            RoadSurfaceType::AlleyPaved => "Alley Paved",

            RoadSurfaceType::Asphalt => "Asphalt",

            RoadSurfaceType::ChipAndSeal => "Chip And Seal",

            RoadSurfaceType::Concrete => "Concrete",

            RoadSurfaceType::Dirt => "Dirt",

            RoadSurfaceType::Gravel => "Gravel",

            RoadSurfaceType::None => "None",

            RoadSurfaceType::Other => "Other",

            RoadSurfaceType::Paved => "Paved",

            RoadSurfaceType::SeeRemarks => "See Remarks",

            RoadSurfaceType::Unimproved => "Unimproved",

            RoadSurfaceType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            RoadSurfaceType::AlleyPaved => "Alley Paved".into(),

            RoadSurfaceType::Asphalt => "Asphalt".into(),

            RoadSurfaceType::ChipAndSeal => "Chip And Seal".into(),

            RoadSurfaceType::Concrete => "Concrete".into(),

            RoadSurfaceType::Dirt => "Dirt".into(),

            RoadSurfaceType::Gravel => "Gravel".into(),

            RoadSurfaceType::None => "None".into(),

            RoadSurfaceType::Other => "Other".into(),

            RoadSurfaceType::Paved => "Paved".into(),

            RoadSurfaceType::SeeRemarks => "See Remarks".into(),

            RoadSurfaceType::Unimproved => "Unimproved".into(),

            RoadSurfaceType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            RoadSurfaceType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for RoadSurfaceType {
    fn from(s: String) -> RoadSurfaceType {
        match s.as_ref() {
            "Alley Paved" => RoadSurfaceType::AlleyPaved,

            "Asphalt" => RoadSurfaceType::Asphalt,

            "Chip And Seal" => RoadSurfaceType::ChipAndSeal,

            "Concrete" => RoadSurfaceType::Concrete,

            "Dirt" => RoadSurfaceType::Dirt,

            "Gravel" => RoadSurfaceType::Gravel,

            "None" => RoadSurfaceType::None,

            "Other" => RoadSurfaceType::Other,

            "Paved" => RoadSurfaceType::Paved,

            "See Remarks" => RoadSurfaceType::SeeRemarks,

            "Unimproved" => RoadSurfaceType::Unimproved,

            _ => RoadSurfaceType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RoadSurfaceType {
    fn from(s: &str) -> RoadSurfaceType {
        match s {
            "Alley Paved" => RoadSurfaceType::AlleyPaved,

            "Asphalt" => RoadSurfaceType::Asphalt,

            "Chip And Seal" => RoadSurfaceType::ChipAndSeal,

            "Concrete" => RoadSurfaceType::Concrete,

            "Dirt" => RoadSurfaceType::Dirt,

            "Gravel" => RoadSurfaceType::Gravel,

            "None" => RoadSurfaceType::None,

            "Other" => RoadSurfaceType::Other,

            "Paved" => RoadSurfaceType::Paved,

            "See Remarks" => RoadSurfaceType::SeeRemarks,

            "Unimproved" => RoadSurfaceType::Unimproved,

            _ => RoadSurfaceType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RoadSurfaceType> for &'a str {
    fn from(s: &'a RoadSurfaceType) -> &'a str {
        match s {
            RoadSurfaceType::AlleyPaved => "Alley Paved",

            RoadSurfaceType::Asphalt => "Asphalt",

            RoadSurfaceType::ChipAndSeal => "Chip And Seal",

            RoadSurfaceType::Concrete => "Concrete",

            RoadSurfaceType::Dirt => "Dirt",

            RoadSurfaceType::Gravel => "Gravel",

            RoadSurfaceType::None => "None",

            RoadSurfaceType::Other => "Other",

            RoadSurfaceType::Paved => "Paved",

            RoadSurfaceType::SeeRemarks => "See Remarks",

            RoadSurfaceType::Unimproved => "Unimproved",

            RoadSurfaceType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RoadSurfaceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RoadSurfaceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
