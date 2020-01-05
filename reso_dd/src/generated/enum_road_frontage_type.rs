// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RoadFrontageType Lookups](https://ddwiki.reso.org/display/DDW17/RoadFrontageType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoadFrontageType {
    /// "[Alley](https://ddwiki.reso.org/display/DDW17/Alley)": The property fronts to an alley.
    Alley,

    /// "[City Street](https://ddwiki.reso.org/display/DDW17/City+Street)": The property fronts to a city street.
    CityStreet,

    /// "[County Road](https://ddwiki.reso.org/display/DDW17/County+Road)": The property fronts to a county road.
    CountyRoad,

    /// "[Easement](https://ddwiki.reso.org/display/DDW17/Easement)": The property fronts to an easement.
    Easement,

    /// "[Freeway](https://ddwiki.reso.org/display/DDW17/Freeway)": The property fronts to a freeway.
    Freeway,

    /// "[Highway](https://ddwiki.reso.org/display/DDW17/Highway)": The property fronts to a highway.
    Highway,

    /// "[Interstate](https://ddwiki.reso.org/display/DDW17/Interstate)": The property fronts to an interstate.
    Interstate,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246205)": The property does not have any road frontage.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246206)": The property fronts to a road other than those in this list.
    Other,

    /// "[Private Road](https://ddwiki.reso.org/display/DDW17/Private+Road)": The property fronts to a private road.
    PrivateRoad,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246208)": See Public or Private Remarks for details on the road frontage.
    SeeRemarks,

    /// "[State Road](https://ddwiki.reso.org/display/DDW17/State+Road)": The property fronts to a state road.
    StateRoad,

    /// "[Unimproved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246210)": The property's road frontage is unimproved.
    Unimproved,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for RoadFrontageType {
    fn from_str(s: &str) -> RoadFrontageType {
        match s {
            "Alley" => RoadFrontageType::Alley,

            "City Street" => RoadFrontageType::CityStreet,

            "County Road" => RoadFrontageType::CountyRoad,

            "Easement" => RoadFrontageType::Easement,

            "Freeway" => RoadFrontageType::Freeway,

            "Highway" => RoadFrontageType::Highway,

            "Interstate" => RoadFrontageType::Interstate,

            "None" => RoadFrontageType::None,

            "Other" => RoadFrontageType::Other,

            "Private Road" => RoadFrontageType::PrivateRoad,

            "See Remarks" => RoadFrontageType::SeeRemarks,

            "State Road" => RoadFrontageType::StateRoad,

            "Unimproved" => RoadFrontageType::Unimproved,

            _ => RoadFrontageType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> RoadFrontageType {
        match s.as_ref() {
            "Alley" => RoadFrontageType::Alley,

            "City Street" => RoadFrontageType::CityStreet,

            "County Road" => RoadFrontageType::CountyRoad,

            "Easement" => RoadFrontageType::Easement,

            "Freeway" => RoadFrontageType::Freeway,

            "Highway" => RoadFrontageType::Highway,

            "Interstate" => RoadFrontageType::Interstate,

            "None" => RoadFrontageType::None,

            "Other" => RoadFrontageType::Other,

            "Private Road" => RoadFrontageType::PrivateRoad,

            "See Remarks" => RoadFrontageType::SeeRemarks,

            "State Road" => RoadFrontageType::StateRoad,

            "Unimproved" => RoadFrontageType::Unimproved,

            _ => RoadFrontageType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            RoadFrontageType::Alley => "Alley",

            RoadFrontageType::CityStreet => "City Street",

            RoadFrontageType::CountyRoad => "County Road",

            RoadFrontageType::Easement => "Easement",

            RoadFrontageType::Freeway => "Freeway",

            RoadFrontageType::Highway => "Highway",

            RoadFrontageType::Interstate => "Interstate",

            RoadFrontageType::None => "None",

            RoadFrontageType::Other => "Other",

            RoadFrontageType::PrivateRoad => "Private Road",

            RoadFrontageType::SeeRemarks => "See Remarks",

            RoadFrontageType::StateRoad => "State Road",

            RoadFrontageType::Unimproved => "Unimproved",

            RoadFrontageType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            RoadFrontageType::Alley => "Alley".into(),

            RoadFrontageType::CityStreet => "City Street".into(),

            RoadFrontageType::CountyRoad => "County Road".into(),

            RoadFrontageType::Easement => "Easement".into(),

            RoadFrontageType::Freeway => "Freeway".into(),

            RoadFrontageType::Highway => "Highway".into(),

            RoadFrontageType::Interstate => "Interstate".into(),

            RoadFrontageType::None => "None".into(),

            RoadFrontageType::Other => "Other".into(),

            RoadFrontageType::PrivateRoad => "Private Road".into(),

            RoadFrontageType::SeeRemarks => "See Remarks".into(),

            RoadFrontageType::StateRoad => "State Road".into(),

            RoadFrontageType::Unimproved => "Unimproved".into(),

            RoadFrontageType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            RoadFrontageType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for RoadFrontageType {
    fn from(s: String) -> RoadFrontageType {
        match s.as_ref() {
            "Alley" => RoadFrontageType::Alley,

            "City Street" => RoadFrontageType::CityStreet,

            "County Road" => RoadFrontageType::CountyRoad,

            "Easement" => RoadFrontageType::Easement,

            "Freeway" => RoadFrontageType::Freeway,

            "Highway" => RoadFrontageType::Highway,

            "Interstate" => RoadFrontageType::Interstate,

            "None" => RoadFrontageType::None,

            "Other" => RoadFrontageType::Other,

            "Private Road" => RoadFrontageType::PrivateRoad,

            "See Remarks" => RoadFrontageType::SeeRemarks,

            "State Road" => RoadFrontageType::StateRoad,

            "Unimproved" => RoadFrontageType::Unimproved,

            _ => RoadFrontageType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RoadFrontageType {
    fn from(s: &str) -> RoadFrontageType {
        match s {
            "Alley" => RoadFrontageType::Alley,

            "City Street" => RoadFrontageType::CityStreet,

            "County Road" => RoadFrontageType::CountyRoad,

            "Easement" => RoadFrontageType::Easement,

            "Freeway" => RoadFrontageType::Freeway,

            "Highway" => RoadFrontageType::Highway,

            "Interstate" => RoadFrontageType::Interstate,

            "None" => RoadFrontageType::None,

            "Other" => RoadFrontageType::Other,

            "Private Road" => RoadFrontageType::PrivateRoad,

            "See Remarks" => RoadFrontageType::SeeRemarks,

            "State Road" => RoadFrontageType::StateRoad,

            "Unimproved" => RoadFrontageType::Unimproved,

            _ => RoadFrontageType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RoadFrontageType> for &'a str {
    fn from(s: &'a RoadFrontageType) -> &'a str {
        match s {
            RoadFrontageType::Alley => "Alley",

            RoadFrontageType::CityStreet => "City Street",

            RoadFrontageType::CountyRoad => "County Road",

            RoadFrontageType::Easement => "Easement",

            RoadFrontageType::Freeway => "Freeway",

            RoadFrontageType::Highway => "Highway",

            RoadFrontageType::Interstate => "Interstate",

            RoadFrontageType::None => "None",

            RoadFrontageType::Other => "Other",

            RoadFrontageType::PrivateRoad => "Private Road",

            RoadFrontageType::SeeRemarks => "See Remarks",

            RoadFrontageType::StateRoad => "State Road",

            RoadFrontageType::Unimproved => "Unimproved",

            RoadFrontageType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RoadFrontageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RoadFrontageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
