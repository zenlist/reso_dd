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

impl crate::ResoEnumeration for CompensationType {
    fn from_str(s: &str) -> CompensationType {
        match s {
            "$" => CompensationType::Dollar,

            "%" => CompensationType::Percent,

            "Other" => CompensationType::Other,

            "See Remarks" => CompensationType::SeeRemarks,

            _ => CompensationType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> CompensationType {
        match s.as_ref() {
            "$" => CompensationType::Dollar,

            "%" => CompensationType::Percent,

            "Other" => CompensationType::Other,

            "See Remarks" => CompensationType::SeeRemarks,

            _ => CompensationType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            CompensationType::Dollar => "$",

            CompensationType::Percent => "%",

            CompensationType::Other => "Other",

            CompensationType::SeeRemarks => "See Remarks",

            CompensationType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            CompensationType::Dollar => "$".into(),

            CompensationType::Percent => "%".into(),

            CompensationType::Other => "Other".into(),

            CompensationType::SeeRemarks => "See Remarks".into(),

            CompensationType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            CompensationType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
