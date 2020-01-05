// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LotDimensionsSource Lookups](https://ddwiki.reso.org/display/DDW17/LotDimensionsSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LotDimensionsSource {
    /// "[Appraiser](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245464)": The lot dimensions were provided by an appraiser.
    Appraiser,

    /// "[Assessor](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245465)": The lot dimensions were provided by the assessor.
    Assessor,

    /// "[Builder](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245466)": The lot dimensions were provided by the builder.
    Builder,

    /// "[Estimated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245467)": The lot dimensions were estimated.
    Estimated,

    /// "[GIS Calculated](https://ddwiki.reso.org/display/DDW17/GIS+Calculated)": The lot dimensions were GIS calculated.
    GISCalculated,

    /// "[Measured](https://ddwiki.reso.org/display/DDW17/Measured)": The lot dimensions were measured.
    Measured,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245470)": The lot dimensions were provided by a source other than those in this list.
    Other,

    /// "[Owner](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245471)": The lot dimensions were provided by the owner.
    Owner,

    /// "[Public Records](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245472)": The lot dimensions were taken from public records.
    PublicRecords,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245473)": See the Public or Private Remarks for details on the source of the lot dimensions.
    SeeRemarks,

    /// "[Survey](https://ddwiki.reso.org/display/DDW17/Survey)": The lot dimensions were provided by a land survey.
    Survey,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for LotDimensionsSource {
    fn from_str(s: &str) -> LotDimensionsSource {
        match s {
            "Appraiser" => LotDimensionsSource::Appraiser,

            "Assessor" => LotDimensionsSource::Assessor,

            "Builder" => LotDimensionsSource::Builder,

            "Estimated" => LotDimensionsSource::Estimated,

            "GIS Calculated" => LotDimensionsSource::GISCalculated,

            "Measured" => LotDimensionsSource::Measured,

            "Other" => LotDimensionsSource::Other,

            "Owner" => LotDimensionsSource::Owner,

            "Public Records" => LotDimensionsSource::PublicRecords,

            "See Remarks" => LotDimensionsSource::SeeRemarks,

            "Survey" => LotDimensionsSource::Survey,

            _ => LotDimensionsSource::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> LotDimensionsSource {
        match s.as_ref() {
            "Appraiser" => LotDimensionsSource::Appraiser,

            "Assessor" => LotDimensionsSource::Assessor,

            "Builder" => LotDimensionsSource::Builder,

            "Estimated" => LotDimensionsSource::Estimated,

            "GIS Calculated" => LotDimensionsSource::GISCalculated,

            "Measured" => LotDimensionsSource::Measured,

            "Other" => LotDimensionsSource::Other,

            "Owner" => LotDimensionsSource::Owner,

            "Public Records" => LotDimensionsSource::PublicRecords,

            "See Remarks" => LotDimensionsSource::SeeRemarks,

            "Survey" => LotDimensionsSource::Survey,

            _ => LotDimensionsSource::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            LotDimensionsSource::Appraiser => "Appraiser",

            LotDimensionsSource::Assessor => "Assessor",

            LotDimensionsSource::Builder => "Builder",

            LotDimensionsSource::Estimated => "Estimated",

            LotDimensionsSource::GISCalculated => "GIS Calculated",

            LotDimensionsSource::Measured => "Measured",

            LotDimensionsSource::Other => "Other",

            LotDimensionsSource::Owner => "Owner",

            LotDimensionsSource::PublicRecords => "Public Records",

            LotDimensionsSource::SeeRemarks => "See Remarks",

            LotDimensionsSource::Survey => "Survey",

            LotDimensionsSource::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            LotDimensionsSource::Appraiser => "Appraiser".into(),

            LotDimensionsSource::Assessor => "Assessor".into(),

            LotDimensionsSource::Builder => "Builder".into(),

            LotDimensionsSource::Estimated => "Estimated".into(),

            LotDimensionsSource::GISCalculated => "GIS Calculated".into(),

            LotDimensionsSource::Measured => "Measured".into(),

            LotDimensionsSource::Other => "Other".into(),

            LotDimensionsSource::Owner => "Owner".into(),

            LotDimensionsSource::PublicRecords => "Public Records".into(),

            LotDimensionsSource::SeeRemarks => "See Remarks".into(),

            LotDimensionsSource::Survey => "Survey".into(),

            LotDimensionsSource::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            LotDimensionsSource::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for LotDimensionsSource {
    fn from(s: String) -> LotDimensionsSource {
        match s.as_ref() {
            "Appraiser" => LotDimensionsSource::Appraiser,

            "Assessor" => LotDimensionsSource::Assessor,

            "Builder" => LotDimensionsSource::Builder,

            "Estimated" => LotDimensionsSource::Estimated,

            "GIS Calculated" => LotDimensionsSource::GISCalculated,

            "Measured" => LotDimensionsSource::Measured,

            "Other" => LotDimensionsSource::Other,

            "Owner" => LotDimensionsSource::Owner,

            "Public Records" => LotDimensionsSource::PublicRecords,

            "See Remarks" => LotDimensionsSource::SeeRemarks,

            "Survey" => LotDimensionsSource::Survey,

            _ => LotDimensionsSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LotDimensionsSource {
    fn from(s: &str) -> LotDimensionsSource {
        match s {
            "Appraiser" => LotDimensionsSource::Appraiser,

            "Assessor" => LotDimensionsSource::Assessor,

            "Builder" => LotDimensionsSource::Builder,

            "Estimated" => LotDimensionsSource::Estimated,

            "GIS Calculated" => LotDimensionsSource::GISCalculated,

            "Measured" => LotDimensionsSource::Measured,

            "Other" => LotDimensionsSource::Other,

            "Owner" => LotDimensionsSource::Owner,

            "Public Records" => LotDimensionsSource::PublicRecords,

            "See Remarks" => LotDimensionsSource::SeeRemarks,

            "Survey" => LotDimensionsSource::Survey,

            _ => LotDimensionsSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LotDimensionsSource> for &'a str {
    fn from(s: &'a LotDimensionsSource) -> &'a str {
        match s {
            LotDimensionsSource::Appraiser => "Appraiser",

            LotDimensionsSource::Assessor => "Assessor",

            LotDimensionsSource::Builder => "Builder",

            LotDimensionsSource::Estimated => "Estimated",

            LotDimensionsSource::GISCalculated => "GIS Calculated",

            LotDimensionsSource::Measured => "Measured",

            LotDimensionsSource::Other => "Other",

            LotDimensionsSource::Owner => "Owner",

            LotDimensionsSource::PublicRecords => "Public Records",

            LotDimensionsSource::SeeRemarks => "See Remarks",

            LotDimensionsSource::Survey => "Survey",

            LotDimensionsSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LotDimensionsSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LotDimensionsSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
