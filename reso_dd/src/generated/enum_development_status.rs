// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [DevelopmentStatus Lookups](https://ddwiki.reso.org/display/DDW17/DevelopmentStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DevelopmentStatus {
    /// "[Completed](https://ddwiki.reso.org/display/DDW17/Completed)": The development of the land is complete.
    Completed,

    /// "[Finished Lot(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244568)": The development of the land is finished.
    FinishedLots,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244569)": The development status of the land is something other than those options in this list.
    Other,

    /// "[Proposed](https://ddwiki.reso.org/display/DDW17/Proposed)": The development of the land is in the proposal phase.
    Proposed,

    /// "[Raw Land](https://ddwiki.reso.org/display/DDW17/Raw+Land)": The land is raw and undeveloped.
    RawLand,

    /// "[Rough Grade](https://ddwiki.reso.org/display/DDW17/Rough+Grade)": The development of the last is in the rough grade phase.
    RoughGrade,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244573)": See the Public or Private remarks for details on the development status of the land.
    SeeRemarks,

    /// "[Site Plan Approved](https://ddwiki.reso.org/display/DDW17/Site+Plan+Approved)": The site plan has been approved for the development.
    SitePlanApproved,

    /// "[Site Plan Filed](https://ddwiki.reso.org/display/DDW17/Site+Plan+Filed)": The site plan has been filed for the development.
    SitePlanFiled,

    /// "[Under Construction](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244576)": There is construction in progress at the development.
    UnderConstruction,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for DevelopmentStatus {
    fn from(s: String) -> DevelopmentStatus {
        match s.as_ref() {
            "Completed" => DevelopmentStatus::Completed,

            "Finished Lot(s)" => DevelopmentStatus::FinishedLots,

            "Other" => DevelopmentStatus::Other,

            "Proposed" => DevelopmentStatus::Proposed,

            "Raw Land" => DevelopmentStatus::RawLand,

            "Rough Grade" => DevelopmentStatus::RoughGrade,

            "See Remarks" => DevelopmentStatus::SeeRemarks,

            "Site Plan Approved" => DevelopmentStatus::SitePlanApproved,

            "Site Plan Filed" => DevelopmentStatus::SitePlanFiled,

            "Under Construction" => DevelopmentStatus::UnderConstruction,

            _ => DevelopmentStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for DevelopmentStatus {
    fn from(s: &str) -> DevelopmentStatus {
        match s {
            "Completed" => DevelopmentStatus::Completed,

            "Finished Lot(s)" => DevelopmentStatus::FinishedLots,

            "Other" => DevelopmentStatus::Other,

            "Proposed" => DevelopmentStatus::Proposed,

            "Raw Land" => DevelopmentStatus::RawLand,

            "Rough Grade" => DevelopmentStatus::RoughGrade,

            "See Remarks" => DevelopmentStatus::SeeRemarks,

            "Site Plan Approved" => DevelopmentStatus::SitePlanApproved,

            "Site Plan Filed" => DevelopmentStatus::SitePlanFiled,

            "Under Construction" => DevelopmentStatus::UnderConstruction,

            _ => DevelopmentStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a DevelopmentStatus> for &'a str {
    fn from(s: &'a DevelopmentStatus) -> &'a str {
        match s {
            DevelopmentStatus::Completed => "Completed",

            DevelopmentStatus::FinishedLots => "Finished Lot(s)",

            DevelopmentStatus::Other => "Other",

            DevelopmentStatus::Proposed => "Proposed",

            DevelopmentStatus::RawLand => "Raw Land",

            DevelopmentStatus::RoughGrade => "Rough Grade",

            DevelopmentStatus::SeeRemarks => "See Remarks",

            DevelopmentStatus::SitePlanApproved => "Site Plan Approved",

            DevelopmentStatus::SitePlanFiled => "Site Plan Filed",

            DevelopmentStatus::UnderConstruction => "Under Construction",

            DevelopmentStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for DevelopmentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DevelopmentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_development_status_format {
    use super::DevelopmentStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<DevelopmentStatus>>,
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
    ) -> Result<Option<Vec<DevelopmentStatus>>, D::Error>
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
