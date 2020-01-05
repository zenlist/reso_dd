// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ScheduleType Lookups](https://ddwiki.reso.org/display/DDW17/ScheduleType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ScheduleType {
    /// "[ASAP](https://ddwiki.reso.org/display/DDW17/ASAP)": The prospect (auto email) will be sent as soon as possible through each day. Actual time is determined by listing additions/changes that match the given criteria and by the host system's delivery policy/schedule.
    ASAP,

    /// "[Daily](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246434)": The prospect (auto email) will be sent according to the Daily Schedule.
    Daily,

    /// "[Monthly](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246435)": The prospect (auto email) will be sent once per month.
    Monthly,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ScheduleType {
    fn from_str(s: &str) -> ScheduleType {
        match s {
            "ASAP" => ScheduleType::ASAP,

            "Daily" => ScheduleType::Daily,

            "Monthly" => ScheduleType::Monthly,

            _ => ScheduleType::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ScheduleType {
        match s.as_ref() {
            "ASAP" => ScheduleType::ASAP,

            "Daily" => ScheduleType::Daily,

            "Monthly" => ScheduleType::Monthly,

            _ => ScheduleType::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ScheduleType::ASAP => "ASAP",

            ScheduleType::Daily => "Daily",

            ScheduleType::Monthly => "Monthly",

            ScheduleType::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ScheduleType::ASAP => "ASAP".into(),

            ScheduleType::Daily => "Daily".into(),

            ScheduleType::Monthly => "Monthly".into(),

            ScheduleType::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ScheduleType::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ScheduleType {
    fn from(s: String) -> ScheduleType {
        match s.as_ref() {
            "ASAP" => ScheduleType::ASAP,

            "Daily" => ScheduleType::Daily,

            "Monthly" => ScheduleType::Monthly,

            _ => ScheduleType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ScheduleType {
    fn from(s: &str) -> ScheduleType {
        match s {
            "ASAP" => ScheduleType::ASAP,

            "Daily" => ScheduleType::Daily,

            "Monthly" => ScheduleType::Monthly,

            _ => ScheduleType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ScheduleType> for &'a str {
    fn from(s: &'a ScheduleType) -> &'a str {
        match s {
            ScheduleType::ASAP => "ASAP",

            ScheduleType::Daily => "Daily",

            ScheduleType::Monthly => "Monthly",

            ScheduleType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ScheduleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ScheduleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
