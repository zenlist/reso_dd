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

pub(crate) mod option_vec_frontage_type_format {
    use super::FrontageType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<FrontageType>>,
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
    ) -> Result<Option<Vec<FrontageType>>, D::Error>
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
