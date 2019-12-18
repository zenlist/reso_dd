// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Heating Lookups](https://ddwiki.reso.org/display/DDW17/Heating+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Heating {
    /// "[Active Solar](https://ddwiki.reso.org/display/DDW17/Active+Solar)": Active solar heating systems use solar energy to heat a fluid -- either liquid or air -- and then transfer the solar heat directly to the interior space or to a storage system for later use.
    ActiveSolar,

    /// "[Baseboard](https://ddwiki.reso.org/display/DDW17/Baseboard)": Baseboard heating utilizes convection, as cold air drops from the window, it enters the baseboard heating unit where the air is warmed by heating elements, typically fins.
    Baseboard,

    /// "[Ceiling](https://ddwiki.reso.org/display/DDW17/Ceiling)": A heating unit that is installed into, or upon the surface, of the ceiling.
    Ceiling,

    /// "[Central](https://ddwiki.reso.org/display/DDW17/Central)": A system where heat is generated in one or more locations in the structure and distributed throughout the structure.  The term "Central" is commonly understood as distribution done by ducting air.  Piping a fluid to radiators is also a central type of heating, but this can be clarified with the options "Radiator" and "Forced Air".
    Central,

    /// "[Coal](https://ddwiki.reso.org/display/DDW17/Coal)": The heating system uses coal as its fuel to generate heat.
    Coal,

    /// "[Coal Stove](https://ddwiki.reso.org/display/DDW17/Coal+Stove)": A coal burning stove that is used for heat.
    CoalStove,

    /// "[Ductless](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244958)": The heating system does not have ducting like that found in central forced air systems.
    Ductless,

    /// "[Electric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244959)": A heating system that utilizes electricity and heating elements, such as coils or fins, to generate heat.
    Electric,

    /// "[ENERGY STAR Qualified Equipment](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244961)": The heating system is ENERGY STAR Qualified.  Specific performance information must be determined by review of the actual unit.
    ENERGYSTARQualifiedEquipment,

    /// "[ENERGY STAR/ACCA RSI Qualified Installation](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244960)": The heating system installation was done by an ENERGY STAR or ACCA RSI qualified contractor.
    ENERGYSTARACCARSIQualifiedInstallation,

    /// "[Exhaust Fan](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244962)": The property has an exhaust fan.
    ExhaustFan,

    /// "[Fireplace Insert](https://ddwiki.reso.org/display/DDW17/Fireplace+Insert)": The property has a fireplace insert for generating heat.
    FireplaceInsert,

    /// "[Fireplace(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244963)": The property has one or more fireplaces used to generate heat.
    Fireplaces,

    /// "[Floor Furnace](https://ddwiki.reso.org/display/DDW17/Floor+Furnace)": A radiant heating system that is mounted into the floor and distributes the heat via convection.
    FloorFurnace,

    /// "[Forced Air](https://ddwiki.reso.org/display/DDW17/Forced+Air)": The property has a forced air system, typically via ducting throughout the structure.
    ForcedAir,

    /// "[Geothermal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244967)": A geothermal heating system, also known as a ground source heat pump, transfers heat from below ground into the structure.
    Geothermal,

    /// "[Gravity](https://ddwiki.reso.org/display/DDW17/Gravity)": A gravity heating system, also known as an octopus furnace, is typically a ducted system that doesn't use a fan, but rather is designed to allow the heat to rise naturally thought the ducts to the different areas of the structure.
    Gravity,

    /// "[Heat Pump](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244969)": A system that exchanges heat between a warm and cool space.  The heat exchange is done between the dwelling and another air space, like outdoors; or a water source; or below ground (geothermal).
    HeatPump,

    /// "[Hot Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244970)": The heating system uses a boiler and pipes to deliver hot water to radiators throughout the dwelling.
    HotWater,

    /// "[Humidity Control](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244971)": The heating system has humidity control.
    HumidityControl,

    /// "[Kerosene](https://ddwiki.reso.org/display/DDW17/Kerosene)": The heating system uses kerosene as its fuel to generate heat.
    Kerosene,

    /// "[Natural Gas](https://ddwiki.reso.org/display/DDW17/Natural+Gas)": The heating system uses natural gas as its fuel to generate heat.
    NaturalGas,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244974)": The property does not have a heating system.
    None,

    /// "[Oil](https://ddwiki.reso.org/display/DDW17/Oil)": The heating system uses oil as its fuel to generate heat.
    Oil,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244976)": The property has a heating system or features that are not included in this list.
    Other,

    /// "[Passive Solar](https://ddwiki.reso.org/display/DDW17/Passive+Solar)": Passive solar is a building design where the walls, windows, floors, etc., are made to collect heat and warm the dwelling.
    PassiveSolar,

    /// "[Pellet Stove](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244978)": The property has a stove that burns compressed wood or biomass pellets to generate heat.
    PelletStove,

    /// "[Propane](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244979)": The heating system uses propane as its fuel to generate heat.
    Propane,

    /// "[Propane Stove](https://ddwiki.reso.org/display/DDW17/Propane+Stove)": The property has a stove that burns propane to generate heat.
    PropaneStove,

    /// "[Radiant](https://ddwiki.reso.org/display/DDW17/Radiant)": The heating system uses radiators to release heat within the dwelling.  The heat is typically delivered to the radiator(s) by water/steam or electricity.
    Radiant,

    /// "[Radiant Ceiling](https://ddwiki.reso.org/display/DDW17/Radiant+Ceiling)": The radiant heating element(s) are located in the ceiling.
    RadiantCeiling,

    /// "[Radiant Floor](https://ddwiki.reso.org/display/DDW17/Radiant+Floor)": The radiant heating element(s) are located in the floor.
    RadiantFloor,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244984)": See the remarks fields for additional information about the heating system included with the property.
    SeeRemarks,

    /// "[Separate Meters](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244985)": The heating system has multiple units and/or is zoned with separate meters for each zone/unit.
    SeparateMeters,

    /// "[Solar](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244986)": The property has a heating system or method that uses an unspecified type of solar heating.
    Solar,

    /// "[Space Heater](https://ddwiki.reso.org/display/DDW17/Space+Heater)": The property comes with a stand-alone space heater.
    SpaceHeater,

    /// "[Steam](https://ddwiki.reso.org/display/DDW17/Steam)": The heating system uses a boiler and pipes to deliver hot water to radiators throughout the dwelling.
    Steam,

    /// "[Varies by Unit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244989)": The type of heating or heating features vary from unit to unit.
    VariesbyUnit,

    /// "[Wall Furnace](https://ddwiki.reso.org/display/DDW17/Wall+Furnace)": Typically a ductless system that is built into a wall to deliver to the room in which it's installed.
    WallFurnace,

    /// "[Wood](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244991)": The heating system uses wood as its fuel to generate heat.
    Wood,

    /// "[Wood Stove](https://ddwiki.reso.org/display/DDW17/Wood+Stove)": The property has a stove that burns wood to generate heat.
    WoodStove,

    /// "[Zoned](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244993)": The heating system is zoned allowing for indepenant control of two or more parts of the structure.
    Zoned,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Heating {
    fn from(s: String) -> Heating {
        match s.as_ref() {
            "Active Solar" => Heating::ActiveSolar,

            "Baseboard" => Heating::Baseboard,

            "Ceiling" => Heating::Ceiling,

            "Central" => Heating::Central,

            "Coal" => Heating::Coal,

            "Coal Stove" => Heating::CoalStove,

            "Ductless" => Heating::Ductless,

            "Electric" => Heating::Electric,

            "ENERGY STAR Qualified Equipment" => Heating::ENERGYSTARQualifiedEquipment,

            "ENERGY STAR/ACCA RSI Qualified Installation" => {
                Heating::ENERGYSTARACCARSIQualifiedInstallation
            }

            "Exhaust Fan" => Heating::ExhaustFan,

            "Fireplace Insert" => Heating::FireplaceInsert,

            "Fireplace(s)" => Heating::Fireplaces,

            "Floor Furnace" => Heating::FloorFurnace,

            "Forced Air" => Heating::ForcedAir,

            "Geothermal" => Heating::Geothermal,

            "Gravity" => Heating::Gravity,

            "Heat Pump" => Heating::HeatPump,

            "Hot Water" => Heating::HotWater,

            "Humidity Control" => Heating::HumidityControl,

            "Kerosene" => Heating::Kerosene,

            "Natural Gas" => Heating::NaturalGas,

            "None" => Heating::None,

            "Oil" => Heating::Oil,

            "Other" => Heating::Other,

            "Passive Solar" => Heating::PassiveSolar,

            "Pellet Stove" => Heating::PelletStove,

            "Propane" => Heating::Propane,

            "Propane Stove" => Heating::PropaneStove,

            "Radiant" => Heating::Radiant,

            "Radiant Ceiling" => Heating::RadiantCeiling,

            "Radiant Floor" => Heating::RadiantFloor,

            "See Remarks" => Heating::SeeRemarks,

            "Separate Meters" => Heating::SeparateMeters,

            "Solar" => Heating::Solar,

            "Space Heater" => Heating::SpaceHeater,

            "Steam" => Heating::Steam,

            "Varies by Unit" => Heating::VariesbyUnit,

            "Wall Furnace" => Heating::WallFurnace,

            "Wood" => Heating::Wood,

            "Wood Stove" => Heating::WoodStove,

            "Zoned" => Heating::Zoned,

            _ => Heating::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Heating {
    fn from(s: &str) -> Heating {
        match s {
            "Active Solar" => Heating::ActiveSolar,

            "Baseboard" => Heating::Baseboard,

            "Ceiling" => Heating::Ceiling,

            "Central" => Heating::Central,

            "Coal" => Heating::Coal,

            "Coal Stove" => Heating::CoalStove,

            "Ductless" => Heating::Ductless,

            "Electric" => Heating::Electric,

            "ENERGY STAR Qualified Equipment" => Heating::ENERGYSTARQualifiedEquipment,

            "ENERGY STAR/ACCA RSI Qualified Installation" => {
                Heating::ENERGYSTARACCARSIQualifiedInstallation
            }

            "Exhaust Fan" => Heating::ExhaustFan,

            "Fireplace Insert" => Heating::FireplaceInsert,

            "Fireplace(s)" => Heating::Fireplaces,

            "Floor Furnace" => Heating::FloorFurnace,

            "Forced Air" => Heating::ForcedAir,

            "Geothermal" => Heating::Geothermal,

            "Gravity" => Heating::Gravity,

            "Heat Pump" => Heating::HeatPump,

            "Hot Water" => Heating::HotWater,

            "Humidity Control" => Heating::HumidityControl,

            "Kerosene" => Heating::Kerosene,

            "Natural Gas" => Heating::NaturalGas,

            "None" => Heating::None,

            "Oil" => Heating::Oil,

            "Other" => Heating::Other,

            "Passive Solar" => Heating::PassiveSolar,

            "Pellet Stove" => Heating::PelletStove,

            "Propane" => Heating::Propane,

            "Propane Stove" => Heating::PropaneStove,

            "Radiant" => Heating::Radiant,

            "Radiant Ceiling" => Heating::RadiantCeiling,

            "Radiant Floor" => Heating::RadiantFloor,

            "See Remarks" => Heating::SeeRemarks,

            "Separate Meters" => Heating::SeparateMeters,

            "Solar" => Heating::Solar,

            "Space Heater" => Heating::SpaceHeater,

            "Steam" => Heating::Steam,

            "Varies by Unit" => Heating::VariesbyUnit,

            "Wall Furnace" => Heating::WallFurnace,

            "Wood" => Heating::Wood,

            "Wood Stove" => Heating::WoodStove,

            "Zoned" => Heating::Zoned,

            _ => Heating::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Heating> for &'a str {
    fn from(s: &'a Heating) -> &'a str {
        match s {
            Heating::ActiveSolar => "Active Solar",

            Heating::Baseboard => "Baseboard",

            Heating::Ceiling => "Ceiling",

            Heating::Central => "Central",

            Heating::Coal => "Coal",

            Heating::CoalStove => "Coal Stove",

            Heating::Ductless => "Ductless",

            Heating::Electric => "Electric",

            Heating::ENERGYSTARQualifiedEquipment => "ENERGY STAR Qualified Equipment",

            Heating::ENERGYSTARACCARSIQualifiedInstallation => {
                "ENERGY STAR/ACCA RSI Qualified Installation"
            }

            Heating::ExhaustFan => "Exhaust Fan",

            Heating::FireplaceInsert => "Fireplace Insert",

            Heating::Fireplaces => "Fireplace(s)",

            Heating::FloorFurnace => "Floor Furnace",

            Heating::ForcedAir => "Forced Air",

            Heating::Geothermal => "Geothermal",

            Heating::Gravity => "Gravity",

            Heating::HeatPump => "Heat Pump",

            Heating::HotWater => "Hot Water",

            Heating::HumidityControl => "Humidity Control",

            Heating::Kerosene => "Kerosene",

            Heating::NaturalGas => "Natural Gas",

            Heating::None => "None",

            Heating::Oil => "Oil",

            Heating::Other => "Other",

            Heating::PassiveSolar => "Passive Solar",

            Heating::PelletStove => "Pellet Stove",

            Heating::Propane => "Propane",

            Heating::PropaneStove => "Propane Stove",

            Heating::Radiant => "Radiant",

            Heating::RadiantCeiling => "Radiant Ceiling",

            Heating::RadiantFloor => "Radiant Floor",

            Heating::SeeRemarks => "See Remarks",

            Heating::SeparateMeters => "Separate Meters",

            Heating::Solar => "Solar",

            Heating::SpaceHeater => "Space Heater",

            Heating::Steam => "Steam",

            Heating::VariesbyUnit => "Varies by Unit",

            Heating::WallFurnace => "Wall Furnace",

            Heating::Wood => "Wood",

            Heating::WoodStove => "Wood Stove",

            Heating::Zoned => "Zoned",

            Heating::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Heating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Heating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_heating_format {
    use super::Heating;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Heating>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Heating>>, D::Error>
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
