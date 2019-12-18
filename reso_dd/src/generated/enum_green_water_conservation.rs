// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenWaterConservation Lookups](https://ddwiki.reso.org/display/DDW17/GreenWaterConservation+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenWaterConservation {
    /// "[Efficient Hot Water Distribution](https://ddwiki.reso.org/display/DDW17/Efficient+Hot+Water+Distribution)": Efficient hot water distribution systems are designed to generate hot water using fewer fuel resources, and to get hot water to low-flow faucets and fixtures more quickly. These systems often feature carefully designed plumbing lines that are less redundant and/or shorter. Rather than measuring time to hot water at a fixture in gallons, efficient distribution systems can be measured in cups. EPA WaterSense Guide for Hot Water Distribution as well as several green building programs have further details. This may also be known as Structured Plumbing.
    EfficientHotWaterDistribution,

    /// "[Gray Water System](https://ddwiki.reso.org/display/DDW17/Gray+Water+System)": The property includes a grey water system.
    GrayWaterSystem,

    /// "[Green Infrastructure](https://ddwiki.reso.org/display/DDW17/Green+Infrastructure)": Green Infrastructure is a set of strategies and specifically designed systems to manage stormwater runoff through a variety of small, cost-effective landscape features located on a property. Green Infrastructure employs infiltration (allowing water to slowly sink into the soil), evaporation/transpiration using native vegetation, and rainwater capture and reuse (storing runoff to water plants, flush toilets, etc.). May include green roof, rain gardens, rain barrels, permeable paving, etc. EPA Green Infrastructure webpage has more information
    GreenInfrastructure,

    /// "[Low-Flow Fixtures](https://ddwiki.reso.org/display/DDW17/Low-Flow+Fixtures)": Toilets, bathroom faucets, showerheads, irrigation controllers, and other products can be manufactured to use less water than minimum standards. Some products are qualified by third-party programs like the EPA WaterSense and are typically at least 20 percent more water-efficient than standard products. EPA WaterSense website has additional information
    LowFlowFixtures,

    /// "[Water Recycling](https://ddwiki.reso.org/display/DDW17/Water+Recycling)": The property includes a system to reuse stormwater via rain barrels or cisterns for landscaping, or to treat and reuse water from bathroom sinks, showers, and clothes washing drains for landscape irrigation and/or toilet flushing.
    WaterRecycling,

    /// "[Water-Smart Landscaping](https://ddwiki.reso.org/display/DDW17/Water-Smart+Landscaping)": Water-smart landscapes are designed to require less water and fertilizer treatments. These landscapes feature regionally appropriate plants that require low water and are native to the local climate. Plants are organized by hydrozones (watering needs). Any irrigation system is qualified for high water-efficiency. Turfgrass is minimized and grown to the tallest height recommended. Strategic maintenance includes mulching and soil aeration. Other details are documented in EPA’s Water-Smart Landscape Design Tips.
    WaterSmartLandscaping,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenWaterConservation {
    fn from(s: String) -> GreenWaterConservation {
        match s.as_ref() {
            "Efficient Hot Water Distribution" => {
                GreenWaterConservation::EfficientHotWaterDistribution
            }

            "Gray Water System" => GreenWaterConservation::GrayWaterSystem,

            "Green Infrastructure" => GreenWaterConservation::GreenInfrastructure,

            "Low-Flow Fixtures" => GreenWaterConservation::LowFlowFixtures,

            "Water Recycling" => GreenWaterConservation::WaterRecycling,

            "Water-Smart Landscaping" => GreenWaterConservation::WaterSmartLandscaping,

            _ => GreenWaterConservation::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenWaterConservation {
    fn from(s: &str) -> GreenWaterConservation {
        match s {
            "Efficient Hot Water Distribution" => {
                GreenWaterConservation::EfficientHotWaterDistribution
            }

            "Gray Water System" => GreenWaterConservation::GrayWaterSystem,

            "Green Infrastructure" => GreenWaterConservation::GreenInfrastructure,

            "Low-Flow Fixtures" => GreenWaterConservation::LowFlowFixtures,

            "Water Recycling" => GreenWaterConservation::WaterRecycling,

            "Water-Smart Landscaping" => GreenWaterConservation::WaterSmartLandscaping,

            _ => GreenWaterConservation::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenWaterConservation> for &'a str {
    fn from(s: &'a GreenWaterConservation) -> &'a str {
        match s {
            GreenWaterConservation::EfficientHotWaterDistribution => {
                "Efficient Hot Water Distribution"
            }

            GreenWaterConservation::GrayWaterSystem => "Gray Water System",

            GreenWaterConservation::GreenInfrastructure => "Green Infrastructure",

            GreenWaterConservation::LowFlowFixtures => "Low-Flow Fixtures",

            GreenWaterConservation::WaterRecycling => "Water Recycling",

            GreenWaterConservation::WaterSmartLandscaping => "Water-Smart Landscaping",

            GreenWaterConservation::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenWaterConservation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenWaterConservation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_water_conservation_format {
    use super::GreenWaterConservation;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenWaterConservation>>,
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
    ) -> Result<Option<Vec<GreenWaterConservation>>, D::Error>
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
