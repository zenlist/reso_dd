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

pub(crate) mod option_vec_power_production_annual_status_format {
    use super::PowerProductionAnnualStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PowerProductionAnnualStatus>>,
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
    ) -> Result<Option<Vec<PowerProductionAnnualStatus>>, D::Error>
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
