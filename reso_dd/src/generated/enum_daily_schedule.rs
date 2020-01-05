// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [DailySchedule Lookups](https://ddwiki.reso.org/display/DDW17/DailySchedule+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DailySchedule {
    /// "[Friday AM](https://ddwiki.reso.org/display/DDW17/Friday+AM)": The prospect (auto email) will be sent every Friday morning.
    FridayAM,

    /// "[Friday PM](https://ddwiki.reso.org/display/DDW17/Friday+PM)": The prospect (auto email) will be sent every Friday evening.
    FridayPM,

    /// "[Monday AM](https://ddwiki.reso.org/display/DDW17/Monday+AM)": The prospect (auto email) will be sent every Monday morning.
    MondayAM,

    /// "[Monday PM](https://ddwiki.reso.org/display/DDW17/Monday+PM)": The prospect (auto email) will be sent every Monday evening.
    MondayPM,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244582)": The prospect (auto email) has not been setup for any daily schedule.
    None,

    /// "[Saturday AM](https://ddwiki.reso.org/display/DDW17/Saturday+AM)": The prospect (auto email) will be sent every Saturday morning.
    SaturdayAM,

    /// "[Saturday PM](https://ddwiki.reso.org/display/DDW17/Saturday+PM)": The prospect (auto email) will be sent every Saturday evening.
    SaturdayPM,

    /// "[Sunday AM](https://ddwiki.reso.org/display/DDW17/Sunday+AM)": The prospect (auto email) will be sent every Sunday morning.
    SundayAM,

    /// "[Sunday PM](https://ddwiki.reso.org/display/DDW17/Sunday+PM)": The prospect (auto email) will be sent every Sunday evening.
    SundayPM,

    /// "[Thursday AM](https://ddwiki.reso.org/display/DDW17/Thursday+AM)": The prospect (auto email) will be sent every Thursday morning.
    ThursdayAM,

    /// "[Thursday PM](https://ddwiki.reso.org/display/DDW17/Thursday+PM)": The prospect (auto email) will be sent every Thursday evening.
    ThursdayPM,

    /// "[Tuesday AM](https://ddwiki.reso.org/display/DDW17/Tuesday+AM)": The prospect (auto email) will be sent every Tuesday morning.
    TuesdayAM,

    /// "[Tuesday PM](https://ddwiki.reso.org/display/DDW17/Tuesday+PM)": The prospect (auto email) will be sent every Tuesday evening.
    TuesdayPM,

    /// "[Wednesday AM](https://ddwiki.reso.org/display/DDW17/Wednesday+AM)": The prospect (auto email) will be sent every Wednesday morning.
    WednesdayAM,

    /// "[Wednesday PM](https://ddwiki.reso.org/display/DDW17/Wednesday+PM)": The prospect (auto email) will be sent every Wednesday evening.
    WednesdayPM,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for DailySchedule {
    fn from_str(s: &str) -> DailySchedule {
        match s {
            "Friday AM" => DailySchedule::FridayAM,

            "Friday PM" => DailySchedule::FridayPM,

            "Monday AM" => DailySchedule::MondayAM,

            "Monday PM" => DailySchedule::MondayPM,

            "None" => DailySchedule::None,

            "Saturday AM" => DailySchedule::SaturdayAM,

            "Saturday PM" => DailySchedule::SaturdayPM,

            "Sunday AM" => DailySchedule::SundayAM,

            "Sunday PM" => DailySchedule::SundayPM,

            "Thursday AM" => DailySchedule::ThursdayAM,

            "Thursday PM" => DailySchedule::ThursdayPM,

            "Tuesday AM" => DailySchedule::TuesdayAM,

            "Tuesday PM" => DailySchedule::TuesdayPM,

            "Wednesday AM" => DailySchedule::WednesdayAM,

            "Wednesday PM" => DailySchedule::WednesdayPM,

            _ => DailySchedule::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> DailySchedule {
        match s.as_ref() {
            "Friday AM" => DailySchedule::FridayAM,

            "Friday PM" => DailySchedule::FridayPM,

            "Monday AM" => DailySchedule::MondayAM,

            "Monday PM" => DailySchedule::MondayPM,

            "None" => DailySchedule::None,

            "Saturday AM" => DailySchedule::SaturdayAM,

            "Saturday PM" => DailySchedule::SaturdayPM,

            "Sunday AM" => DailySchedule::SundayAM,

            "Sunday PM" => DailySchedule::SundayPM,

            "Thursday AM" => DailySchedule::ThursdayAM,

            "Thursday PM" => DailySchedule::ThursdayPM,

            "Tuesday AM" => DailySchedule::TuesdayAM,

            "Tuesday PM" => DailySchedule::TuesdayPM,

            "Wednesday AM" => DailySchedule::WednesdayAM,

            "Wednesday PM" => DailySchedule::WednesdayPM,

            _ => DailySchedule::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            DailySchedule::FridayAM => "Friday AM",

            DailySchedule::FridayPM => "Friday PM",

            DailySchedule::MondayAM => "Monday AM",

            DailySchedule::MondayPM => "Monday PM",

            DailySchedule::None => "None",

            DailySchedule::SaturdayAM => "Saturday AM",

            DailySchedule::SaturdayPM => "Saturday PM",

            DailySchedule::SundayAM => "Sunday AM",

            DailySchedule::SundayPM => "Sunday PM",

            DailySchedule::ThursdayAM => "Thursday AM",

            DailySchedule::ThursdayPM => "Thursday PM",

            DailySchedule::TuesdayAM => "Tuesday AM",

            DailySchedule::TuesdayPM => "Tuesday PM",

            DailySchedule::WednesdayAM => "Wednesday AM",

            DailySchedule::WednesdayPM => "Wednesday PM",

            DailySchedule::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            DailySchedule::FridayAM => "Friday AM".into(),

            DailySchedule::FridayPM => "Friday PM".into(),

            DailySchedule::MondayAM => "Monday AM".into(),

            DailySchedule::MondayPM => "Monday PM".into(),

            DailySchedule::None => "None".into(),

            DailySchedule::SaturdayAM => "Saturday AM".into(),

            DailySchedule::SaturdayPM => "Saturday PM".into(),

            DailySchedule::SundayAM => "Sunday AM".into(),

            DailySchedule::SundayPM => "Sunday PM".into(),

            DailySchedule::ThursdayAM => "Thursday AM".into(),

            DailySchedule::ThursdayPM => "Thursday PM".into(),

            DailySchedule::TuesdayAM => "Tuesday AM".into(),

            DailySchedule::TuesdayPM => "Tuesday PM".into(),

            DailySchedule::WednesdayAM => "Wednesday AM".into(),

            DailySchedule::WednesdayPM => "Wednesday PM".into(),

            DailySchedule::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            DailySchedule::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for DailySchedule {
    fn from(s: String) -> DailySchedule {
        match s.as_ref() {
            "Friday AM" => DailySchedule::FridayAM,

            "Friday PM" => DailySchedule::FridayPM,

            "Monday AM" => DailySchedule::MondayAM,

            "Monday PM" => DailySchedule::MondayPM,

            "None" => DailySchedule::None,

            "Saturday AM" => DailySchedule::SaturdayAM,

            "Saturday PM" => DailySchedule::SaturdayPM,

            "Sunday AM" => DailySchedule::SundayAM,

            "Sunday PM" => DailySchedule::SundayPM,

            "Thursday AM" => DailySchedule::ThursdayAM,

            "Thursday PM" => DailySchedule::ThursdayPM,

            "Tuesday AM" => DailySchedule::TuesdayAM,

            "Tuesday PM" => DailySchedule::TuesdayPM,

            "Wednesday AM" => DailySchedule::WednesdayAM,

            "Wednesday PM" => DailySchedule::WednesdayPM,

            _ => DailySchedule::OpenEnumeration(s),
        }
    }
}

impl From<&str> for DailySchedule {
    fn from(s: &str) -> DailySchedule {
        match s {
            "Friday AM" => DailySchedule::FridayAM,

            "Friday PM" => DailySchedule::FridayPM,

            "Monday AM" => DailySchedule::MondayAM,

            "Monday PM" => DailySchedule::MondayPM,

            "None" => DailySchedule::None,

            "Saturday AM" => DailySchedule::SaturdayAM,

            "Saturday PM" => DailySchedule::SaturdayPM,

            "Sunday AM" => DailySchedule::SundayAM,

            "Sunday PM" => DailySchedule::SundayPM,

            "Thursday AM" => DailySchedule::ThursdayAM,

            "Thursday PM" => DailySchedule::ThursdayPM,

            "Tuesday AM" => DailySchedule::TuesdayAM,

            "Tuesday PM" => DailySchedule::TuesdayPM,

            "Wednesday AM" => DailySchedule::WednesdayAM,

            "Wednesday PM" => DailySchedule::WednesdayPM,

            _ => DailySchedule::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a DailySchedule> for &'a str {
    fn from(s: &'a DailySchedule) -> &'a str {
        match s {
            DailySchedule::FridayAM => "Friday AM",

            DailySchedule::FridayPM => "Friday PM",

            DailySchedule::MondayAM => "Monday AM",

            DailySchedule::MondayPM => "Monday PM",

            DailySchedule::None => "None",

            DailySchedule::SaturdayAM => "Saturday AM",

            DailySchedule::SaturdayPM => "Saturday PM",

            DailySchedule::SundayAM => "Sunday AM",

            DailySchedule::SundayPM => "Sunday PM",

            DailySchedule::ThursdayAM => "Thursday AM",

            DailySchedule::ThursdayPM => "Thursday PM",

            DailySchedule::TuesdayAM => "Tuesday AM",

            DailySchedule::TuesdayPM => "Tuesday PM",

            DailySchedule::WednesdayAM => "Wednesday AM",

            DailySchedule::WednesdayPM => "Wednesday PM",

            DailySchedule::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for DailySchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DailySchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
