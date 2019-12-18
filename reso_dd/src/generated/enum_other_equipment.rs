// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OtherEquipment Lookups](https://ddwiki.reso.org/display/DDW17/OtherEquipment+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OtherEquipment {
    /// "[Air Purifier](https://ddwiki.reso.org/display/DDW17/Air+Purifier)": The property includes an air purifier.
    AirPurifier,

    /// "[Call Listing Agent](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245816)": Call the listing agent for more information about other equipment included with the property.
    CallListingAgent,

    /// "[Compressor](https://ddwiki.reso.org/display/DDW17/Compressor)": The property includes a compressor.
    Compressor,

    /// "[DC Well Pump](https://ddwiki.reso.org/display/DDW17/DC+Well+Pump)": The property includes a DC well pump.
    DCWellPump,

    /// "[Dehumidifier](https://ddwiki.reso.org/display/DDW17/Dehumidifier)": The property includes a dehumidifier.
    Dehumidifier,

    /// "[Farm Equipment](https://ddwiki.reso.org/display/DDW17/Farm+Equipment)": The property includes farm equipment.
    FarmEquipment,

    /// "[Fuel Tank(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245821)": The property includes a fuel tank(s).
    FuelTanks,

    /// "[Generator](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245822)": The property includes a generator.
    Generator,

    /// "[Home Theater](https://ddwiki.reso.org/display/DDW17/Home+Theater)": The property includes a home theater.
    HomeTheater,

    /// "[Intercom](https://ddwiki.reso.org/display/DDW17/Intercom)": The property includes an intercom.
    Intercom,

    /// "[Irrigation Equipment](https://ddwiki.reso.org/display/DDW17/Irrigation+Equipment)": The property includes irrigation equipment.
    IrrigationEquipment,

    /// "[List Available](https://ddwiki.reso.org/display/DDW17/List+Available)": A list of other equipment included with the property is available upon request.
    ListAvailable,

    /// "[Livestock Equipment](https://ddwiki.reso.org/display/DDW17/Livestock+Equipment)": The property includes livestock equipment.
    LivestockEquipment,

    /// "[Negotiable](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245828)": The other equipment included with the property is negotiable.
    Negotiable,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245829)": There is no other equipment included with the property.
    None,

    /// "[Orchard Equipment](https://ddwiki.reso.org/display/DDW17/Orchard+Equipment)": The property includes orchard equipment.
    OrchardEquipment,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245831)": The property includes equipment other than what's included in this list.
    Other,

    /// "[Rotary Antenna](https://ddwiki.reso.org/display/DDW17/Rotary+Antenna)": The property includes a rotary antenna.
    RotaryAntenna,

    /// "[Satellite Dish](https://ddwiki.reso.org/display/DDW17/Satellite+Dish)": The property includes a satellite dish.
    SatelliteDish,

    /// "[TV Antenna](https://ddwiki.reso.org/display/DDW17/TV+Antenna)": The property includes a TV antenna.
    TVAntenna,

    /// "[Varies by Unit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245835)": The equipment included with the property varies from unit to unit.
    VariesbyUnit,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for OtherEquipment {
    fn from(s: String) -> OtherEquipment {
        match s.as_ref() {
            "Air Purifier" => OtherEquipment::AirPurifier,

            "Call Listing Agent" => OtherEquipment::CallListingAgent,

            "Compressor" => OtherEquipment::Compressor,

            "DC Well Pump" => OtherEquipment::DCWellPump,

            "Dehumidifier" => OtherEquipment::Dehumidifier,

            "Farm Equipment" => OtherEquipment::FarmEquipment,

            "Fuel Tank(s)" => OtherEquipment::FuelTanks,

            "Generator" => OtherEquipment::Generator,

            "Home Theater" => OtherEquipment::HomeTheater,

            "Intercom" => OtherEquipment::Intercom,

            "Irrigation Equipment" => OtherEquipment::IrrigationEquipment,

            "List Available" => OtherEquipment::ListAvailable,

            "Livestock Equipment" => OtherEquipment::LivestockEquipment,

            "Negotiable" => OtherEquipment::Negotiable,

            "None" => OtherEquipment::None,

            "Orchard Equipment" => OtherEquipment::OrchardEquipment,

            "Other" => OtherEquipment::Other,

            "Rotary Antenna" => OtherEquipment::RotaryAntenna,

            "Satellite Dish" => OtherEquipment::SatelliteDish,

            "TV Antenna" => OtherEquipment::TVAntenna,

            "Varies by Unit" => OtherEquipment::VariesbyUnit,

            _ => OtherEquipment::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OtherEquipment {
    fn from(s: &str) -> OtherEquipment {
        match s {
            "Air Purifier" => OtherEquipment::AirPurifier,

            "Call Listing Agent" => OtherEquipment::CallListingAgent,

            "Compressor" => OtherEquipment::Compressor,

            "DC Well Pump" => OtherEquipment::DCWellPump,

            "Dehumidifier" => OtherEquipment::Dehumidifier,

            "Farm Equipment" => OtherEquipment::FarmEquipment,

            "Fuel Tank(s)" => OtherEquipment::FuelTanks,

            "Generator" => OtherEquipment::Generator,

            "Home Theater" => OtherEquipment::HomeTheater,

            "Intercom" => OtherEquipment::Intercom,

            "Irrigation Equipment" => OtherEquipment::IrrigationEquipment,

            "List Available" => OtherEquipment::ListAvailable,

            "Livestock Equipment" => OtherEquipment::LivestockEquipment,

            "Negotiable" => OtherEquipment::Negotiable,

            "None" => OtherEquipment::None,

            "Orchard Equipment" => OtherEquipment::OrchardEquipment,

            "Other" => OtherEquipment::Other,

            "Rotary Antenna" => OtherEquipment::RotaryAntenna,

            "Satellite Dish" => OtherEquipment::SatelliteDish,

            "TV Antenna" => OtherEquipment::TVAntenna,

            "Varies by Unit" => OtherEquipment::VariesbyUnit,

            _ => OtherEquipment::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OtherEquipment> for &'a str {
    fn from(s: &'a OtherEquipment) -> &'a str {
        match s {
            OtherEquipment::AirPurifier => "Air Purifier",

            OtherEquipment::CallListingAgent => "Call Listing Agent",

            OtherEquipment::Compressor => "Compressor",

            OtherEquipment::DCWellPump => "DC Well Pump",

            OtherEquipment::Dehumidifier => "Dehumidifier",

            OtherEquipment::FarmEquipment => "Farm Equipment",

            OtherEquipment::FuelTanks => "Fuel Tank(s)",

            OtherEquipment::Generator => "Generator",

            OtherEquipment::HomeTheater => "Home Theater",

            OtherEquipment::Intercom => "Intercom",

            OtherEquipment::IrrigationEquipment => "Irrigation Equipment",

            OtherEquipment::ListAvailable => "List Available",

            OtherEquipment::LivestockEquipment => "Livestock Equipment",

            OtherEquipment::Negotiable => "Negotiable",

            OtherEquipment::None => "None",

            OtherEquipment::OrchardEquipment => "Orchard Equipment",

            OtherEquipment::Other => "Other",

            OtherEquipment::RotaryAntenna => "Rotary Antenna",

            OtherEquipment::SatelliteDish => "Satellite Dish",

            OtherEquipment::TVAntenna => "TV Antenna",

            OtherEquipment::VariesbyUnit => "Varies by Unit",

            OtherEquipment::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OtherEquipment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OtherEquipment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_other_equipment_format {
    use super::OtherEquipment;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<OtherEquipment>>,
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
    ) -> Result<Option<Vec<OtherEquipment>>, D::Error>
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
