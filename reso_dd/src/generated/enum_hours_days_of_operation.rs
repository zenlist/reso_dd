// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [HoursDaysOfOperation Lookups](https://ddwiki.reso.org/display/DDW17/HoursDaysOfOperation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HoursDaysOfOperation {
    /// "[Evenings Only](https://ddwiki.reso.org/display/DDW17/Evenings+Only)": The business being sold is open in the evenings only.
    EveningsOnly,

    /// "[Open -8 Hours/Day](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244946)": The business being sold is open less than 8 hours per day.
    OpenLessThan8HoursDay,

    /// "[Open 24 Hours](https://ddwiki.reso.org/display/DDW17/Open+24+Hours)": The business being sold is open 24 hours per day.
    Open24Hours,

    /// "[Open 7 Days](https://ddwiki.reso.org/display/DDW17/Open+7+Days)": The business being sold is open 7 days per week.
    Open7Days,

    /// "[Open 8 Hours/Day](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244945)": The business being sold is open 8 hours per day.
    Open8HoursDay,

    /// "[Open 8+ Hours/Day](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244947)": The business being sold is open more than 8 hours/day.
    Open8PlusHoursDay,

    /// "[Open Monday-Friday](https://ddwiki.reso.org/display/DDW17/Open+Monday-Friday)": The business being sold is open Monday through Friday.
    OpenMondayFriday,

    /// "[Open Saturday](https://ddwiki.reso.org/display/DDW17/Open+Saturday)": The business being sold is open on Saturdays.
    OpenSaturday,

    /// "[Open Sunday](https://ddwiki.reso.org/display/DDW17/Open+Sunday)": The business being sold is open on Sundays.
    OpenSunday,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for HoursDaysOfOperation {
    fn from(s: String) -> HoursDaysOfOperation {
        match s.as_ref() {
            "Evenings Only" => HoursDaysOfOperation::EveningsOnly,

            "Open -8 Hours/Day" => HoursDaysOfOperation::OpenLessThan8HoursDay,

            "Open 24 Hours" => HoursDaysOfOperation::Open24Hours,

            "Open 7 Days" => HoursDaysOfOperation::Open7Days,

            "Open 8 Hours/Day" => HoursDaysOfOperation::Open8HoursDay,

            "Open 8+ Hours/Day" => HoursDaysOfOperation::Open8PlusHoursDay,

            "Open Monday-Friday" => HoursDaysOfOperation::OpenMondayFriday,

            "Open Saturday" => HoursDaysOfOperation::OpenSaturday,

            "Open Sunday" => HoursDaysOfOperation::OpenSunday,

            _ => HoursDaysOfOperation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for HoursDaysOfOperation {
    fn from(s: &str) -> HoursDaysOfOperation {
        match s {
            "Evenings Only" => HoursDaysOfOperation::EveningsOnly,

            "Open -8 Hours/Day" => HoursDaysOfOperation::OpenLessThan8HoursDay,

            "Open 24 Hours" => HoursDaysOfOperation::Open24Hours,

            "Open 7 Days" => HoursDaysOfOperation::Open7Days,

            "Open 8 Hours/Day" => HoursDaysOfOperation::Open8HoursDay,

            "Open 8+ Hours/Day" => HoursDaysOfOperation::Open8PlusHoursDay,

            "Open Monday-Friday" => HoursDaysOfOperation::OpenMondayFriday,

            "Open Saturday" => HoursDaysOfOperation::OpenSaturday,

            "Open Sunday" => HoursDaysOfOperation::OpenSunday,

            _ => HoursDaysOfOperation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a HoursDaysOfOperation> for &'a str {
    fn from(s: &'a HoursDaysOfOperation) -> &'a str {
        match s {
            HoursDaysOfOperation::EveningsOnly => "Evenings Only",

            HoursDaysOfOperation::OpenLessThan8HoursDay => "Open -8 Hours/Day",

            HoursDaysOfOperation::Open24Hours => "Open 24 Hours",

            HoursDaysOfOperation::Open7Days => "Open 7 Days",

            HoursDaysOfOperation::Open8HoursDay => "Open 8 Hours/Day",

            HoursDaysOfOperation::Open8PlusHoursDay => "Open 8+ Hours/Day",

            HoursDaysOfOperation::OpenMondayFriday => "Open Monday-Friday",

            HoursDaysOfOperation::OpenSaturday => "Open Saturday",

            HoursDaysOfOperation::OpenSunday => "Open Sunday",

            HoursDaysOfOperation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for HoursDaysOfOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for HoursDaysOfOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_hours_days_of_operation_format {
    use super::HoursDaysOfOperation;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<HoursDaysOfOperation>>,
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
    ) -> Result<Option<Vec<HoursDaysOfOperation>>, D::Error>
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
