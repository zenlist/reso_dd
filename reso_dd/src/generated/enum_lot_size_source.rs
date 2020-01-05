// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LotSizeSource Lookups](https://ddwiki.reso.org/display/DDW17/LotSizeSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LotSizeSource {
    /// "[Appraiser](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245418)": An appraiser provided the measurement of the lot size.
    Appraiser,

    /// "[Assessor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245419)": The assessor provided the measurement of the lot size.
    Assessor,

    /// "[Builder](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245420)": The builder provided the measurement of the lot size.
    Builder,

    /// "[Estimated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245421)": The measurement of the lot size is an estimate.
    Estimated,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245422)": The measurement of the lot size was provided by another party not listed.
    Other,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245423)": The owner provided the measurement of the lot size.
    Owner,

    /// "[Plans](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245424)": The measurement of the lot size was taken from building plans.
    Plans,

    /// "[Public Records](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245425)": The measurement of the lot size was received from public records.
    PublicRecords,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245426)": See remarks for information about the source of the lot size measurement.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LotSizeSource {
    fn from_str(s: &str) -> LotSizeSource {
        match s {
            "Appraiser" => LotSizeSource::Appraiser,

            "Assessor" => LotSizeSource::Assessor,

            "Builder" => LotSizeSource::Builder,

            "Estimated" => LotSizeSource::Estimated,

            "Other" => LotSizeSource::Other,

            "Owner" => LotSizeSource::Owner,

            "Plans" => LotSizeSource::Plans,

            "Public Records" => LotSizeSource::PublicRecords,

            "See Remarks" => LotSizeSource::SeeRemarks,

            _ => LotSizeSource::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LotSizeSource {
        match s.as_ref() {
            "Appraiser" => LotSizeSource::Appraiser,

            "Assessor" => LotSizeSource::Assessor,

            "Builder" => LotSizeSource::Builder,

            "Estimated" => LotSizeSource::Estimated,

            "Other" => LotSizeSource::Other,

            "Owner" => LotSizeSource::Owner,

            "Plans" => LotSizeSource::Plans,

            "Public Records" => LotSizeSource::PublicRecords,

            "See Remarks" => LotSizeSource::SeeRemarks,

            _ => LotSizeSource::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LotSizeSource::Appraiser => "Appraiser",

            LotSizeSource::Assessor => "Assessor",

            LotSizeSource::Builder => "Builder",

            LotSizeSource::Estimated => "Estimated",

            LotSizeSource::Other => "Other",

            LotSizeSource::Owner => "Owner",

            LotSizeSource::Plans => "Plans",

            LotSizeSource::PublicRecords => "Public Records",

            LotSizeSource::SeeRemarks => "See Remarks",

            LotSizeSource::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LotSizeSource::Appraiser => "Appraiser".into(),

            LotSizeSource::Assessor => "Assessor".into(),

            LotSizeSource::Builder => "Builder".into(),

            LotSizeSource::Estimated => "Estimated".into(),

            LotSizeSource::Other => "Other".into(),

            LotSizeSource::Owner => "Owner".into(),

            LotSizeSource::Plans => "Plans".into(),

            LotSizeSource::PublicRecords => "Public Records".into(),

            LotSizeSource::SeeRemarks => "See Remarks".into(),

            LotSizeSource::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LotSizeSource::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LotSizeSource {
    fn from(s: String) -> LotSizeSource {
        match s.as_ref() {
            "Appraiser" => LotSizeSource::Appraiser,

            "Assessor" => LotSizeSource::Assessor,

            "Builder" => LotSizeSource::Builder,

            "Estimated" => LotSizeSource::Estimated,

            "Other" => LotSizeSource::Other,

            "Owner" => LotSizeSource::Owner,

            "Plans" => LotSizeSource::Plans,

            "Public Records" => LotSizeSource::PublicRecords,

            "See Remarks" => LotSizeSource::SeeRemarks,

            _ => LotSizeSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LotSizeSource {
    fn from(s: &str) -> LotSizeSource {
        match s {
            "Appraiser" => LotSizeSource::Appraiser,

            "Assessor" => LotSizeSource::Assessor,

            "Builder" => LotSizeSource::Builder,

            "Estimated" => LotSizeSource::Estimated,

            "Other" => LotSizeSource::Other,

            "Owner" => LotSizeSource::Owner,

            "Plans" => LotSizeSource::Plans,

            "Public Records" => LotSizeSource::PublicRecords,

            "See Remarks" => LotSizeSource::SeeRemarks,

            _ => LotSizeSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LotSizeSource> for &'a str {
    fn from(s: &'a LotSizeSource) -> &'a str {
        match s {
            LotSizeSource::Appraiser => "Appraiser",

            LotSizeSource::Assessor => "Assessor",

            LotSizeSource::Builder => "Builder",

            LotSizeSource::Estimated => "Estimated",

            LotSizeSource::Other => "Other",

            LotSizeSource::Owner => "Owner",

            LotSizeSource::Plans => "Plans",

            LotSizeSource::PublicRecords => "Public Records",

            LotSizeSource::SeeRemarks => "See Remarks",

            LotSizeSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LotSizeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LotSizeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
