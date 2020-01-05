// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenEnergyGeneration Lookups](https://ddwiki.reso.org/display/DDW17/GreenEnergyGeneration+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenEnergyGeneration {
    /// "[Solar](https://ddwiki.reso.org/display/DDW17/Solar)": Renewable form of onsite power generation. Most common are solar photovoltaic (PV) devices which generate electricity directly from sunlight via an electronic process that occurs naturally in certain types of material, called semiconductors. Powers electrical devices or sends electricity to the grid. See: <a href="http://www.seia.org/policy/solar-technology/photovoltaic-solar-electric">http://www.seia.org/policy/solar-technology/photovoltaic-solar-electric</a>
    Solar,

    /// "[Wind](https://ddwiki.reso.org/display/DDW17/Wind)": Renewable form of onsite power generation. Wind turbines use wind to make electricity. Powers electrical devices or sends electricity to the grid. See: <a href="http://energy.gov/eere/wind/how-do-wind-turbines-work">http://energy.gov/eere/wind/how-do-wind-turbines-work</a>
    Wind,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for GreenEnergyGeneration {
    fn from_str(s: &str) -> GreenEnergyGeneration {
        match s {
            "Solar" => GreenEnergyGeneration::Solar,

            "Wind" => GreenEnergyGeneration::Wind,

            _ => GreenEnergyGeneration::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> GreenEnergyGeneration {
        match s.as_ref() {
            "Solar" => GreenEnergyGeneration::Solar,

            "Wind" => GreenEnergyGeneration::Wind,

            _ => GreenEnergyGeneration::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            GreenEnergyGeneration::Solar => "Solar",

            GreenEnergyGeneration::Wind => "Wind",

            GreenEnergyGeneration::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            GreenEnergyGeneration::Solar => "Solar".into(),

            GreenEnergyGeneration::Wind => "Wind".into(),

            GreenEnergyGeneration::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            GreenEnergyGeneration::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for GreenEnergyGeneration {
    fn from(s: String) -> GreenEnergyGeneration {
        match s.as_ref() {
            "Solar" => GreenEnergyGeneration::Solar,

            "Wind" => GreenEnergyGeneration::Wind,

            _ => GreenEnergyGeneration::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenEnergyGeneration {
    fn from(s: &str) -> GreenEnergyGeneration {
        match s {
            "Solar" => GreenEnergyGeneration::Solar,

            "Wind" => GreenEnergyGeneration::Wind,

            _ => GreenEnergyGeneration::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenEnergyGeneration> for &'a str {
    fn from(s: &'a GreenEnergyGeneration) -> &'a str {
        match s {
            GreenEnergyGeneration::Solar => "Solar",

            GreenEnergyGeneration::Wind => "Wind",

            GreenEnergyGeneration::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenEnergyGeneration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenEnergyGeneration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
