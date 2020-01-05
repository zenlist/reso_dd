// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ExteriorFeatures Lookups](https://ddwiki.reso.org/display/DDW17/ExteriorFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExteriorFeatures {
    /// "[Awning(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244680)": The property has one or more awnings on it's exterior.
    Awnings,

    /// "[Balcony](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244681)": The property has an exterior balcony.
    Balcony,

    /// "[Barbecue](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244682)": The property has an outdoors barbeque.
    Barbecue,

    /// "[Basketball Court](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244683)": The property has a basketball court.
    BasketballCourt,

    /// "[Boat Slip](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244684)": The property includes a boat slip.
    BoatSlip,

    /// "[Built-in Barbecue](https://ddwiki.reso.org/display/DDW17/Built-in+Barbecue)": The property has a built-in outdoor barbeque.
    BuiltinBarbecue,

    /// "[Courtyard](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244686)": The property has a courtyard.
    Courtyard,

    /// "[Covered Courtyard](https://ddwiki.reso.org/display/DDW17/Covered+Courtyard)": The property has a covered courtyard.
    CoveredCourtyard,

    /// "[Dock](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244688)": The property includes a dock.
    Dock,

    /// "[Dog Run](https://ddwiki.reso.org/display/DDW17/Dog+Run)": The property has a dog run.
    DogRun,

    /// "[Electric Grill](https://ddwiki.reso.org/display/DDW17/Electric+Grill)": The property has an outdoor electric grill.
    ElectricGrill,

    /// "[Fire Pit](https://ddwiki.reso.org/display/DDW17/Fire+Pit)": The property has an outdoor fire pit.
    FirePit,

    /// "[Garden](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244692)": The property has a garden.
    Garden,

    /// "[Gas Grill](https://ddwiki.reso.org/display/DDW17/Gas+Grill)": The property has an outdoor gas grill.
    GasGrill,

    /// "[Gray Water System](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244694)": The property has a grey water system.
    GrayWaterSystem,

    /// "[Kennel](https://ddwiki.reso.org/display/DDW17/Kennel)": The property has a kennel.
    Kennel,

    /// "[Lighting](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244696)": The property has exterior lighting.
    Lighting,

    /// "[Misting System](https://ddwiki.reso.org/display/DDW17/Misting+System)": The property has a misting system.
    MistingSystem,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244698)": The property has no exterior features.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244699)": The property has exterior features other than those in this list.
    Other,

    /// "[Outdoor Grill](https://ddwiki.reso.org/display/DDW17/Outdoor+Grill)": The property has an outdoor grill.
    OutdoorGrill,

    /// "[Outdoor Kitchen](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244701)": The property has an outdoor kitchen.
    OutdoorKitchen,

    /// "[Outdoor Shower](https://ddwiki.reso.org/display/DDW17/Outdoor+Shower)": The property has an outdoor shower.
    OutdoorShower,

    /// "[Permeable Paving](https://ddwiki.reso.org/display/DDW17/Permeable+Paving)": The property has preamble paving that allows fluids to run through the paving to the below ground or channeling.
    PermeablePaving,

    /// "[Playground](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244704)": The property has a playground.
    Playground,

    /// "[Private Entrance](https://ddwiki.reso.org/display/DDW17/Private+Entrance)": The property has a private entrance.
    PrivateEntrance,

    /// "[Private Yard](https://ddwiki.reso.org/display/DDW17/Private+Yard)": The property has a private yard.
    PrivateYard,

    /// "[Rain Barrel/Cistern(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244707)": The property has a cistern for water collection.
    RainBarrelCisterns,

    /// "[Rain Gutters](https://ddwiki.reso.org/display/DDW17/Rain+Gutters)": The structure has ran gutters.
    RainGutters,

    /// "[RV Hookup](https://ddwiki.reso.org/display/DDW17/RV+Hookup)": The property has hookups for recreational vehicles.
    RVHookup,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244710)": The property has external storage.
    Storage,

    /// "[Tennis Court(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244711)": The property has one or more tennis courts.
    TennisCourts,

    /// "[Uncovered Courtyard](https://ddwiki.reso.org/display/DDW17/Uncovered+Courtyard)": The property has an uncovered courtyard.
    UncoveredCourtyard,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ExteriorFeatures {
    fn from_str(s: &str) -> ExteriorFeatures {
        match s {
            "Awning(s)" => ExteriorFeatures::Awnings,

            "Balcony" => ExteriorFeatures::Balcony,

            "Barbecue" => ExteriorFeatures::Barbecue,

            "Basketball Court" => ExteriorFeatures::BasketballCourt,

            "Boat Slip" => ExteriorFeatures::BoatSlip,

            "Built-in Barbecue" => ExteriorFeatures::BuiltinBarbecue,

            "Courtyard" => ExteriorFeatures::Courtyard,

            "Covered Courtyard" => ExteriorFeatures::CoveredCourtyard,

            "Dock" => ExteriorFeatures::Dock,

            "Dog Run" => ExteriorFeatures::DogRun,

            "Electric Grill" => ExteriorFeatures::ElectricGrill,

            "Fire Pit" => ExteriorFeatures::FirePit,

            "Garden" => ExteriorFeatures::Garden,

            "Gas Grill" => ExteriorFeatures::GasGrill,

            "Gray Water System" => ExteriorFeatures::GrayWaterSystem,

            "Kennel" => ExteriorFeatures::Kennel,

            "Lighting" => ExteriorFeatures::Lighting,

            "Misting System" => ExteriorFeatures::MistingSystem,

            "None" => ExteriorFeatures::None,

            "Other" => ExteriorFeatures::Other,

            "Outdoor Grill" => ExteriorFeatures::OutdoorGrill,

            "Outdoor Kitchen" => ExteriorFeatures::OutdoorKitchen,

            "Outdoor Shower" => ExteriorFeatures::OutdoorShower,

            "Permeable Paving" => ExteriorFeatures::PermeablePaving,

            "Playground" => ExteriorFeatures::Playground,

            "Private Entrance" => ExteriorFeatures::PrivateEntrance,

            "Private Yard" => ExteriorFeatures::PrivateYard,

            "Rain Barrel/Cistern(s)" => ExteriorFeatures::RainBarrelCisterns,

            "Rain Gutters" => ExteriorFeatures::RainGutters,

            "RV Hookup" => ExteriorFeatures::RVHookup,

            "Storage" => ExteriorFeatures::Storage,

            "Tennis Court(s)" => ExteriorFeatures::TennisCourts,

            "Uncovered Courtyard" => ExteriorFeatures::UncoveredCourtyard,

            _ => ExteriorFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ExteriorFeatures {
        match s.as_ref() {
            "Awning(s)" => ExteriorFeatures::Awnings,

            "Balcony" => ExteriorFeatures::Balcony,

            "Barbecue" => ExteriorFeatures::Barbecue,

            "Basketball Court" => ExteriorFeatures::BasketballCourt,

            "Boat Slip" => ExteriorFeatures::BoatSlip,

            "Built-in Barbecue" => ExteriorFeatures::BuiltinBarbecue,

            "Courtyard" => ExteriorFeatures::Courtyard,

            "Covered Courtyard" => ExteriorFeatures::CoveredCourtyard,

            "Dock" => ExteriorFeatures::Dock,

            "Dog Run" => ExteriorFeatures::DogRun,

            "Electric Grill" => ExteriorFeatures::ElectricGrill,

            "Fire Pit" => ExteriorFeatures::FirePit,

            "Garden" => ExteriorFeatures::Garden,

            "Gas Grill" => ExteriorFeatures::GasGrill,

            "Gray Water System" => ExteriorFeatures::GrayWaterSystem,

            "Kennel" => ExteriorFeatures::Kennel,

            "Lighting" => ExteriorFeatures::Lighting,

            "Misting System" => ExteriorFeatures::MistingSystem,

            "None" => ExteriorFeatures::None,

            "Other" => ExteriorFeatures::Other,

            "Outdoor Grill" => ExteriorFeatures::OutdoorGrill,

            "Outdoor Kitchen" => ExteriorFeatures::OutdoorKitchen,

            "Outdoor Shower" => ExteriorFeatures::OutdoorShower,

            "Permeable Paving" => ExteriorFeatures::PermeablePaving,

            "Playground" => ExteriorFeatures::Playground,

            "Private Entrance" => ExteriorFeatures::PrivateEntrance,

            "Private Yard" => ExteriorFeatures::PrivateYard,

            "Rain Barrel/Cistern(s)" => ExteriorFeatures::RainBarrelCisterns,

            "Rain Gutters" => ExteriorFeatures::RainGutters,

            "RV Hookup" => ExteriorFeatures::RVHookup,

            "Storage" => ExteriorFeatures::Storage,

            "Tennis Court(s)" => ExteriorFeatures::TennisCourts,

            "Uncovered Courtyard" => ExteriorFeatures::UncoveredCourtyard,

            _ => ExteriorFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ExteriorFeatures::Awnings => "Awning(s)",

            ExteriorFeatures::Balcony => "Balcony",

            ExteriorFeatures::Barbecue => "Barbecue",

            ExteriorFeatures::BasketballCourt => "Basketball Court",

            ExteriorFeatures::BoatSlip => "Boat Slip",

            ExteriorFeatures::BuiltinBarbecue => "Built-in Barbecue",

            ExteriorFeatures::Courtyard => "Courtyard",

            ExteriorFeatures::CoveredCourtyard => "Covered Courtyard",

            ExteriorFeatures::Dock => "Dock",

            ExteriorFeatures::DogRun => "Dog Run",

            ExteriorFeatures::ElectricGrill => "Electric Grill",

            ExteriorFeatures::FirePit => "Fire Pit",

            ExteriorFeatures::Garden => "Garden",

            ExteriorFeatures::GasGrill => "Gas Grill",

            ExteriorFeatures::GrayWaterSystem => "Gray Water System",

            ExteriorFeatures::Kennel => "Kennel",

            ExteriorFeatures::Lighting => "Lighting",

            ExteriorFeatures::MistingSystem => "Misting System",

            ExteriorFeatures::None => "None",

            ExteriorFeatures::Other => "Other",

            ExteriorFeatures::OutdoorGrill => "Outdoor Grill",

            ExteriorFeatures::OutdoorKitchen => "Outdoor Kitchen",

            ExteriorFeatures::OutdoorShower => "Outdoor Shower",

            ExteriorFeatures::PermeablePaving => "Permeable Paving",

            ExteriorFeatures::Playground => "Playground",

            ExteriorFeatures::PrivateEntrance => "Private Entrance",

            ExteriorFeatures::PrivateYard => "Private Yard",

            ExteriorFeatures::RainBarrelCisterns => "Rain Barrel/Cistern(s)",

            ExteriorFeatures::RainGutters => "Rain Gutters",

            ExteriorFeatures::RVHookup => "RV Hookup",

            ExteriorFeatures::Storage => "Storage",

            ExteriorFeatures::TennisCourts => "Tennis Court(s)",

            ExteriorFeatures::UncoveredCourtyard => "Uncovered Courtyard",

            ExteriorFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ExteriorFeatures::Awnings => "Awning(s)".into(),

            ExteriorFeatures::Balcony => "Balcony".into(),

            ExteriorFeatures::Barbecue => "Barbecue".into(),

            ExteriorFeatures::BasketballCourt => "Basketball Court".into(),

            ExteriorFeatures::BoatSlip => "Boat Slip".into(),

            ExteriorFeatures::BuiltinBarbecue => "Built-in Barbecue".into(),

            ExteriorFeatures::Courtyard => "Courtyard".into(),

            ExteriorFeatures::CoveredCourtyard => "Covered Courtyard".into(),

            ExteriorFeatures::Dock => "Dock".into(),

            ExteriorFeatures::DogRun => "Dog Run".into(),

            ExteriorFeatures::ElectricGrill => "Electric Grill".into(),

            ExteriorFeatures::FirePit => "Fire Pit".into(),

            ExteriorFeatures::Garden => "Garden".into(),

            ExteriorFeatures::GasGrill => "Gas Grill".into(),

            ExteriorFeatures::GrayWaterSystem => "Gray Water System".into(),

            ExteriorFeatures::Kennel => "Kennel".into(),

            ExteriorFeatures::Lighting => "Lighting".into(),

            ExteriorFeatures::MistingSystem => "Misting System".into(),

            ExteriorFeatures::None => "None".into(),

            ExteriorFeatures::Other => "Other".into(),

            ExteriorFeatures::OutdoorGrill => "Outdoor Grill".into(),

            ExteriorFeatures::OutdoorKitchen => "Outdoor Kitchen".into(),

            ExteriorFeatures::OutdoorShower => "Outdoor Shower".into(),

            ExteriorFeatures::PermeablePaving => "Permeable Paving".into(),

            ExteriorFeatures::Playground => "Playground".into(),

            ExteriorFeatures::PrivateEntrance => "Private Entrance".into(),

            ExteriorFeatures::PrivateYard => "Private Yard".into(),

            ExteriorFeatures::RainBarrelCisterns => "Rain Barrel/Cistern(s)".into(),

            ExteriorFeatures::RainGutters => "Rain Gutters".into(),

            ExteriorFeatures::RVHookup => "RV Hookup".into(),

            ExteriorFeatures::Storage => "Storage".into(),

            ExteriorFeatures::TennisCourts => "Tennis Court(s)".into(),

            ExteriorFeatures::UncoveredCourtyard => "Uncovered Courtyard".into(),

            ExteriorFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ExteriorFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ExteriorFeatures {
    fn from(s: String) -> ExteriorFeatures {
        match s.as_ref() {
            "Awning(s)" => ExteriorFeatures::Awnings,

            "Balcony" => ExteriorFeatures::Balcony,

            "Barbecue" => ExteriorFeatures::Barbecue,

            "Basketball Court" => ExteriorFeatures::BasketballCourt,

            "Boat Slip" => ExteriorFeatures::BoatSlip,

            "Built-in Barbecue" => ExteriorFeatures::BuiltinBarbecue,

            "Courtyard" => ExteriorFeatures::Courtyard,

            "Covered Courtyard" => ExteriorFeatures::CoveredCourtyard,

            "Dock" => ExteriorFeatures::Dock,

            "Dog Run" => ExteriorFeatures::DogRun,

            "Electric Grill" => ExteriorFeatures::ElectricGrill,

            "Fire Pit" => ExteriorFeatures::FirePit,

            "Garden" => ExteriorFeatures::Garden,

            "Gas Grill" => ExteriorFeatures::GasGrill,

            "Gray Water System" => ExteriorFeatures::GrayWaterSystem,

            "Kennel" => ExteriorFeatures::Kennel,

            "Lighting" => ExteriorFeatures::Lighting,

            "Misting System" => ExteriorFeatures::MistingSystem,

            "None" => ExteriorFeatures::None,

            "Other" => ExteriorFeatures::Other,

            "Outdoor Grill" => ExteriorFeatures::OutdoorGrill,

            "Outdoor Kitchen" => ExteriorFeatures::OutdoorKitchen,

            "Outdoor Shower" => ExteriorFeatures::OutdoorShower,

            "Permeable Paving" => ExteriorFeatures::PermeablePaving,

            "Playground" => ExteriorFeatures::Playground,

            "Private Entrance" => ExteriorFeatures::PrivateEntrance,

            "Private Yard" => ExteriorFeatures::PrivateYard,

            "Rain Barrel/Cistern(s)" => ExteriorFeatures::RainBarrelCisterns,

            "Rain Gutters" => ExteriorFeatures::RainGutters,

            "RV Hookup" => ExteriorFeatures::RVHookup,

            "Storage" => ExteriorFeatures::Storage,

            "Tennis Court(s)" => ExteriorFeatures::TennisCourts,

            "Uncovered Courtyard" => ExteriorFeatures::UncoveredCourtyard,

            _ => ExteriorFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ExteriorFeatures {
    fn from(s: &str) -> ExteriorFeatures {
        match s {
            "Awning(s)" => ExteriorFeatures::Awnings,

            "Balcony" => ExteriorFeatures::Balcony,

            "Barbecue" => ExteriorFeatures::Barbecue,

            "Basketball Court" => ExteriorFeatures::BasketballCourt,

            "Boat Slip" => ExteriorFeatures::BoatSlip,

            "Built-in Barbecue" => ExteriorFeatures::BuiltinBarbecue,

            "Courtyard" => ExteriorFeatures::Courtyard,

            "Covered Courtyard" => ExteriorFeatures::CoveredCourtyard,

            "Dock" => ExteriorFeatures::Dock,

            "Dog Run" => ExteriorFeatures::DogRun,

            "Electric Grill" => ExteriorFeatures::ElectricGrill,

            "Fire Pit" => ExteriorFeatures::FirePit,

            "Garden" => ExteriorFeatures::Garden,

            "Gas Grill" => ExteriorFeatures::GasGrill,

            "Gray Water System" => ExteriorFeatures::GrayWaterSystem,

            "Kennel" => ExteriorFeatures::Kennel,

            "Lighting" => ExteriorFeatures::Lighting,

            "Misting System" => ExteriorFeatures::MistingSystem,

            "None" => ExteriorFeatures::None,

            "Other" => ExteriorFeatures::Other,

            "Outdoor Grill" => ExteriorFeatures::OutdoorGrill,

            "Outdoor Kitchen" => ExteriorFeatures::OutdoorKitchen,

            "Outdoor Shower" => ExteriorFeatures::OutdoorShower,

            "Permeable Paving" => ExteriorFeatures::PermeablePaving,

            "Playground" => ExteriorFeatures::Playground,

            "Private Entrance" => ExteriorFeatures::PrivateEntrance,

            "Private Yard" => ExteriorFeatures::PrivateYard,

            "Rain Barrel/Cistern(s)" => ExteriorFeatures::RainBarrelCisterns,

            "Rain Gutters" => ExteriorFeatures::RainGutters,

            "RV Hookup" => ExteriorFeatures::RVHookup,

            "Storage" => ExteriorFeatures::Storage,

            "Tennis Court(s)" => ExteriorFeatures::TennisCourts,

            "Uncovered Courtyard" => ExteriorFeatures::UncoveredCourtyard,

            _ => ExteriorFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ExteriorFeatures> for &'a str {
    fn from(s: &'a ExteriorFeatures) -> &'a str {
        match s {
            ExteriorFeatures::Awnings => "Awning(s)",

            ExteriorFeatures::Balcony => "Balcony",

            ExteriorFeatures::Barbecue => "Barbecue",

            ExteriorFeatures::BasketballCourt => "Basketball Court",

            ExteriorFeatures::BoatSlip => "Boat Slip",

            ExteriorFeatures::BuiltinBarbecue => "Built-in Barbecue",

            ExteriorFeatures::Courtyard => "Courtyard",

            ExteriorFeatures::CoveredCourtyard => "Covered Courtyard",

            ExteriorFeatures::Dock => "Dock",

            ExteriorFeatures::DogRun => "Dog Run",

            ExteriorFeatures::ElectricGrill => "Electric Grill",

            ExteriorFeatures::FirePit => "Fire Pit",

            ExteriorFeatures::Garden => "Garden",

            ExteriorFeatures::GasGrill => "Gas Grill",

            ExteriorFeatures::GrayWaterSystem => "Gray Water System",

            ExteriorFeatures::Kennel => "Kennel",

            ExteriorFeatures::Lighting => "Lighting",

            ExteriorFeatures::MistingSystem => "Misting System",

            ExteriorFeatures::None => "None",

            ExteriorFeatures::Other => "Other",

            ExteriorFeatures::OutdoorGrill => "Outdoor Grill",

            ExteriorFeatures::OutdoorKitchen => "Outdoor Kitchen",

            ExteriorFeatures::OutdoorShower => "Outdoor Shower",

            ExteriorFeatures::PermeablePaving => "Permeable Paving",

            ExteriorFeatures::Playground => "Playground",

            ExteriorFeatures::PrivateEntrance => "Private Entrance",

            ExteriorFeatures::PrivateYard => "Private Yard",

            ExteriorFeatures::RainBarrelCisterns => "Rain Barrel/Cistern(s)",

            ExteriorFeatures::RainGutters => "Rain Gutters",

            ExteriorFeatures::RVHookup => "RV Hookup",

            ExteriorFeatures::Storage => "Storage",

            ExteriorFeatures::TennisCourts => "Tennis Court(s)",

            ExteriorFeatures::UncoveredCourtyard => "Uncovered Courtyard",

            ExteriorFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ExteriorFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ExteriorFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
