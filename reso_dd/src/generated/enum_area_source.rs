// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [AreaSource Lookups](https://ddwiki.reso.org/display/DDW17/AreaSource+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AreaSource {
    /// "[Appraiser](https://ddwiki.reso.org/display/DDW17/Appraiser)": An appraiser provided the measurement of the area.
    Appraiser,

    /// "[Assessor](https://ddwiki.reso.org/display/DDW17/Assessor)": The assessor provided the measurement of the area.
    Assessor,

    /// "[Builder](https://ddwiki.reso.org/display/DDW17/Builder)": The builder provided the measurement of the area.
    Builder,

    /// "[Estimated](https://ddwiki.reso.org/display/DDW17/Estimated)": The measurement of the area is an estimate.
    Estimated,

    /// "[Other](https://ddwiki.reso.org/display/DDW17/Other)": The measurement of the area was provided by another party not listed.
    Other,

    /// "[Owner](https://ddwiki.reso.org/display/DDW17/Owner)": The owner provided the measurement of the area.
    Owner,

    /// "[Plans](https://ddwiki.reso.org/display/DDW17/Plans)": The measurement of the area was taken from building plans.
    Plans,

    /// "[Public Records](https://ddwiki.reso.org/display/DDW17/Public+Records)": The measurement of the area was received from public records.
    PublicRecords,

    /// "[See Remarks](https://ddwiki.reso.org/display/DDW17/See+Remarks)": See remarks for information about the source of the area measurement.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for AreaSource {
    fn from_str(s: &str) -> AreaSource {
        match s {
            "Appraiser" => AreaSource::Appraiser,

            "Assessor" => AreaSource::Assessor,

            "Builder" => AreaSource::Builder,

            "Estimated" => AreaSource::Estimated,

            "Other" => AreaSource::Other,

            "Owner" => AreaSource::Owner,

            "Plans" => AreaSource::Plans,

            "Public Records" => AreaSource::PublicRecords,

            "See Remarks" => AreaSource::SeeRemarks,

            _ => AreaSource::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> AreaSource {
        match s.as_ref() {
            "Appraiser" => AreaSource::Appraiser,

            "Assessor" => AreaSource::Assessor,

            "Builder" => AreaSource::Builder,

            "Estimated" => AreaSource::Estimated,

            "Other" => AreaSource::Other,

            "Owner" => AreaSource::Owner,

            "Plans" => AreaSource::Plans,

            "Public Records" => AreaSource::PublicRecords,

            "See Remarks" => AreaSource::SeeRemarks,

            _ => AreaSource::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            AreaSource::Appraiser => "Appraiser",

            AreaSource::Assessor => "Assessor",

            AreaSource::Builder => "Builder",

            AreaSource::Estimated => "Estimated",

            AreaSource::Other => "Other",

            AreaSource::Owner => "Owner",

            AreaSource::Plans => "Plans",

            AreaSource::PublicRecords => "Public Records",

            AreaSource::SeeRemarks => "See Remarks",

            AreaSource::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            AreaSource::Appraiser => "Appraiser".into(),

            AreaSource::Assessor => "Assessor".into(),

            AreaSource::Builder => "Builder".into(),

            AreaSource::Estimated => "Estimated".into(),

            AreaSource::Other => "Other".into(),

            AreaSource::Owner => "Owner".into(),

            AreaSource::Plans => "Plans".into(),

            AreaSource::PublicRecords => "Public Records".into(),

            AreaSource::SeeRemarks => "See Remarks".into(),

            AreaSource::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            AreaSource::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for AreaSource {
    fn from(s: String) -> AreaSource {
        match s.as_ref() {
            "Appraiser" => AreaSource::Appraiser,

            "Assessor" => AreaSource::Assessor,

            "Builder" => AreaSource::Builder,

            "Estimated" => AreaSource::Estimated,

            "Other" => AreaSource::Other,

            "Owner" => AreaSource::Owner,

            "Plans" => AreaSource::Plans,

            "Public Records" => AreaSource::PublicRecords,

            "See Remarks" => AreaSource::SeeRemarks,

            _ => AreaSource::OpenEnumeration(s),
        }
    }
}

impl From<&str> for AreaSource {
    fn from(s: &str) -> AreaSource {
        match s {
            "Appraiser" => AreaSource::Appraiser,

            "Assessor" => AreaSource::Assessor,

            "Builder" => AreaSource::Builder,

            "Estimated" => AreaSource::Estimated,

            "Other" => AreaSource::Other,

            "Owner" => AreaSource::Owner,

            "Plans" => AreaSource::Plans,

            "Public Records" => AreaSource::PublicRecords,

            "See Remarks" => AreaSource::SeeRemarks,

            _ => AreaSource::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a AreaSource> for &'a str {
    fn from(s: &'a AreaSource) -> &'a str {
        match s {
            AreaSource::Appraiser => "Appraiser",

            AreaSource::Assessor => "Assessor",

            AreaSource::Builder => "Builder",

            AreaSource::Estimated => "Estimated",

            AreaSource::Other => "Other",

            AreaSource::Owner => "Owner",

            AreaSource::Plans => "Plans",

            AreaSource::PublicRecords => "Public Records",

            AreaSource::SeeRemarks => "See Remarks",

            AreaSource::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for AreaSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AreaSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
