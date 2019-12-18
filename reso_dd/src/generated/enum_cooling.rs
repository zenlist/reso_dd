// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Cooling Lookups](https://ddwiki.reso.org/display/DDW17/Cooling+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Cooling {
    /// "[Attic Fan](https://ddwiki.reso.org/display/DDW17/Attic+Fan)": The property has an attic fan.
    AtticFan,

    /// "[Ceiling Fan(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244511)": The property has one or more ceiling fans.
    CeilingFans,

    /// "[Central Air](https://ddwiki.reso.org/display/DDW17/Central+Air)": The property has central air conditioning.
    CentralAir,

    /// "[Dual](https://ddwiki.reso.org/display/DDW17/Dual)": The cooling system has two units.
    Dual,

    /// "[Ductless](https://ddwiki.reso.org/display/DDW17/Ductless)": The cooling system does not ducted nor a wall/window type unit.  A mini-split is a common type of ductless system where an outdoor condenser is connected to an indoor fan unit that feeds the room in which it's located, rather than being ducted throughout the structure.
    Ductless,

    /// "[Electric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244515)": The cooling system is powered by electricity.
    Electric,

    /// "[ENERGY STAR Qualified Equipment](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+Equipment)": The cooling system is ENERGY STAR Qualified.
    ENERGYSTARQualifiedEquipment,

    /// "[Evaporative Cooling](https://ddwiki.reso.org/display/DDW17/Evaporative+Cooling)": The cooling system works by way of water evaporation rather than a compressor and coolant.  Evaporative cooling systems are often referred to as swamp coolers.
    EvaporativeCooling,

    /// "[Exhaust Fan](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244518)": The structure has an exhaust fan.
    ExhaustFan,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244519)": The cooling system is powered by gas.
    Gas,

    /// "[Geothermal](https://ddwiki.reso.org/display/DDW17/Geothermal)": The cooling system runs on a geothermal source.
    Geothermal,

    /// "[Heat Pump](https://ddwiki.reso.org/display/DDW17/Heat+Pump)": A system that exchanges heat between a warm and cool space.  The heat exchange is done between the dwelling and another air space, like outdoors; or a water source; or below ground (geothermal).
    HeatPump,

    /// "[Humidity Control](https://ddwiki.reso.org/display/DDW17/Humidity+Control)": The cooling system includes humidity control.
    HumidityControl,

    /// "[Multi Units](https://ddwiki.reso.org/display/DDW17/Multi+Units)": The cooing system includes more than one unit.
    MultiUnits,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244524)": The property includes no cooling system.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244525)": The cooling system is different, or has features, that are not included in this list.
    Other,

    /// "[Roof Turbine(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244526)": The cooling utilizes a roof turbine.
    RoofTurbines,

    /// "[Separate Meters](https://ddwiki.reso.org/display/DDW17/Separate+Meters)": The cooling system has separate meters for its multiple units/zones.
    SeparateMeters,

    /// "[Varies by Unit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244528)": The cooling equipment varies by unit.
    VariesbyUnit,

    /// "[Wall Unit(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244530)": The cooling system is stand alone and mounted in an opening in an outer wall.
    WallUnits,

    /// "[Wall/Window Unit(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244529)": The cooling system is mounted in an opening in the wall or in a window.
    WallWindowUnits,

    /// "[Whole House Fan](https://ddwiki.reso.org/display/DDW17/Whole+House+Fan)": The property has a whole house fan.
    WholeHouseFan,

    /// "[Window Unit(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244532)": The cooling system is window mounted.
    WindowUnits,

    /// "[Zoned](https://ddwiki.reso.org/display/DDW17/Zoned)": The cooling system has more than one zone.
    Zoned,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Cooling {
    fn from(s: String) -> Cooling {
        match s.as_ref() {
            "Attic Fan" => Cooling::AtticFan,

            "Ceiling Fan(s)" => Cooling::CeilingFans,

            "Central Air" => Cooling::CentralAir,

            "Dual" => Cooling::Dual,

            "Ductless" => Cooling::Ductless,

            "Electric" => Cooling::Electric,

            "ENERGY STAR Qualified Equipment" => Cooling::ENERGYSTARQualifiedEquipment,

            "Evaporative Cooling" => Cooling::EvaporativeCooling,

            "Exhaust Fan" => Cooling::ExhaustFan,

            "Gas" => Cooling::Gas,

            "Geothermal" => Cooling::Geothermal,

            "Heat Pump" => Cooling::HeatPump,

            "Humidity Control" => Cooling::HumidityControl,

            "Multi Units" => Cooling::MultiUnits,

            "None" => Cooling::None,

            "Other" => Cooling::Other,

            "Roof Turbine(s)" => Cooling::RoofTurbines,

            "Separate Meters" => Cooling::SeparateMeters,

            "Varies by Unit" => Cooling::VariesbyUnit,

            "Wall Unit(s)" => Cooling::WallUnits,

            "Wall/Window Unit(s)" => Cooling::WallWindowUnits,

            "Whole House Fan" => Cooling::WholeHouseFan,

            "Window Unit(s)" => Cooling::WindowUnits,

            "Zoned" => Cooling::Zoned,

            _ => Cooling::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Cooling {
    fn from(s: &str) -> Cooling {
        match s {
            "Attic Fan" => Cooling::AtticFan,

            "Ceiling Fan(s)" => Cooling::CeilingFans,

            "Central Air" => Cooling::CentralAir,

            "Dual" => Cooling::Dual,

            "Ductless" => Cooling::Ductless,

            "Electric" => Cooling::Electric,

            "ENERGY STAR Qualified Equipment" => Cooling::ENERGYSTARQualifiedEquipment,

            "Evaporative Cooling" => Cooling::EvaporativeCooling,

            "Exhaust Fan" => Cooling::ExhaustFan,

            "Gas" => Cooling::Gas,

            "Geothermal" => Cooling::Geothermal,

            "Heat Pump" => Cooling::HeatPump,

            "Humidity Control" => Cooling::HumidityControl,

            "Multi Units" => Cooling::MultiUnits,

            "None" => Cooling::None,

            "Other" => Cooling::Other,

            "Roof Turbine(s)" => Cooling::RoofTurbines,

            "Separate Meters" => Cooling::SeparateMeters,

            "Varies by Unit" => Cooling::VariesbyUnit,

            "Wall Unit(s)" => Cooling::WallUnits,

            "Wall/Window Unit(s)" => Cooling::WallWindowUnits,

            "Whole House Fan" => Cooling::WholeHouseFan,

            "Window Unit(s)" => Cooling::WindowUnits,

            "Zoned" => Cooling::Zoned,

            _ => Cooling::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Cooling> for &'a str {
    fn from(s: &'a Cooling) -> &'a str {
        match s {
            Cooling::AtticFan => "Attic Fan",

            Cooling::CeilingFans => "Ceiling Fan(s)",

            Cooling::CentralAir => "Central Air",

            Cooling::Dual => "Dual",

            Cooling::Ductless => "Ductless",

            Cooling::Electric => "Electric",

            Cooling::ENERGYSTARQualifiedEquipment => "ENERGY STAR Qualified Equipment",

            Cooling::EvaporativeCooling => "Evaporative Cooling",

            Cooling::ExhaustFan => "Exhaust Fan",

            Cooling::Gas => "Gas",

            Cooling::Geothermal => "Geothermal",

            Cooling::HeatPump => "Heat Pump",

            Cooling::HumidityControl => "Humidity Control",

            Cooling::MultiUnits => "Multi Units",

            Cooling::None => "None",

            Cooling::Other => "Other",

            Cooling::RoofTurbines => "Roof Turbine(s)",

            Cooling::SeparateMeters => "Separate Meters",

            Cooling::VariesbyUnit => "Varies by Unit",

            Cooling::WallUnits => "Wall Unit(s)",

            Cooling::WallWindowUnits => "Wall/Window Unit(s)",

            Cooling::WholeHouseFan => "Whole House Fan",

            Cooling::WindowUnits => "Window Unit(s)",

            Cooling::Zoned => "Zoned",

            Cooling::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Cooling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Cooling {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_cooling_format {
    use super::Cooling;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Cooling>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Cooling>>, D::Error>
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
