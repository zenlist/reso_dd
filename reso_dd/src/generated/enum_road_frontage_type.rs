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

pub(crate) mod option_vec_road_frontage_type_format {
    use super::RoadFrontageType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<RoadFrontageType>>,
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
    ) -> Result<Option<Vec<RoadFrontageType>>, D::Error>
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
