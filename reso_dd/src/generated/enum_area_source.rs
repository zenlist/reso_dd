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

pub(crate) mod option_vec_area_source_format {
    use super::AreaSource;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<AreaSource>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<AreaSource>>, D::Error>
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
