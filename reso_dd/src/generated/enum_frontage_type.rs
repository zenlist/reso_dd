// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [FrontageType Lookups](https://ddwiki.reso.org/display/DDW17/FrontageType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FrontageType {
    /// "[Bay/Harbor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244780)": The property fronts to a bay or harbor.
    BayHarbor,

    /// "[Golf Course](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244781)": The property fronts to a golf course.
    GolfCourse,

    /// "[Lagoon/Estuary](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244782)": The property fronts to a lagoon or estuary.
    LagoonEstuary,

    /// "[Lakefront](https://ddwiki.reso.org/display/DDW17/Lakefront)": The property is on a lakefront.
    Lakefront,

    /// "[Oceanfront](https://ddwiki.reso.org/display/DDW17/Oceanfront)": The property is on an oceanfront.
    Oceanfront,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244785)": The frontage of the property is something other than the options in this list.
    Other,

    /// "[River](https://ddwiki.reso.org/display/DDW17/River)": The property is on a riverfront.
    River,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244787)": See the Public or Private Remarks for details on the frontage of the property.
    SeeRemarks,

    /// "[Waterfront](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244788)": The property is on a waterfront.
    Waterfront,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for FrontageType {
    fn from_str(s: &str) -> FrontageType {
        match s {
            "Bay/Harbor" => FrontageType::BayHarbor,

            "Golf Course" => FrontageType::GolfCourse,

            "Lagoon/Estuary" => FrontageType::LagoonEstuary,

            "Lakefront" => FrontageType::Lakefront,

            "Oceanfront" => FrontageType::Oceanfront,

            "Other" => FrontageType::Other,

            "River" => FrontageType::River,

            "See Remarks" => FrontageType::SeeRemarks,

            "Waterfront" => FrontageType::Waterfront,

            _ => FrontageType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> FrontageType {
        match s.as_ref() {
            "Bay/Harbor" => FrontageType::BayHarbor,

            "Golf Course" => FrontageType::GolfCourse,

            "Lagoon/Estuary" => FrontageType::LagoonEstuary,

            "Lakefront" => FrontageType::Lakefront,

            "Oceanfront" => FrontageType::Oceanfront,

            "Other" => FrontageType::Other,

            "River" => FrontageType::River,

            "See Remarks" => FrontageType::SeeRemarks,

            "Waterfront" => FrontageType::Waterfront,

            _ => FrontageType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            FrontageType::BayHarbor => "Bay/Harbor",

            FrontageType::GolfCourse => "Golf Course",

            FrontageType::LagoonEstuary => "Lagoon/Estuary",

            FrontageType::Lakefront => "Lakefront",

            FrontageType::Oceanfront => "Oceanfront",

            FrontageType::Other => "Other",

            FrontageType::River => "River",

            FrontageType::SeeRemarks => "See Remarks",

            FrontageType::Waterfront => "Waterfront",

            FrontageType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            FrontageType::BayHarbor => "Bay/Harbor".into(),

            FrontageType::GolfCourse => "Golf Course".into(),

            FrontageType::LagoonEstuary => "Lagoon/Estuary".into(),

            FrontageType::Lakefront => "Lakefront".into(),

            FrontageType::Oceanfront => "Oceanfront".into(),

            FrontageType::Other => "Other".into(),

            FrontageType::River => "River".into(),

            FrontageType::SeeRemarks => "See Remarks".into(),

            FrontageType::Waterfront => "Waterfront".into(),

            FrontageType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            FrontageType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for FrontageType {
    fn from(s: String) -> FrontageType {
        match s.as_ref() {
            "Bay/Harbor" => FrontageType::BayHarbor,

            "Golf Course" => FrontageType::GolfCourse,

            "Lagoon/Estuary" => FrontageType::LagoonEstuary,

            "Lakefront" => FrontageType::Lakefront,

            "Oceanfront" => FrontageType::Oceanfront,

            "Other" => FrontageType::Other,

            "River" => FrontageType::River,

            "See Remarks" => FrontageType::SeeRemarks,

            "Waterfront" => FrontageType::Waterfront,

            _ => FrontageType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for FrontageType {
    fn from(s: &str) -> FrontageType {
        match s {
            "Bay/Harbor" => FrontageType::BayHarbor,

            "Golf Course" => FrontageType::GolfCourse,

            "Lagoon/Estuary" => FrontageType::LagoonEstuary,

            "Lakefront" => FrontageType::Lakefront,

            "Oceanfront" => FrontageType::Oceanfront,

            "Other" => FrontageType::Other,

            "River" => FrontageType::River,

            "See Remarks" => FrontageType::SeeRemarks,

            "Waterfront" => FrontageType::Waterfront,

            _ => FrontageType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a FrontageType> for &'a str {
    fn from(s: &'a FrontageType) -> &'a str {
        match s {
            FrontageType::BayHarbor => "Bay/Harbor",

            FrontageType::GolfCourse => "Golf Course",

            FrontageType::LagoonEstuary => "Lagoon/Estuary",

            FrontageType::Lakefront => "Lakefront",

            FrontageType::Oceanfront => "Oceanfront",

            FrontageType::Other => "Other",

            FrontageType::River => "River",

            FrontageType::SeeRemarks => "See Remarks",

            FrontageType::Waterfront => "Waterfront",

            FrontageType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for FrontageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FrontageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
