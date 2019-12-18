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

pub(crate) mod option_vec_green_energy_generation_format {
    use super::GreenEnergyGeneration;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenEnergyGeneration>>,
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
    ) -> Result<Option<Vec<GreenEnergyGeneration>>, D::Error>
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
