// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [GreenIndoorAirQuality Lookups](https://ddwiki.reso.org/display/DDW17/GreenIndoorAirQuality+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreenIndoorAirQuality {
    /// "[Contaminant Control](https://ddwiki.reso.org/display/DDW17/Contaminant+Control)": Property has been carefully designed to prevent, monitor, and suppress pollution issues. Carefully selected low-emission materials have been used in the home. May include passive or active radon control, carbon monoxide monitoring, and high-efficiency sealed combustion for equipment such as furnaces and water heaters. May include elimination of materials that contain lead or asbestos. May include reduction of materials that contain volatile organic compounds (VOCs, including formaldehyde) and pesticides.
    ContaminantControl,

    /// "[Integrated Pest Management](https://ddwiki.reso.org/display/DDW17/Integrated+Pest+Management)": Property is designed for systematic management of pests that uses prevention, exclusion, monitoring, and suppression.
    IntegratedPestManagement,

    /// "[Moisture Control](https://ddwiki.reso.org/display/DDW17/Moisture+Control)": Every foundation, roof, roofing component, exterior wall, door, skylight, and window is designed and maintained to be watertight and free of persistent dampness or moisture.
    MoistureControl,

    /// "[Ventilation](https://ddwiki.reso.org/display/DDW17/Ventilation)": Furnaces, water heaters, woodstoves, and other devices that employ combustion-burning fuel are vented to the outside in a manner that meets manufacturer specifications to prevent back-drafting. Natural and/or mechanical ventilation delivers fresh air to every habitable room and bathroom to remove moisture laden air and other contaminants generated during cooking and bathing/showering. The air exhausted from a bathroom, toilet room, or kitchen does not vent into habitable space or an attic.
    Ventilation,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for GreenIndoorAirQuality {
    fn from(s: String) -> GreenIndoorAirQuality {
        match s.as_ref() {
            "Contaminant Control" => GreenIndoorAirQuality::ContaminantControl,

            "Integrated Pest Management" => GreenIndoorAirQuality::IntegratedPestManagement,

            "Moisture Control" => GreenIndoorAirQuality::MoistureControl,

            "Ventilation" => GreenIndoorAirQuality::Ventilation,

            _ => GreenIndoorAirQuality::OpenEnumeration(s),
        }
    }
}

impl From<&str> for GreenIndoorAirQuality {
    fn from(s: &str) -> GreenIndoorAirQuality {
        match s {
            "Contaminant Control" => GreenIndoorAirQuality::ContaminantControl,

            "Integrated Pest Management" => GreenIndoorAirQuality::IntegratedPestManagement,

            "Moisture Control" => GreenIndoorAirQuality::MoistureControl,

            "Ventilation" => GreenIndoorAirQuality::Ventilation,

            _ => GreenIndoorAirQuality::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a GreenIndoorAirQuality> for &'a str {
    fn from(s: &'a GreenIndoorAirQuality) -> &'a str {
        match s {
            GreenIndoorAirQuality::ContaminantControl => "Contaminant Control",

            GreenIndoorAirQuality::IntegratedPestManagement => "Integrated Pest Management",

            GreenIndoorAirQuality::MoistureControl => "Moisture Control",

            GreenIndoorAirQuality::Ventilation => "Ventilation",

            GreenIndoorAirQuality::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for GreenIndoorAirQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for GreenIndoorAirQuality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_green_indoor_air_quality_format {
    use super::GreenIndoorAirQuality;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<GreenIndoorAirQuality>>,
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
    ) -> Result<Option<Vec<GreenIndoorAirQuality>>, D::Error>
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
