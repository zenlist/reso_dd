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

pub(crate) mod option_vec_road_responsibility_format {
    use super::RoadResponsibility;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<RoadResponsibility>>,
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
    ) -> Result<Option<Vec<RoadResponsibility>>, D::Error>
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
