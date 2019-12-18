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

pub(crate) mod option_vec_year_built_source_format {
    use super::YearBuiltSource;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<YearBuiltSource>>,
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
    ) -> Result<Option<Vec<YearBuiltSource>>, D::Error>
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
