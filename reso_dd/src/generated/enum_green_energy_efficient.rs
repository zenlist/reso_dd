// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenEnergyEfficient Lookups](https://ddwiki.reso.org/display/DDW17/GreenEnergyEfficient+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenEnergyEfficient {
    /// "[Appliances](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244891)": For purposes of marketing, the property has appliances that have some green/efficient rating or quality.
    Appliances,

    /// "[Construction](https://ddwiki.reso.org/display/DDW17/Construction)": For purposes of marketing, the property has construction that has some green/efficient rating or quality.
    Construction,

    /// "[Doors](https://ddwiki.reso.org/display/DDW17/Doors)": For purposes of marketing, the property has doors that have some green/efficient rating or quality.
    Doors,

    /// "[Exposure/Shade](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244894)": For purposes of marketing, the property has exposure/shade that has some green/efficient rating or quality.
    ExposureShade,

    /// "[HVAC](https://ddwiki.reso.org/display/DDW17/HVAC)": For purposes of marketing, the property has a heating, ventilation and air conditioning system that has some green/efficient rating or quality.
    HVAC,

    /// "[Incentives](https://ddwiki.reso.org/display/DDW17/Incentives)": For purposes of marketing, the property has incentives that have some green/efficiency focus.
    Incentives,

    /// "[Insulation](https://ddwiki.reso.org/display/DDW17/Insulation)": For purposes of marketing, the property has insulation that has some green/efficient rating or quality.
    Insulation,

    /// "[Lighting](https://ddwiki.reso.org/display/DDW17/Lighting)": For purposes of marketing, the property has lighting that has some green/efficient rating or quality.
    Lighting,

    /// "[Roof](https://ddwiki.reso.org/display/DDW17/Roof)": For purposes of marketing, the property has a roof that has some green/efficient rating or quality.
    Roof,

    /// "[Thermostat](https://ddwiki.reso.org/display/DDW17/Thermostat)": For purposes of marketing, the property has thermostat(s) that have some green/efficient rating or quality.
    Thermostat,

    /// "[Water Heater](https://ddwiki.reso.org/display/DDW17/Water+Heater)": For purposes of marketing, the property has a water heater that have some green/efficient rating or quality.
    WaterHeater,

    /// "[Windows](https://ddwiki.reso.org/display/DDW17/Windows)": For purposes of marketing, the property has windows that have some green/efficient rating or quality.
    Windows,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenEnergyEfficient {
    fn from(s: String) -> GreenEnergyEfficient {
        match s.as_ref() {
            "Appliances" => GreenEnergyEfficient::Appliances,

            "Construction" => GreenEnergyEfficient::Construction,

            "Doors" => GreenEnergyEfficient::Doors,

            "Exposure/Shade" => GreenEnergyEfficient::ExposureShade,

            "HVAC" => GreenEnergyEfficient::HVAC,

            "Incentives" => GreenEnergyEfficient::Incentives,

            "Insulation" => GreenEnergyEfficient::Insulation,

            "Lighting" => GreenEnergyEfficient::Lighting,

            "Roof" => GreenEnergyEfficient::Roof,

            "Thermostat" => GreenEnergyEfficient::Thermostat,

            "Water Heater" => GreenEnergyEfficient::WaterHeater,

            "Windows" => GreenEnergyEfficient::Windows,

            _ => GreenEnergyEfficient::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenEnergyEfficient {
    fn from(s: &str) -> GreenEnergyEfficient {
        match s {
            "Appliances" => GreenEnergyEfficient::Appliances,

            "Construction" => GreenEnergyEfficient::Construction,

            "Doors" => GreenEnergyEfficient::Doors,

            "Exposure/Shade" => GreenEnergyEfficient::ExposureShade,

            "HVAC" => GreenEnergyEfficient::HVAC,

            "Incentives" => GreenEnergyEfficient::Incentives,

            "Insulation" => GreenEnergyEfficient::Insulation,

            "Lighting" => GreenEnergyEfficient::Lighting,

            "Roof" => GreenEnergyEfficient::Roof,

            "Thermostat" => GreenEnergyEfficient::Thermostat,

            "Water Heater" => GreenEnergyEfficient::WaterHeater,

            "Windows" => GreenEnergyEfficient::Windows,

            _ => GreenEnergyEfficient::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenEnergyEfficient> for &'a str {
    fn from(s: &'a GreenEnergyEfficient) -> &'a str {
        match s {
            GreenEnergyEfficient::Appliances => "Appliances",

            GreenEnergyEfficient::Construction => "Construction",

            GreenEnergyEfficient::Doors => "Doors",

            GreenEnergyEfficient::ExposureShade => "Exposure/Shade",

            GreenEnergyEfficient::HVAC => "HVAC",

            GreenEnergyEfficient::Incentives => "Incentives",

            GreenEnergyEfficient::Insulation => "Insulation",

            GreenEnergyEfficient::Lighting => "Lighting",

            GreenEnergyEfficient::Roof => "Roof",

            GreenEnergyEfficient::Thermostat => "Thermostat",

            GreenEnergyEfficient::WaterHeater => "Water Heater",

            GreenEnergyEfficient::Windows => "Windows",

            GreenEnergyEfficient::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenEnergyEfficient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenEnergyEfficient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_energy_efficient_format {
    use super::GreenEnergyEfficient;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenEnergyEfficient>>,
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
    ) -> Result<Option<Vec<GreenEnergyEfficient>>, D::Error>
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
