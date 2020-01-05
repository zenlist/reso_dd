// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [InteriorOrRoomFeatures Lookups](https://ddwiki.reso.org/display/DDW17/InteriorOrRoomFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InteriorOrRoomFeatures {
    /// "[Bar](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245100)": A built-in or movable fixture for the storage, preparation, serving and/or consumption of drinks.
    Bar,

    /// "[Beamed Ceilings](https://ddwiki.reso.org/display/DDW17/Beamed+Ceilings)": A property where the room, or rooms, have exposed beams across the ceiling.
    BeamedCeilings,

    /// "[Bidet](https://ddwiki.reso.org/display/DDW17/Bidet)": A type of sink designed to wash the undercarriage of the human body.
    Bidet,

    /// "[Bookcases](https://ddwiki.reso.org/display/DDW17/Bookcases)": Shelfs for books or other objects which may or may not be built into the property.
    Bookcases,

    /// "[Breakfast Bar](https://ddwiki.reso.org/display/DDW17/Breakfast+Bar)": A surface designed for eating, which is typically smaller than dining table and attached to the other kitchen surfaces.
    BreakfastBar,

    /// "[Built-in Features](https://ddwiki.reso.org/display/DDW17/Built-in+Features)": Some features are physically attached to the structure.
    BuiltinFeatures,

    /// "[Cathedral Ceiling(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245106)": A type of vaulted ceiling that is typically higher than normal ceilings and has a slant or curve to reach it's upper most point, which tends to be equal distance from the two shorter walls in the room.
    CathedralCeilings,

    /// "[Cedar Closet(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245107)": A closet that is partially or fully lined with cedar wood.
    CedarClosets,

    /// "[Ceiling Fan(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245108)": The room(s) have fans that are mounted from the ceiling.
    CeilingFans,

    /// "[Central Vacuum](https://ddwiki.reso.org/display/DDW17/Central+Vacuum)": A built-in vacuum that typically consists of a power/collection unit that is typically install in a garage or closet, tubing from the power unit to rooms thought the house, and wall mounted receptacles for the connection of a movable vacuum hose.
    CentralVacuum,

    /// "[Chandelier](https://ddwiki.reso.org/display/DDW17/Chandelier)": A decorative lighting fixture that typically branches out with several lights (or candles) with other decorative components such as glass, crystal or other reflective or light enhancing materials.
    Chandelier,

    /// "[Coffered Ceiling(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245111)": A ceiling with multiple decorative indentations, trays or sunken panels.
    CofferedCeilings,

    /// "[Crown Molding](https://ddwiki.reso.org/display/DDW17/Crown+Molding)": A decorative trim covering the seam between the ceiling and walls.
    CrownMolding,

    /// "[Double Vanity](https://ddwiki.reso.org/display/DDW17/Double+Vanity)": Bathroom cabinetry with two built-in sinks.
    DoubleVanity,

    /// "[Dry Bar](https://ddwiki.reso.org/display/DDW17/Dry+Bar)": A built-in or movable fixture for the storage, preparation, serving and consumption of drinks that does not have a water supply or sink.
    DryBar,

    /// "[Dumbwaiter](https://ddwiki.reso.org/display/DDW17/Dumbwaiter)": A small elevator, typically for carrying food between floors in a structure.
    Dumbwaiter,

    /// "[Eat-in Kitchen](https://ddwiki.reso.org/display/DDW17/Eat-in+Kitchen)": A kitchen that has been designed to accommodate dining.
    EatinKitchen,

    /// "[Elevator](https://ddwiki.reso.org/display/DDW17/Elevator)": A platform or compartment housed within a shaft for raising or lowering people or objects.
    Elevator,

    /// "[Entrance Foyer](https://ddwiki.reso.org/display/DDW17/Entrance+Foyer)": A room or hall at the entrance leading to other parts of the structure.
    EntranceFoyer,

    /// "[Granite Counters](https://ddwiki.reso.org/display/DDW17/Granite+Counters)": The counters are made of a type of granite stone.
    GraniteCounters,

    /// "[High Ceilings](https://ddwiki.reso.org/display/DDW17/High+Ceilings)": The ceiling height is greater than what might be considered a normal celling height.
    HighCeilings,

    /// "[High Speed Internet](https://ddwiki.reso.org/display/DDW17/High+Speed+Internet)": The property has access to high speed internet service, but may or may not be wired and/or connected to that service.
    HighSpeedInternet,

    /// "[His and Hers Closets](https://ddwiki.reso.org/display/DDW17/His+and+Hers+Closets)": The room(s) have two separate closets.
    HisandHersClosets,

    /// "[In-Law Floorplan](https://ddwiki.reso.org/display/DDW17/In-Law+Floorplan)": The structure has an area within that has the characteristics of an independent apartment.  Typically with a living area, kitchen, bedroom and bathroom, and in-law floor plan is not necessarily an Accessory Dwelling Unit (ADU).
    InLawFloorplan,

    /// "[Kitchen Island](https://ddwiki.reso.org/display/DDW17/Kitchen+Island)": A separate counter surface in a kitchen that is not attached to other surfaces or to a wall.  A kitchen island may or may not include a sink, stove or other fixtures.
    KitchenIsland,

    /// "[Laminate Counters](https://ddwiki.reso.org/display/DDW17/Laminate+Counters)": The counters are covered with a laminate.
    LaminateCounters,

    /// "[Low Flow Plumbing Fixtures](https://ddwiki.reso.org/display/DDW17/Low+Flow+Plumbing+Fixtures)": Some or all of the fixtures are designed to save water.
    LowFlowPlumbingFixtures,

    /// "[Master Downstairs](https://ddwiki.reso.org/display/DDW17/Master+Downstairs)": There is a master bedroom on the main level of the structure.
    MasterDownstairs,

    /// "[Natural Woodwork](https://ddwiki.reso.org/display/DDW17/Natural+Woodwork)": The property or room has features made from real wood.
    NaturalWoodwork,

    /// "[Open Floorplan](https://ddwiki.reso.org/display/DDW17/Open+Floorplan)": A generic design term for a floor plan that makes use of large open spaces and avoids the use of small enclosed spaces.
    OpenFloorplan,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245130)": The room or interior has features other than those included in this list.
    Other,

    /// "[Pantry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245131)": A small room or closet where food, dishes and utensils are stored.
    Pantry,

    /// "[Recessed Lighting](https://ddwiki.reso.org/display/DDW17/Recessed+Lighting)": A light fixture installed into a hallow opening in the celling.
    RecessedLighting,

    /// "[Sauna](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245133)": A small room or separate structure designed to produce heat, wet with steam, or dry, to induce perspiration.
    Sauna,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245134)": See the remarks fields for additional information about the room or interior.
    SeeRemarks,

    /// "[Smart Home](https://ddwiki.reso.org/display/DDW17/Smart+Home)": Smart Home features are a generic term for electronic automation of features such as lighting, heating/cooling, security and other amenities.  The features are typically linked though an app or software via one or more third party services. The features are also known for their convenience and energy efficiency.
    SmartHome,

    /// "[Smart Thermostat](https://ddwiki.reso.org/display/DDW17/Smart+Thermostat)": A heating/cooling control unit that has convenience and energy saving aspects.  A smart thermostat may also integrate with a larger smart home system and typically operates through a third party service.
    SmartThermostat,

    /// "[Soaking Tub](https://ddwiki.reso.org/display/DDW17/Soaking+Tub)": A bath tub that is typically deeper and may be shorter than traditional tubs.
    SoakingTub,

    /// "[Solar Tube(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245138)": A reflective tube that extends from a light gathering surface on the roof of the structure down into a room where the outside light is distributed.
    SolarTubes,

    /// "[Sound System](https://ddwiki.reso.org/display/DDW17/Sound+System)": The includes a sound system.  This typically includes in-wall wiring and recessed/built-in speakers and a built in location for the amplifier and other audio equipment.
    SoundSystem,

    /// "[Stone Counters](https://ddwiki.reso.org/display/DDW17/Stone+Counters)": The property or room has counters that are made of some type of stone.
    StoneCounters,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245141)": The property or room has storage space.
    Storage,

    /// "[Tile Counters](https://ddwiki.reso.org/display/DDW17/Tile+Counters)": The property or room has counters that are made of some type of tile.
    TileCounters,

    /// "[Track Lighting](https://ddwiki.reso.org/display/DDW17/Track+Lighting)": A type of lighting where the light fixtures are mounted on a track allowing for adjustment of the position of the lights.
    TrackLighting,

    /// "[Tray Ceiling(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245144)": A ceiling with a inverted tray or recessed area, often rectangular, that adds depth and interest.
    TrayCeilings,

    /// "[Vaulted Ceiling(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245145)": From the Italian word Volta, is typically a high celling with no attic between the ceiling and the roof.  When a vaulted celling has two angles that meet in the center of the room, you may use Cathedral Ceiling(s).
    VaultedCeilings,

    /// "[Walk-In Closet(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245146)": A closet that is a small room with an entryway.
    WalkInClosets,

    /// "[WaterSense Fixture(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245147)": Water fixtures that are backed by independent, third–party certification and meet EPA’s specifications for water efficiency and performance.
    WaterSenseFixtures,

    /// "[Wet Bar](https://ddwiki.reso.org/display/DDW17/Wet+Bar)": Commonly a built-in fixture for the storage, preparation, serving and/or consumption of drinks that has a faucet and sink.
    WetBar,

    /// "[Wired for Data](https://ddwiki.reso.org/display/DDW17/Wired+for+Data)": The property has been wired for data, typically Category 5 or 6 wiring for the support of ethernet data communications.
    WiredforData,

    /// "[Wired for Sound](https://ddwiki.reso.org/display/DDW17/Wired+for+Sound)": The property has been wired for a built-in sound system.  This typically includes in-wall wiring and recessed/built-in speakers and a location for audio equipment.  The wiring is in place, but equipment may not be included.
    WiredforSound,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for InteriorOrRoomFeatures {
    fn from_str(s: &str) -> InteriorOrRoomFeatures {
        match s {
            "Bar" => InteriorOrRoomFeatures::Bar,

            "Beamed Ceilings" => InteriorOrRoomFeatures::BeamedCeilings,

            "Bidet" => InteriorOrRoomFeatures::Bidet,

            "Bookcases" => InteriorOrRoomFeatures::Bookcases,

            "Breakfast Bar" => InteriorOrRoomFeatures::BreakfastBar,

            "Built-in Features" => InteriorOrRoomFeatures::BuiltinFeatures,

            "Cathedral Ceiling(s)" => InteriorOrRoomFeatures::CathedralCeilings,

            "Cedar Closet(s)" => InteriorOrRoomFeatures::CedarClosets,

            "Ceiling Fan(s)" => InteriorOrRoomFeatures::CeilingFans,

            "Central Vacuum" => InteriorOrRoomFeatures::CentralVacuum,

            "Chandelier" => InteriorOrRoomFeatures::Chandelier,

            "Coffered Ceiling(s)" => InteriorOrRoomFeatures::CofferedCeilings,

            "Crown Molding" => InteriorOrRoomFeatures::CrownMolding,

            "Double Vanity" => InteriorOrRoomFeatures::DoubleVanity,

            "Dry Bar" => InteriorOrRoomFeatures::DryBar,

            "Dumbwaiter" => InteriorOrRoomFeatures::Dumbwaiter,

            "Eat-in Kitchen" => InteriorOrRoomFeatures::EatinKitchen,

            "Elevator" => InteriorOrRoomFeatures::Elevator,

            "Entrance Foyer" => InteriorOrRoomFeatures::EntranceFoyer,

            "Granite Counters" => InteriorOrRoomFeatures::GraniteCounters,

            "High Ceilings" => InteriorOrRoomFeatures::HighCeilings,

            "High Speed Internet" => InteriorOrRoomFeatures::HighSpeedInternet,

            "His and Hers Closets" => InteriorOrRoomFeatures::HisandHersClosets,

            "In-Law Floorplan" => InteriorOrRoomFeatures::InLawFloorplan,

            "Kitchen Island" => InteriorOrRoomFeatures::KitchenIsland,

            "Laminate Counters" => InteriorOrRoomFeatures::LaminateCounters,

            "Low Flow Plumbing Fixtures" => InteriorOrRoomFeatures::LowFlowPlumbingFixtures,

            "Master Downstairs" => InteriorOrRoomFeatures::MasterDownstairs,

            "Natural Woodwork" => InteriorOrRoomFeatures::NaturalWoodwork,

            "Open Floorplan" => InteriorOrRoomFeatures::OpenFloorplan,

            "Other" => InteriorOrRoomFeatures::Other,

            "Pantry" => InteriorOrRoomFeatures::Pantry,

            "Recessed Lighting" => InteriorOrRoomFeatures::RecessedLighting,

            "Sauna" => InteriorOrRoomFeatures::Sauna,

            "See Remarks" => InteriorOrRoomFeatures::SeeRemarks,

            "Smart Home" => InteriorOrRoomFeatures::SmartHome,

            "Smart Thermostat" => InteriorOrRoomFeatures::SmartThermostat,

            "Soaking Tub" => InteriorOrRoomFeatures::SoakingTub,

            "Solar Tube(s)" => InteriorOrRoomFeatures::SolarTubes,

            "Sound System" => InteriorOrRoomFeatures::SoundSystem,

            "Stone Counters" => InteriorOrRoomFeatures::StoneCounters,

            "Storage" => InteriorOrRoomFeatures::Storage,

            "Tile Counters" => InteriorOrRoomFeatures::TileCounters,

            "Track Lighting" => InteriorOrRoomFeatures::TrackLighting,

            "Tray Ceiling(s)" => InteriorOrRoomFeatures::TrayCeilings,

            "Vaulted Ceiling(s)" => InteriorOrRoomFeatures::VaultedCeilings,

            "Walk-In Closet(s)" => InteriorOrRoomFeatures::WalkInClosets,

            "WaterSense Fixture(s)" => InteriorOrRoomFeatures::WaterSenseFixtures,

            "Wet Bar" => InteriorOrRoomFeatures::WetBar,

            "Wired for Data" => InteriorOrRoomFeatures::WiredforData,

            "Wired for Sound" => InteriorOrRoomFeatures::WiredforSound,

            _ => InteriorOrRoomFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> InteriorOrRoomFeatures {
        match s.as_ref() {
            "Bar" => InteriorOrRoomFeatures::Bar,

            "Beamed Ceilings" => InteriorOrRoomFeatures::BeamedCeilings,

            "Bidet" => InteriorOrRoomFeatures::Bidet,

            "Bookcases" => InteriorOrRoomFeatures::Bookcases,

            "Breakfast Bar" => InteriorOrRoomFeatures::BreakfastBar,

            "Built-in Features" => InteriorOrRoomFeatures::BuiltinFeatures,

            "Cathedral Ceiling(s)" => InteriorOrRoomFeatures::CathedralCeilings,

            "Cedar Closet(s)" => InteriorOrRoomFeatures::CedarClosets,

            "Ceiling Fan(s)" => InteriorOrRoomFeatures::CeilingFans,

            "Central Vacuum" => InteriorOrRoomFeatures::CentralVacuum,

            "Chandelier" => InteriorOrRoomFeatures::Chandelier,

            "Coffered Ceiling(s)" => InteriorOrRoomFeatures::CofferedCeilings,

            "Crown Molding" => InteriorOrRoomFeatures::CrownMolding,

            "Double Vanity" => InteriorOrRoomFeatures::DoubleVanity,

            "Dry Bar" => InteriorOrRoomFeatures::DryBar,

            "Dumbwaiter" => InteriorOrRoomFeatures::Dumbwaiter,

            "Eat-in Kitchen" => InteriorOrRoomFeatures::EatinKitchen,

            "Elevator" => InteriorOrRoomFeatures::Elevator,

            "Entrance Foyer" => InteriorOrRoomFeatures::EntranceFoyer,

            "Granite Counters" => InteriorOrRoomFeatures::GraniteCounters,

            "High Ceilings" => InteriorOrRoomFeatures::HighCeilings,

            "High Speed Internet" => InteriorOrRoomFeatures::HighSpeedInternet,

            "His and Hers Closets" => InteriorOrRoomFeatures::HisandHersClosets,

            "In-Law Floorplan" => InteriorOrRoomFeatures::InLawFloorplan,

            "Kitchen Island" => InteriorOrRoomFeatures::KitchenIsland,

            "Laminate Counters" => InteriorOrRoomFeatures::LaminateCounters,

            "Low Flow Plumbing Fixtures" => InteriorOrRoomFeatures::LowFlowPlumbingFixtures,

            "Master Downstairs" => InteriorOrRoomFeatures::MasterDownstairs,

            "Natural Woodwork" => InteriorOrRoomFeatures::NaturalWoodwork,

            "Open Floorplan" => InteriorOrRoomFeatures::OpenFloorplan,

            "Other" => InteriorOrRoomFeatures::Other,

            "Pantry" => InteriorOrRoomFeatures::Pantry,

            "Recessed Lighting" => InteriorOrRoomFeatures::RecessedLighting,

            "Sauna" => InteriorOrRoomFeatures::Sauna,

            "See Remarks" => InteriorOrRoomFeatures::SeeRemarks,

            "Smart Home" => InteriorOrRoomFeatures::SmartHome,

            "Smart Thermostat" => InteriorOrRoomFeatures::SmartThermostat,

            "Soaking Tub" => InteriorOrRoomFeatures::SoakingTub,

            "Solar Tube(s)" => InteriorOrRoomFeatures::SolarTubes,

            "Sound System" => InteriorOrRoomFeatures::SoundSystem,

            "Stone Counters" => InteriorOrRoomFeatures::StoneCounters,

            "Storage" => InteriorOrRoomFeatures::Storage,

            "Tile Counters" => InteriorOrRoomFeatures::TileCounters,

            "Track Lighting" => InteriorOrRoomFeatures::TrackLighting,

            "Tray Ceiling(s)" => InteriorOrRoomFeatures::TrayCeilings,

            "Vaulted Ceiling(s)" => InteriorOrRoomFeatures::VaultedCeilings,

            "Walk-In Closet(s)" => InteriorOrRoomFeatures::WalkInClosets,

            "WaterSense Fixture(s)" => InteriorOrRoomFeatures::WaterSenseFixtures,

            "Wet Bar" => InteriorOrRoomFeatures::WetBar,

            "Wired for Data" => InteriorOrRoomFeatures::WiredforData,

            "Wired for Sound" => InteriorOrRoomFeatures::WiredforSound,

            _ => InteriorOrRoomFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            InteriorOrRoomFeatures::Bar => "Bar",

            InteriorOrRoomFeatures::BeamedCeilings => "Beamed Ceilings",

            InteriorOrRoomFeatures::Bidet => "Bidet",

            InteriorOrRoomFeatures::Bookcases => "Bookcases",

            InteriorOrRoomFeatures::BreakfastBar => "Breakfast Bar",

            InteriorOrRoomFeatures::BuiltinFeatures => "Built-in Features",

            InteriorOrRoomFeatures::CathedralCeilings => "Cathedral Ceiling(s)",

            InteriorOrRoomFeatures::CedarClosets => "Cedar Closet(s)",

            InteriorOrRoomFeatures::CeilingFans => "Ceiling Fan(s)",

            InteriorOrRoomFeatures::CentralVacuum => "Central Vacuum",

            InteriorOrRoomFeatures::Chandelier => "Chandelier",

            InteriorOrRoomFeatures::CofferedCeilings => "Coffered Ceiling(s)",

            InteriorOrRoomFeatures::CrownMolding => "Crown Molding",

            InteriorOrRoomFeatures::DoubleVanity => "Double Vanity",

            InteriorOrRoomFeatures::DryBar => "Dry Bar",

            InteriorOrRoomFeatures::Dumbwaiter => "Dumbwaiter",

            InteriorOrRoomFeatures::EatinKitchen => "Eat-in Kitchen",

            InteriorOrRoomFeatures::Elevator => "Elevator",

            InteriorOrRoomFeatures::EntranceFoyer => "Entrance Foyer",

            InteriorOrRoomFeatures::GraniteCounters => "Granite Counters",

            InteriorOrRoomFeatures::HighCeilings => "High Ceilings",

            InteriorOrRoomFeatures::HighSpeedInternet => "High Speed Internet",

            InteriorOrRoomFeatures::HisandHersClosets => "His and Hers Closets",

            InteriorOrRoomFeatures::InLawFloorplan => "In-Law Floorplan",

            InteriorOrRoomFeatures::KitchenIsland => "Kitchen Island",

            InteriorOrRoomFeatures::LaminateCounters => "Laminate Counters",

            InteriorOrRoomFeatures::LowFlowPlumbingFixtures => "Low Flow Plumbing Fixtures",

            InteriorOrRoomFeatures::MasterDownstairs => "Master Downstairs",

            InteriorOrRoomFeatures::NaturalWoodwork => "Natural Woodwork",

            InteriorOrRoomFeatures::OpenFloorplan => "Open Floorplan",

            InteriorOrRoomFeatures::Other => "Other",

            InteriorOrRoomFeatures::Pantry => "Pantry",

            InteriorOrRoomFeatures::RecessedLighting => "Recessed Lighting",

            InteriorOrRoomFeatures::Sauna => "Sauna",

            InteriorOrRoomFeatures::SeeRemarks => "See Remarks",

            InteriorOrRoomFeatures::SmartHome => "Smart Home",

            InteriorOrRoomFeatures::SmartThermostat => "Smart Thermostat",

            InteriorOrRoomFeatures::SoakingTub => "Soaking Tub",

            InteriorOrRoomFeatures::SolarTubes => "Solar Tube(s)",

            InteriorOrRoomFeatures::SoundSystem => "Sound System",

            InteriorOrRoomFeatures::StoneCounters => "Stone Counters",

            InteriorOrRoomFeatures::Storage => "Storage",

            InteriorOrRoomFeatures::TileCounters => "Tile Counters",

            InteriorOrRoomFeatures::TrackLighting => "Track Lighting",

            InteriorOrRoomFeatures::TrayCeilings => "Tray Ceiling(s)",

            InteriorOrRoomFeatures::VaultedCeilings => "Vaulted Ceiling(s)",

            InteriorOrRoomFeatures::WalkInClosets => "Walk-In Closet(s)",

            InteriorOrRoomFeatures::WaterSenseFixtures => "WaterSense Fixture(s)",

            InteriorOrRoomFeatures::WetBar => "Wet Bar",

            InteriorOrRoomFeatures::WiredforData => "Wired for Data",

            InteriorOrRoomFeatures::WiredforSound => "Wired for Sound",

            InteriorOrRoomFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            InteriorOrRoomFeatures::Bar => "Bar".into(),

            InteriorOrRoomFeatures::BeamedCeilings => "Beamed Ceilings".into(),

            InteriorOrRoomFeatures::Bidet => "Bidet".into(),

            InteriorOrRoomFeatures::Bookcases => "Bookcases".into(),

            InteriorOrRoomFeatures::BreakfastBar => "Breakfast Bar".into(),

            InteriorOrRoomFeatures::BuiltinFeatures => "Built-in Features".into(),

            InteriorOrRoomFeatures::CathedralCeilings => "Cathedral Ceiling(s)".into(),

            InteriorOrRoomFeatures::CedarClosets => "Cedar Closet(s)".into(),

            InteriorOrRoomFeatures::CeilingFans => "Ceiling Fan(s)".into(),

            InteriorOrRoomFeatures::CentralVacuum => "Central Vacuum".into(),

            InteriorOrRoomFeatures::Chandelier => "Chandelier".into(),

            InteriorOrRoomFeatures::CofferedCeilings => "Coffered Ceiling(s)".into(),

            InteriorOrRoomFeatures::CrownMolding => "Crown Molding".into(),

            InteriorOrRoomFeatures::DoubleVanity => "Double Vanity".into(),

            InteriorOrRoomFeatures::DryBar => "Dry Bar".into(),

            InteriorOrRoomFeatures::Dumbwaiter => "Dumbwaiter".into(),

            InteriorOrRoomFeatures::EatinKitchen => "Eat-in Kitchen".into(),

            InteriorOrRoomFeatures::Elevator => "Elevator".into(),

            InteriorOrRoomFeatures::EntranceFoyer => "Entrance Foyer".into(),

            InteriorOrRoomFeatures::GraniteCounters => "Granite Counters".into(),

            InteriorOrRoomFeatures::HighCeilings => "High Ceilings".into(),

            InteriorOrRoomFeatures::HighSpeedInternet => "High Speed Internet".into(),

            InteriorOrRoomFeatures::HisandHersClosets => "His and Hers Closets".into(),

            InteriorOrRoomFeatures::InLawFloorplan => "In-Law Floorplan".into(),

            InteriorOrRoomFeatures::KitchenIsland => "Kitchen Island".into(),

            InteriorOrRoomFeatures::LaminateCounters => "Laminate Counters".into(),

            InteriorOrRoomFeatures::LowFlowPlumbingFixtures => "Low Flow Plumbing Fixtures".into(),

            InteriorOrRoomFeatures::MasterDownstairs => "Master Downstairs".into(),

            InteriorOrRoomFeatures::NaturalWoodwork => "Natural Woodwork".into(),

            InteriorOrRoomFeatures::OpenFloorplan => "Open Floorplan".into(),

            InteriorOrRoomFeatures::Other => "Other".into(),

            InteriorOrRoomFeatures::Pantry => "Pantry".into(),

            InteriorOrRoomFeatures::RecessedLighting => "Recessed Lighting".into(),

            InteriorOrRoomFeatures::Sauna => "Sauna".into(),

            InteriorOrRoomFeatures::SeeRemarks => "See Remarks".into(),

            InteriorOrRoomFeatures::SmartHome => "Smart Home".into(),

            InteriorOrRoomFeatures::SmartThermostat => "Smart Thermostat".into(),

            InteriorOrRoomFeatures::SoakingTub => "Soaking Tub".into(),

            InteriorOrRoomFeatures::SolarTubes => "Solar Tube(s)".into(),

            InteriorOrRoomFeatures::SoundSystem => "Sound System".into(),

            InteriorOrRoomFeatures::StoneCounters => "Stone Counters".into(),

            InteriorOrRoomFeatures::Storage => "Storage".into(),

            InteriorOrRoomFeatures::TileCounters => "Tile Counters".into(),

            InteriorOrRoomFeatures::TrackLighting => "Track Lighting".into(),

            InteriorOrRoomFeatures::TrayCeilings => "Tray Ceiling(s)".into(),

            InteriorOrRoomFeatures::VaultedCeilings => "Vaulted Ceiling(s)".into(),

            InteriorOrRoomFeatures::WalkInClosets => "Walk-In Closet(s)".into(),

            InteriorOrRoomFeatures::WaterSenseFixtures => "WaterSense Fixture(s)".into(),

            InteriorOrRoomFeatures::WetBar => "Wet Bar".into(),

            InteriorOrRoomFeatures::WiredforData => "Wired for Data".into(),

            InteriorOrRoomFeatures::WiredforSound => "Wired for Sound".into(),

            InteriorOrRoomFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            InteriorOrRoomFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for InteriorOrRoomFeatures {
    fn from(s: String) -> InteriorOrRoomFeatures {
        match s.as_ref() {
            "Bar" => InteriorOrRoomFeatures::Bar,

            "Beamed Ceilings" => InteriorOrRoomFeatures::BeamedCeilings,

            "Bidet" => InteriorOrRoomFeatures::Bidet,

            "Bookcases" => InteriorOrRoomFeatures::Bookcases,

            "Breakfast Bar" => InteriorOrRoomFeatures::BreakfastBar,

            "Built-in Features" => InteriorOrRoomFeatures::BuiltinFeatures,

            "Cathedral Ceiling(s)" => InteriorOrRoomFeatures::CathedralCeilings,

            "Cedar Closet(s)" => InteriorOrRoomFeatures::CedarClosets,

            "Ceiling Fan(s)" => InteriorOrRoomFeatures::CeilingFans,

            "Central Vacuum" => InteriorOrRoomFeatures::CentralVacuum,

            "Chandelier" => InteriorOrRoomFeatures::Chandelier,

            "Coffered Ceiling(s)" => InteriorOrRoomFeatures::CofferedCeilings,

            "Crown Molding" => InteriorOrRoomFeatures::CrownMolding,

            "Double Vanity" => InteriorOrRoomFeatures::DoubleVanity,

            "Dry Bar" => InteriorOrRoomFeatures::DryBar,

            "Dumbwaiter" => InteriorOrRoomFeatures::Dumbwaiter,

            "Eat-in Kitchen" => InteriorOrRoomFeatures::EatinKitchen,

            "Elevator" => InteriorOrRoomFeatures::Elevator,

            "Entrance Foyer" => InteriorOrRoomFeatures::EntranceFoyer,

            "Granite Counters" => InteriorOrRoomFeatures::GraniteCounters,

            "High Ceilings" => InteriorOrRoomFeatures::HighCeilings,

            "High Speed Internet" => InteriorOrRoomFeatures::HighSpeedInternet,

            "His and Hers Closets" => InteriorOrRoomFeatures::HisandHersClosets,

            "In-Law Floorplan" => InteriorOrRoomFeatures::InLawFloorplan,

            "Kitchen Island" => InteriorOrRoomFeatures::KitchenIsland,

            "Laminate Counters" => InteriorOrRoomFeatures::LaminateCounters,

            "Low Flow Plumbing Fixtures" => InteriorOrRoomFeatures::LowFlowPlumbingFixtures,

            "Master Downstairs" => InteriorOrRoomFeatures::MasterDownstairs,

            "Natural Woodwork" => InteriorOrRoomFeatures::NaturalWoodwork,

            "Open Floorplan" => InteriorOrRoomFeatures::OpenFloorplan,

            "Other" => InteriorOrRoomFeatures::Other,

            "Pantry" => InteriorOrRoomFeatures::Pantry,

            "Recessed Lighting" => InteriorOrRoomFeatures::RecessedLighting,

            "Sauna" => InteriorOrRoomFeatures::Sauna,

            "See Remarks" => InteriorOrRoomFeatures::SeeRemarks,

            "Smart Home" => InteriorOrRoomFeatures::SmartHome,

            "Smart Thermostat" => InteriorOrRoomFeatures::SmartThermostat,

            "Soaking Tub" => InteriorOrRoomFeatures::SoakingTub,

            "Solar Tube(s)" => InteriorOrRoomFeatures::SolarTubes,

            "Sound System" => InteriorOrRoomFeatures::SoundSystem,

            "Stone Counters" => InteriorOrRoomFeatures::StoneCounters,

            "Storage" => InteriorOrRoomFeatures::Storage,

            "Tile Counters" => InteriorOrRoomFeatures::TileCounters,

            "Track Lighting" => InteriorOrRoomFeatures::TrackLighting,

            "Tray Ceiling(s)" => InteriorOrRoomFeatures::TrayCeilings,

            "Vaulted Ceiling(s)" => InteriorOrRoomFeatures::VaultedCeilings,

            "Walk-In Closet(s)" => InteriorOrRoomFeatures::WalkInClosets,

            "WaterSense Fixture(s)" => InteriorOrRoomFeatures::WaterSenseFixtures,

            "Wet Bar" => InteriorOrRoomFeatures::WetBar,

            "Wired for Data" => InteriorOrRoomFeatures::WiredforData,

            "Wired for Sound" => InteriorOrRoomFeatures::WiredforSound,

            _ => InteriorOrRoomFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for InteriorOrRoomFeatures {
    fn from(s: &str) -> InteriorOrRoomFeatures {
        match s {
            "Bar" => InteriorOrRoomFeatures::Bar,

            "Beamed Ceilings" => InteriorOrRoomFeatures::BeamedCeilings,

            "Bidet" => InteriorOrRoomFeatures::Bidet,

            "Bookcases" => InteriorOrRoomFeatures::Bookcases,

            "Breakfast Bar" => InteriorOrRoomFeatures::BreakfastBar,

            "Built-in Features" => InteriorOrRoomFeatures::BuiltinFeatures,

            "Cathedral Ceiling(s)" => InteriorOrRoomFeatures::CathedralCeilings,

            "Cedar Closet(s)" => InteriorOrRoomFeatures::CedarClosets,

            "Ceiling Fan(s)" => InteriorOrRoomFeatures::CeilingFans,

            "Central Vacuum" => InteriorOrRoomFeatures::CentralVacuum,

            "Chandelier" => InteriorOrRoomFeatures::Chandelier,

            "Coffered Ceiling(s)" => InteriorOrRoomFeatures::CofferedCeilings,

            "Crown Molding" => InteriorOrRoomFeatures::CrownMolding,

            "Double Vanity" => InteriorOrRoomFeatures::DoubleVanity,

            "Dry Bar" => InteriorOrRoomFeatures::DryBar,

            "Dumbwaiter" => InteriorOrRoomFeatures::Dumbwaiter,

            "Eat-in Kitchen" => InteriorOrRoomFeatures::EatinKitchen,

            "Elevator" => InteriorOrRoomFeatures::Elevator,

            "Entrance Foyer" => InteriorOrRoomFeatures::EntranceFoyer,

            "Granite Counters" => InteriorOrRoomFeatures::GraniteCounters,

            "High Ceilings" => InteriorOrRoomFeatures::HighCeilings,

            "High Speed Internet" => InteriorOrRoomFeatures::HighSpeedInternet,

            "His and Hers Closets" => InteriorOrRoomFeatures::HisandHersClosets,

            "In-Law Floorplan" => InteriorOrRoomFeatures::InLawFloorplan,

            "Kitchen Island" => InteriorOrRoomFeatures::KitchenIsland,

            "Laminate Counters" => InteriorOrRoomFeatures::LaminateCounters,

            "Low Flow Plumbing Fixtures" => InteriorOrRoomFeatures::LowFlowPlumbingFixtures,

            "Master Downstairs" => InteriorOrRoomFeatures::MasterDownstairs,

            "Natural Woodwork" => InteriorOrRoomFeatures::NaturalWoodwork,

            "Open Floorplan" => InteriorOrRoomFeatures::OpenFloorplan,

            "Other" => InteriorOrRoomFeatures::Other,

            "Pantry" => InteriorOrRoomFeatures::Pantry,

            "Recessed Lighting" => InteriorOrRoomFeatures::RecessedLighting,

            "Sauna" => InteriorOrRoomFeatures::Sauna,

            "See Remarks" => InteriorOrRoomFeatures::SeeRemarks,

            "Smart Home" => InteriorOrRoomFeatures::SmartHome,

            "Smart Thermostat" => InteriorOrRoomFeatures::SmartThermostat,

            "Soaking Tub" => InteriorOrRoomFeatures::SoakingTub,

            "Solar Tube(s)" => InteriorOrRoomFeatures::SolarTubes,

            "Sound System" => InteriorOrRoomFeatures::SoundSystem,

            "Stone Counters" => InteriorOrRoomFeatures::StoneCounters,

            "Storage" => InteriorOrRoomFeatures::Storage,

            "Tile Counters" => InteriorOrRoomFeatures::TileCounters,

            "Track Lighting" => InteriorOrRoomFeatures::TrackLighting,

            "Tray Ceiling(s)" => InteriorOrRoomFeatures::TrayCeilings,

            "Vaulted Ceiling(s)" => InteriorOrRoomFeatures::VaultedCeilings,

            "Walk-In Closet(s)" => InteriorOrRoomFeatures::WalkInClosets,

            "WaterSense Fixture(s)" => InteriorOrRoomFeatures::WaterSenseFixtures,

            "Wet Bar" => InteriorOrRoomFeatures::WetBar,

            "Wired for Data" => InteriorOrRoomFeatures::WiredforData,

            "Wired for Sound" => InteriorOrRoomFeatures::WiredforSound,

            _ => InteriorOrRoomFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a InteriorOrRoomFeatures> for &'a str {
    fn from(s: &'a InteriorOrRoomFeatures) -> &'a str {
        match s {
            InteriorOrRoomFeatures::Bar => "Bar",

            InteriorOrRoomFeatures::BeamedCeilings => "Beamed Ceilings",

            InteriorOrRoomFeatures::Bidet => "Bidet",

            InteriorOrRoomFeatures::Bookcases => "Bookcases",

            InteriorOrRoomFeatures::BreakfastBar => "Breakfast Bar",

            InteriorOrRoomFeatures::BuiltinFeatures => "Built-in Features",

            InteriorOrRoomFeatures::CathedralCeilings => "Cathedral Ceiling(s)",

            InteriorOrRoomFeatures::CedarClosets => "Cedar Closet(s)",

            InteriorOrRoomFeatures::CeilingFans => "Ceiling Fan(s)",

            InteriorOrRoomFeatures::CentralVacuum => "Central Vacuum",

            InteriorOrRoomFeatures::Chandelier => "Chandelier",

            InteriorOrRoomFeatures::CofferedCeilings => "Coffered Ceiling(s)",

            InteriorOrRoomFeatures::CrownMolding => "Crown Molding",

            InteriorOrRoomFeatures::DoubleVanity => "Double Vanity",

            InteriorOrRoomFeatures::DryBar => "Dry Bar",

            InteriorOrRoomFeatures::Dumbwaiter => "Dumbwaiter",

            InteriorOrRoomFeatures::EatinKitchen => "Eat-in Kitchen",

            InteriorOrRoomFeatures::Elevator => "Elevator",

            InteriorOrRoomFeatures::EntranceFoyer => "Entrance Foyer",

            InteriorOrRoomFeatures::GraniteCounters => "Granite Counters",

            InteriorOrRoomFeatures::HighCeilings => "High Ceilings",

            InteriorOrRoomFeatures::HighSpeedInternet => "High Speed Internet",

            InteriorOrRoomFeatures::HisandHersClosets => "His and Hers Closets",

            InteriorOrRoomFeatures::InLawFloorplan => "In-Law Floorplan",

            InteriorOrRoomFeatures::KitchenIsland => "Kitchen Island",

            InteriorOrRoomFeatures::LaminateCounters => "Laminate Counters",

            InteriorOrRoomFeatures::LowFlowPlumbingFixtures => "Low Flow Plumbing Fixtures",

            InteriorOrRoomFeatures::MasterDownstairs => "Master Downstairs",

            InteriorOrRoomFeatures::NaturalWoodwork => "Natural Woodwork",

            InteriorOrRoomFeatures::OpenFloorplan => "Open Floorplan",

            InteriorOrRoomFeatures::Other => "Other",

            InteriorOrRoomFeatures::Pantry => "Pantry",

            InteriorOrRoomFeatures::RecessedLighting => "Recessed Lighting",

            InteriorOrRoomFeatures::Sauna => "Sauna",

            InteriorOrRoomFeatures::SeeRemarks => "See Remarks",

            InteriorOrRoomFeatures::SmartHome => "Smart Home",

            InteriorOrRoomFeatures::SmartThermostat => "Smart Thermostat",

            InteriorOrRoomFeatures::SoakingTub => "Soaking Tub",

            InteriorOrRoomFeatures::SolarTubes => "Solar Tube(s)",

            InteriorOrRoomFeatures::SoundSystem => "Sound System",

            InteriorOrRoomFeatures::StoneCounters => "Stone Counters",

            InteriorOrRoomFeatures::Storage => "Storage",

            InteriorOrRoomFeatures::TileCounters => "Tile Counters",

            InteriorOrRoomFeatures::TrackLighting => "Track Lighting",

            InteriorOrRoomFeatures::TrayCeilings => "Tray Ceiling(s)",

            InteriorOrRoomFeatures::VaultedCeilings => "Vaulted Ceiling(s)",

            InteriorOrRoomFeatures::WalkInClosets => "Walk-In Closet(s)",

            InteriorOrRoomFeatures::WaterSenseFixtures => "WaterSense Fixture(s)",

            InteriorOrRoomFeatures::WetBar => "Wet Bar",

            InteriorOrRoomFeatures::WiredforData => "Wired for Data",

            InteriorOrRoomFeatures::WiredforSound => "Wired for Sound",

            InteriorOrRoomFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for InteriorOrRoomFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for InteriorOrRoomFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
