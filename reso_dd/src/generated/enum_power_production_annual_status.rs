// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PowerProductionAnnualStatus Lookups](https://ddwiki.reso.org/display/DDW17/PowerProductionAnnualStatus+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PowerProductionAnnualStatus {
    /// "[Actual](https://ddwiki.reso.org/display/DDW17/Actual)": Annual production derived from 12 or more months of actual data. The most recent 12 months is preferred because systems can degrade, albeit slowly, over time and, more importantly, conditions (e.g., trees) can change.  Therefore older data might over- or under-estimate current production amounts.
    Actual,

    /// "[Estimated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245861)": Annual production as estimated at the time or before the system began operation.
    Estimated,

    /// "[Partially Estimated](https://ddwiki.reso.org/display/DDW17/Partially+Estimated)": Annual production derived from less than 12 months of actual data, and therefore extrapolated to estimate annual production.
    PartiallyEstimated,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PowerProductionAnnualStatus {
    fn from_str(s: &str) -> PowerProductionAnnualStatus {
        match s {
            "Actual" => PowerProductionAnnualStatus::Actual,

            "Estimated" => PowerProductionAnnualStatus::Estimated,

            "Partially Estimated" => PowerProductionAnnualStatus::PartiallyEstimated,

            _ => PowerProductionAnnualStatus::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PowerProductionAnnualStatus {
        match s.as_ref() {
            "Actual" => PowerProductionAnnualStatus::Actual,

            "Estimated" => PowerProductionAnnualStatus::Estimated,

            "Partially Estimated" => PowerProductionAnnualStatus::PartiallyEstimated,

            _ => PowerProductionAnnualStatus::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PowerProductionAnnualStatus::Actual => "Actual",

            PowerProductionAnnualStatus::Estimated => "Estimated",

            PowerProductionAnnualStatus::PartiallyEstimated => "Partially Estimated",

            PowerProductionAnnualStatus::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PowerProductionAnnualStatus::Actual => "Actual".into(),

            PowerProductionAnnualStatus::Estimated => "Estimated".into(),

            PowerProductionAnnualStatus::PartiallyEstimated => "Partially Estimated".into(),

            PowerProductionAnnualStatus::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PowerProductionAnnualStatus::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PowerProductionAnnualStatus {
    fn from(s: String) -> PowerProductionAnnualStatus {
        match s.as_ref() {
            "Actual" => PowerProductionAnnualStatus::Actual,

            "Estimated" => PowerProductionAnnualStatus::Estimated,

            "Partially Estimated" => PowerProductionAnnualStatus::PartiallyEstimated,

            _ => PowerProductionAnnualStatus::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PowerProductionAnnualStatus {
    fn from(s: &str) -> PowerProductionAnnualStatus {
        match s {
            "Actual" => PowerProductionAnnualStatus::Actual,

            "Estimated" => PowerProductionAnnualStatus::Estimated,

            "Partially Estimated" => PowerProductionAnnualStatus::PartiallyEstimated,

            _ => PowerProductionAnnualStatus::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PowerProductionAnnualStatus> for &'a str {
    fn from(s: &'a PowerProductionAnnualStatus) -> &'a str {
        match s {
            PowerProductionAnnualStatus::Actual => "Actual",

            PowerProductionAnnualStatus::Estimated => "Estimated",

            PowerProductionAnnualStatus::PartiallyEstimated => "Partially Estimated",

            PowerProductionAnnualStatus::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PowerProductionAnnualStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PowerProductionAnnualStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
