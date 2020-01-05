// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RoadResponsibility Lookups](https://ddwiki.reso.org/display/DDW17/RoadResponsibility+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoadResponsibility {
    /// "[Private Maintained Road](https://ddwiki.reso.org/display/DDW17/Private+Maintained+Road)": The property's road is privately maintained.
    PrivateMaintainedRoad,

    /// "[Public Maintained Road](https://ddwiki.reso.org/display/DDW17/Public+Maintained+Road)": The property's road is publicly maintained.
    PublicMaintainedRoad,

    /// "[Road Maintenance Agreement](https://ddwiki.reso.org/display/DDW17/Road+Maintenance+Agreement)": The property has a road maintenance agreement.
    RoadMaintenanceAgreement,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for RoadResponsibility {
    fn from_str(s: &str) -> RoadResponsibility {
        match s {
            "Private Maintained Road" => RoadResponsibility::PrivateMaintainedRoad,

            "Public Maintained Road" => RoadResponsibility::PublicMaintainedRoad,

            "Road Maintenance Agreement" => RoadResponsibility::RoadMaintenanceAgreement,

            _ => RoadResponsibility::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> RoadResponsibility {
        match s.as_ref() {
            "Private Maintained Road" => RoadResponsibility::PrivateMaintainedRoad,

            "Public Maintained Road" => RoadResponsibility::PublicMaintainedRoad,

            "Road Maintenance Agreement" => RoadResponsibility::RoadMaintenanceAgreement,

            _ => RoadResponsibility::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            RoadResponsibility::PrivateMaintainedRoad => "Private Maintained Road",

            RoadResponsibility::PublicMaintainedRoad => "Public Maintained Road",

            RoadResponsibility::RoadMaintenanceAgreement => "Road Maintenance Agreement",

            RoadResponsibility::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            RoadResponsibility::PrivateMaintainedRoad => "Private Maintained Road".into(),

            RoadResponsibility::PublicMaintainedRoad => "Public Maintained Road".into(),

            RoadResponsibility::RoadMaintenanceAgreement => "Road Maintenance Agreement".into(),

            RoadResponsibility::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            RoadResponsibility::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for RoadResponsibility {
    fn from(s: String) -> RoadResponsibility {
        match s.as_ref() {
            "Private Maintained Road" => RoadResponsibility::PrivateMaintainedRoad,

            "Public Maintained Road" => RoadResponsibility::PublicMaintainedRoad,

            "Road Maintenance Agreement" => RoadResponsibility::RoadMaintenanceAgreement,

            _ => RoadResponsibility::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RoadResponsibility {
    fn from(s: &str) -> RoadResponsibility {
        match s {
            "Private Maintained Road" => RoadResponsibility::PrivateMaintainedRoad,

            "Public Maintained Road" => RoadResponsibility::PublicMaintainedRoad,

            "Road Maintenance Agreement" => RoadResponsibility::RoadMaintenanceAgreement,

            _ => RoadResponsibility::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RoadResponsibility> for &'a str {
    fn from(s: &'a RoadResponsibility) -> &'a str {
        match s {
            RoadResponsibility::PrivateMaintainedRoad => "Private Maintained Road",

            RoadResponsibility::PublicMaintainedRoad => "Public Maintained Road",

            RoadResponsibility::RoadMaintenanceAgreement => "Road Maintenance Agreement",

            RoadResponsibility::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RoadResponsibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RoadResponsibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
