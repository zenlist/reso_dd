// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [YearBuiltSource Lookups](https://ddwiki.reso.org/display/DDW17/YearBuiltSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum YearBuiltSource {
    /// "[Appraiser](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246666)": An appraiser provided the year built.
    Appraiser,

    /// "[Assessor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246667)": The assessor provided the year built.
    Assessor,

    /// "[Builder](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246668)": The builder provided the year built.
    Builder,

    /// "[Estimated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246669)": The year built is an estimate.
    Estimated,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246670)": The year built was provided by another party not listed.
    Other,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246671)": The owner provided the year built.
    Owner,

    /// "[Public Records](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246672)": The year built was received from public records.
    PublicRecords,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246673)": See remarks for information about the source of the lot size measurement.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for YearBuiltSource {
    fn from_str(s: &str) -> YearBuiltSource {
        match s {
            "Appraiser" => YearBuiltSource::Appraiser,

            "Assessor" => YearBuiltSource::Assessor,

            "Builder" => YearBuiltSource::Builder,

            "Estimated" => YearBuiltSource::Estimated,

            "Other" => YearBuiltSource::Other,

            "Owner" => YearBuiltSource::Owner,

            "Public Records" => YearBuiltSource::PublicRecords,

            "See Remarks" => YearBuiltSource::SeeRemarks,

            _ => YearBuiltSource::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> YearBuiltSource {
        match s.as_ref() {
            "Appraiser" => YearBuiltSource::Appraiser,

            "Assessor" => YearBuiltSource::Assessor,

            "Builder" => YearBuiltSource::Builder,

            "Estimated" => YearBuiltSource::Estimated,

            "Other" => YearBuiltSource::Other,

            "Owner" => YearBuiltSource::Owner,

            "Public Records" => YearBuiltSource::PublicRecords,

            "See Remarks" => YearBuiltSource::SeeRemarks,

            _ => YearBuiltSource::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            YearBuiltSource::Appraiser => "Appraiser",

            YearBuiltSource::Assessor => "Assessor",

            YearBuiltSource::Builder => "Builder",

            YearBuiltSource::Estimated => "Estimated",

            YearBuiltSource::Other => "Other",

            YearBuiltSource::Owner => "Owner",

            YearBuiltSource::PublicRecords => "Public Records",

            YearBuiltSource::SeeRemarks => "See Remarks",

            YearBuiltSource::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            YearBuiltSource::Appraiser => "Appraiser".into(),

            YearBuiltSource::Assessor => "Assessor".into(),

            YearBuiltSource::Builder => "Builder".into(),

            YearBuiltSource::Estimated => "Estimated".into(),

            YearBuiltSource::Other => "Other".into(),

            YearBuiltSource::Owner => "Owner".into(),

            YearBuiltSource::PublicRecords => "Public Records".into(),

            YearBuiltSource::SeeRemarks => "See Remarks".into(),

            YearBuiltSource::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            YearBuiltSource::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for YearBuiltSource {
    fn from(s: String) -> YearBuiltSource {
        match s.as_ref() {
            "Appraiser" => YearBuiltSource::Appraiser,

            "Assessor" => YearBuiltSource::Assessor,

            "Builder" => YearBuiltSource::Builder,

            "Estimated" => YearBuiltSource::Estimated,

            "Other" => YearBuiltSource::Other,

            "Owner" => YearBuiltSource::Owner,

            "Public Records" => YearBuiltSource::PublicRecords,

            "See Remarks" => YearBuiltSource::SeeRemarks,

            _ => YearBuiltSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for YearBuiltSource {
    fn from(s: &str) -> YearBuiltSource {
        match s {
            "Appraiser" => YearBuiltSource::Appraiser,

            "Assessor" => YearBuiltSource::Assessor,

            "Builder" => YearBuiltSource::Builder,

            "Estimated" => YearBuiltSource::Estimated,

            "Other" => YearBuiltSource::Other,

            "Owner" => YearBuiltSource::Owner,

            "Public Records" => YearBuiltSource::PublicRecords,

            "See Remarks" => YearBuiltSource::SeeRemarks,

            _ => YearBuiltSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a YearBuiltSource> for &'a str {
    fn from(s: &'a YearBuiltSource) -> &'a str {
        match s {
            YearBuiltSource::Appraiser => "Appraiser",

            YearBuiltSource::Assessor => "Assessor",

            YearBuiltSource::Builder => "Builder",

            YearBuiltSource::Estimated => "Estimated",

            YearBuiltSource::Other => "Other",

            YearBuiltSource::Owner => "Owner",

            YearBuiltSource::PublicRecords => "Public Records",

            YearBuiltSource::SeeRemarks => "See Remarks",

            YearBuiltSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for YearBuiltSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for YearBuiltSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
