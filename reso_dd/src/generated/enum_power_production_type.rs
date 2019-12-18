// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PowerProductionType Lookups](https://ddwiki.reso.org/display/DDW17/PowerProductionType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PowerProductionType {
    /// "[Photovoltaics](https://ddwiki.reso.org/display/DDW17/Photovoltaics)": Solar photovoltaic (PV) devices which generate electricity directly from sunlight via an electronic process that occurs naturally in certain types of material, called semiconductors. Powers electrical devices or sends electricity to the grid. See: <a href="http://www.seia.org/policy/solar-technology/photovoltaic-solar-electric">http://www.seia.org/policy/solar-technology/photovoltaic-solar-electric</a>
    Photovoltaics,

    /// "[Wind](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245865)": Renewable form of onsite power generation. Wind turbines use wind to make electricity. Powers electrical devices or sends electricity to the grid. http://energy.gov/eere/wind/how-do-wind-turbines-work
    Wind,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for PowerProductionType {
    fn from(s: String) -> PowerProductionType {
        match s.as_ref() {
            "Photovoltaics" => PowerProductionType::Photovoltaics,

            "Wind" => PowerProductionType::Wind,

            _ => PowerProductionType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PowerProductionType {
    fn from(s: &str) -> PowerProductionType {
        match s {
            "Photovoltaics" => PowerProductionType::Photovoltaics,

            "Wind" => PowerProductionType::Wind,

            _ => PowerProductionType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PowerProductionType> for &'a str {
    fn from(s: &'a PowerProductionType) -> &'a str {
        match s {
            PowerProductionType::Photovoltaics => "Photovoltaics",

            PowerProductionType::Wind => "Wind",

            PowerProductionType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PowerProductionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PowerProductionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_power_production_type_format {
    use super::PowerProductionType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<PowerProductionType>>,
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
    ) -> Result<Option<Vec<PowerProductionType>>, D::Error>
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
