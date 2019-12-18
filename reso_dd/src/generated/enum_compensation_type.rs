// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [CompensationType Lookups](https://ddwiki.reso.org/display/DDW17/CompensationType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompensationType {
    /// "[$](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244063)": The value entered in the BuyerAgencyCompensation field is in dollars.
    Dollar,

    /// "[%](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244064)": The value entered in the BuyerAgencyCompensation field is a percent of the gross compensation.
    Percent,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244066)": A compensation type not included in this list
    Other,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244065)": The Buyer Agency Compensation Type is something other than % or $ or is some special combination of $, %, and other compensation types.  See the applicable remarks field for more details about the compensation.
    SeeRemarks,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for CompensationType {
    fn from(s: String) -> CompensationType {
        match s.as_ref() {
            "$" => CompensationType::Dollar,

            "%" => CompensationType::Percent,

            "Other" => CompensationType::Other,

            "See Remarks" => CompensationType::SeeRemarks,

            _ => CompensationType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for CompensationType {
    fn from(s: &str) -> CompensationType {
        match s {
            "$" => CompensationType::Dollar,

            "%" => CompensationType::Percent,

            "Other" => CompensationType::Other,

            "See Remarks" => CompensationType::SeeRemarks,

            _ => CompensationType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a CompensationType> for &'a str {
    fn from(s: &'a CompensationType) -> &'a str {
        match s {
            CompensationType::Dollar => "$",

            CompensationType::Percent => "%",

            CompensationType::Other => "Other",

            CompensationType::SeeRemarks => "See Remarks",

            CompensationType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for CompensationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CompensationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_compensation_type_format {
    use super::CompensationType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<CompensationType>>,
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
    ) -> Result<Option<Vec<CompensationType>>, D::Error>
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
