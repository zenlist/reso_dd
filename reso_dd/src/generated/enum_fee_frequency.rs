// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [FeeFrequency Lookups](https://ddwiki.reso.org/display/DDW17/FeeFrequency+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FeeFrequency {
    /// "[Annually](https://ddwiki.reso.org/display/DDW17/Annually)": Fee is paid or received once a year.
    Annually,

    /// "[Bi-Monthly](https://ddwiki.reso.org/display/DDW17/Bi-Monthly)": Fee is paid or received every other month.
    BiMonthly,

    /// "[Bi-Weekly](https://ddwiki.reso.org/display/DDW17/Bi-Weekly)": Fee is paid or received every other week.
    BiWeekly,

    /// "[Daily](https://ddwiki.reso.org/display/DDW17/Daily)": Fee is paid or received daily.
    Daily,

    /// "[Monthly](https://ddwiki.reso.org/display/DDW17/Monthly)": Fee is paid or received once a month.
    Monthly,

    /// "[One Time](https://ddwiki.reso.org/display/DDW17/One+Time)": Fee is paid or received once and is not reoccurring.
    OneTime,

    /// "[Quarterly](https://ddwiki.reso.org/display/DDW17/Quarterly)": Fee is paid or received every three months.
    Quarterly,

    /// "[Seasonal](https://ddwiki.reso.org/display/DDW17/Seasonal)": Fee is paid or received seasonally.
    Seasonal,

    /// "[Semi-Annually](https://ddwiki.reso.org/display/DDW17/Semi-Annually)": Fee is paid or received twice a year.
    SemiAnnually,

    /// "[Semi-Monthly](https://ddwiki.reso.org/display/DDW17/Semi-Monthly)": Fee is paid or received twice a month, generally on the 1st and 15th, but that may vary.
    SemiMonthly,

    /// "[Weekly](https://ddwiki.reso.org/display/DDW17/Weekly)": Fee is paid or received weekly.
    Weekly,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for FeeFrequency {
    fn from_str(s: &str) -> FeeFrequency {
        match s {
            "Annually" => FeeFrequency::Annually,

            "Bi-Monthly" => FeeFrequency::BiMonthly,

            "Bi-Weekly" => FeeFrequency::BiWeekly,

            "Daily" => FeeFrequency::Daily,

            "Monthly" => FeeFrequency::Monthly,

            "One Time" => FeeFrequency::OneTime,

            "Quarterly" => FeeFrequency::Quarterly,

            "Seasonal" => FeeFrequency::Seasonal,

            "Semi-Annually" => FeeFrequency::SemiAnnually,

            "Semi-Monthly" => FeeFrequency::SemiMonthly,

            "Weekly" => FeeFrequency::Weekly,

            _ => FeeFrequency::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> FeeFrequency {
        match s.as_ref() {
            "Annually" => FeeFrequency::Annually,

            "Bi-Monthly" => FeeFrequency::BiMonthly,

            "Bi-Weekly" => FeeFrequency::BiWeekly,

            "Daily" => FeeFrequency::Daily,

            "Monthly" => FeeFrequency::Monthly,

            "One Time" => FeeFrequency::OneTime,

            "Quarterly" => FeeFrequency::Quarterly,

            "Seasonal" => FeeFrequency::Seasonal,

            "Semi-Annually" => FeeFrequency::SemiAnnually,

            "Semi-Monthly" => FeeFrequency::SemiMonthly,

            "Weekly" => FeeFrequency::Weekly,

            _ => FeeFrequency::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            FeeFrequency::Annually => "Annually",

            FeeFrequency::BiMonthly => "Bi-Monthly",

            FeeFrequency::BiWeekly => "Bi-Weekly",

            FeeFrequency::Daily => "Daily",

            FeeFrequency::Monthly => "Monthly",

            FeeFrequency::OneTime => "One Time",

            FeeFrequency::Quarterly => "Quarterly",

            FeeFrequency::Seasonal => "Seasonal",

            FeeFrequency::SemiAnnually => "Semi-Annually",

            FeeFrequency::SemiMonthly => "Semi-Monthly",

            FeeFrequency::Weekly => "Weekly",

            FeeFrequency::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            FeeFrequency::Annually => "Annually".into(),

            FeeFrequency::BiMonthly => "Bi-Monthly".into(),

            FeeFrequency::BiWeekly => "Bi-Weekly".into(),

            FeeFrequency::Daily => "Daily".into(),

            FeeFrequency::Monthly => "Monthly".into(),

            FeeFrequency::OneTime => "One Time".into(),

            FeeFrequency::Quarterly => "Quarterly".into(),

            FeeFrequency::Seasonal => "Seasonal".into(),

            FeeFrequency::SemiAnnually => "Semi-Annually".into(),

            FeeFrequency::SemiMonthly => "Semi-Monthly".into(),

            FeeFrequency::Weekly => "Weekly".into(),

            FeeFrequency::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            FeeFrequency::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for FeeFrequency {
    fn from(s: String) -> FeeFrequency {
        match s.as_ref() {
            "Annually" => FeeFrequency::Annually,

            "Bi-Monthly" => FeeFrequency::BiMonthly,

            "Bi-Weekly" => FeeFrequency::BiWeekly,

            "Daily" => FeeFrequency::Daily,

            "Monthly" => FeeFrequency::Monthly,

            "One Time" => FeeFrequency::OneTime,

            "Quarterly" => FeeFrequency::Quarterly,

            "Seasonal" => FeeFrequency::Seasonal,

            "Semi-Annually" => FeeFrequency::SemiAnnually,

            "Semi-Monthly" => FeeFrequency::SemiMonthly,

            "Weekly" => FeeFrequency::Weekly,

            _ => FeeFrequency::OpenEnumeration(s),
        }
    }
}

impl From<&str> for FeeFrequency {
    fn from(s: &str) -> FeeFrequency {
        match s {
            "Annually" => FeeFrequency::Annually,

            "Bi-Monthly" => FeeFrequency::BiMonthly,

            "Bi-Weekly" => FeeFrequency::BiWeekly,

            "Daily" => FeeFrequency::Daily,

            "Monthly" => FeeFrequency::Monthly,

            "One Time" => FeeFrequency::OneTime,

            "Quarterly" => FeeFrequency::Quarterly,

            "Seasonal" => FeeFrequency::Seasonal,

            "Semi-Annually" => FeeFrequency::SemiAnnually,

            "Semi-Monthly" => FeeFrequency::SemiMonthly,

            "Weekly" => FeeFrequency::Weekly,

            _ => FeeFrequency::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a FeeFrequency> for &'a str {
    fn from(s: &'a FeeFrequency) -> &'a str {
        match s {
            FeeFrequency::Annually => "Annually",

            FeeFrequency::BiMonthly => "Bi-Monthly",

            FeeFrequency::BiWeekly => "Bi-Weekly",

            FeeFrequency::Daily => "Daily",

            FeeFrequency::Monthly => "Monthly",

            FeeFrequency::OneTime => "One Time",

            FeeFrequency::Quarterly => "Quarterly",

            FeeFrequency::Seasonal => "Seasonal",

            FeeFrequency::SemiAnnually => "Semi-Annually",

            FeeFrequency::SemiMonthly => "Semi-Monthly",

            FeeFrequency::Weekly => "Weekly",

            FeeFrequency::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for FeeFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FeeFrequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
