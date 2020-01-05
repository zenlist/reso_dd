// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [OtherStructures Lookups](https://ddwiki.reso.org/display/DDW17/OtherStructures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OtherStructures {
    /// "[Airplane Hangar](https://ddwiki.reso.org/display/DDW17/Airplane+Hangar)": The property includes an airplane hangar.
    AirplaneHangar,

    /// "[Arena](https://ddwiki.reso.org/display/DDW17/Arena)": The property includes an arena.
    Arena,

    /// "[Barn(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245763)": The property includes a barn(s).
    Barns,

    /// "[Boat House](https://ddwiki.reso.org/display/DDW17/Boat+House)": The property includes a boat house.
    BoatHouse,

    /// "[Cabana](https://ddwiki.reso.org/display/DDW17/Cabana)": The property includes a cabana.
    Cabana,

    /// "[Cave(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245766)": The property includes a cave(s).
    Caves,

    /// "[Corral(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245767)": The property includes a corral(s).
    Corrals,

    /// "[Covered Arena](https://ddwiki.reso.org/display/DDW17/Covered+Arena)": The property includes a covered arena.
    CoveredArena,

    /// "[Garage(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245769)": The property includes a garage(s).
    Garages,

    /// "[Gazebo](https://ddwiki.reso.org/display/DDW17/Gazebo)": The property includes a gazebo.
    Gazebo,

    /// "[Grain Storage](https://ddwiki.reso.org/display/DDW17/Grain+Storage)": The property includes grain storage.
    GrainStorage,

    /// "[Greenhouse](https://ddwiki.reso.org/display/DDW17/Greenhouse)": The property includes a greenhouse.
    Greenhouse,

    /// "[Guest House](https://ddwiki.reso.org/display/DDW17/Guest+House)": The property includes a guest house.
    GuestHouse,

    /// "[Kennel/Dog Run](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245774)": The property includes a kennel or dog run.
    KennelDogRun,

    /// "[Mobile Home](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245775)": The property includes a mobile home.
    MobileHome,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245776)": The property has no other structures.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245777)": The property includes a structure other than those included in this list.
    Other,

    /// "[Outbuilding](https://ddwiki.reso.org/display/DDW17/Outbuilding)": The property includes an outbuilding.
    Outbuilding,

    /// "[Outdoor Kitchen](https://ddwiki.reso.org/display/DDW17/Outdoor+Kitchen)": The property includes an outdoor kitchen.
    OutdoorKitchen,

    /// "[Packing Shed](https://ddwiki.reso.org/display/DDW17/Packing+Shed)": The property includes a packing shed.
    PackingShed,

    /// "[Pergola](https://ddwiki.reso.org/display/DDW17/Pergola)": The property includes a pergola.
    Pergola,

    /// "[Pool House](https://ddwiki.reso.org/display/DDW17/Pool+House)": The property includes a pool house.
    PoolHouse,

    /// "[Poultry Coop](https://ddwiki.reso.org/display/DDW17/Poultry+Coop)": The property includes a poultry coop.
    PoultryCoop,

    /// "[Residence](https://ddwiki.reso.org/display/DDW17/Residence)": The property includes a residence structure.
    Residence,

    /// "[RV/Boat Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245785)": The property includes RV or boat storage.
    RVBoatStorage,

    /// "[Second Garage](https://ddwiki.reso.org/display/DDW17/Second+Garage)": The property includes a second garage.
    SecondGarage,

    /// "[Second Residence](https://ddwiki.reso.org/display/DDW17/Second+Residence)": The property includes a second residence.
    SecondResidence,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245788)": See the Public or Private Remarks for information about other structures on the property.
    SeeRemarks,

    /// "[Shed(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245789)": The property includes a shed(s).
    Sheds,

    /// "[Stable(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245790)": The property includes stable(s).
    Stables,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245791)": The property includes storage.
    Storage,

    /// "[Tennis Court(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245792)": The property includes a tennis court(s).
    TennisCourts,

    /// "[Workshop](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245793)": The property includes a workshop.
    Workshop,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for OtherStructures {
    fn from_str(s: &str) -> OtherStructures {
        match s {
            "Airplane Hangar" => OtherStructures::AirplaneHangar,

            "Arena" => OtherStructures::Arena,

            "Barn(s)" => OtherStructures::Barns,

            "Boat House" => OtherStructures::BoatHouse,

            "Cabana" => OtherStructures::Cabana,

            "Cave(s)" => OtherStructures::Caves,

            "Corral(s)" => OtherStructures::Corrals,

            "Covered Arena" => OtherStructures::CoveredArena,

            "Garage(s)" => OtherStructures::Garages,

            "Gazebo" => OtherStructures::Gazebo,

            "Grain Storage" => OtherStructures::GrainStorage,

            "Greenhouse" => OtherStructures::Greenhouse,

            "Guest House" => OtherStructures::GuestHouse,

            "Kennel/Dog Run" => OtherStructures::KennelDogRun,

            "Mobile Home" => OtherStructures::MobileHome,

            "None" => OtherStructures::None,

            "Other" => OtherStructures::Other,

            "Outbuilding" => OtherStructures::Outbuilding,

            "Outdoor Kitchen" => OtherStructures::OutdoorKitchen,

            "Packing Shed" => OtherStructures::PackingShed,

            "Pergola" => OtherStructures::Pergola,

            "Pool House" => OtherStructures::PoolHouse,

            "Poultry Coop" => OtherStructures::PoultryCoop,

            "Residence" => OtherStructures::Residence,

            "RV/Boat Storage" => OtherStructures::RVBoatStorage,

            "Second Garage" => OtherStructures::SecondGarage,

            "Second Residence" => OtherStructures::SecondResidence,

            "See Remarks" => OtherStructures::SeeRemarks,

            "Shed(s)" => OtherStructures::Sheds,

            "Stable(s)" => OtherStructures::Stables,

            "Storage" => OtherStructures::Storage,

            "Tennis Court(s)" => OtherStructures::TennisCourts,

            "Workshop" => OtherStructures::Workshop,

            _ => OtherStructures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> OtherStructures {
        match s.as_ref() {
            "Airplane Hangar" => OtherStructures::AirplaneHangar,

            "Arena" => OtherStructures::Arena,

            "Barn(s)" => OtherStructures::Barns,

            "Boat House" => OtherStructures::BoatHouse,

            "Cabana" => OtherStructures::Cabana,

            "Cave(s)" => OtherStructures::Caves,

            "Corral(s)" => OtherStructures::Corrals,

            "Covered Arena" => OtherStructures::CoveredArena,

            "Garage(s)" => OtherStructures::Garages,

            "Gazebo" => OtherStructures::Gazebo,

            "Grain Storage" => OtherStructures::GrainStorage,

            "Greenhouse" => OtherStructures::Greenhouse,

            "Guest House" => OtherStructures::GuestHouse,

            "Kennel/Dog Run" => OtherStructures::KennelDogRun,

            "Mobile Home" => OtherStructures::MobileHome,

            "None" => OtherStructures::None,

            "Other" => OtherStructures::Other,

            "Outbuilding" => OtherStructures::Outbuilding,

            "Outdoor Kitchen" => OtherStructures::OutdoorKitchen,

            "Packing Shed" => OtherStructures::PackingShed,

            "Pergola" => OtherStructures::Pergola,

            "Pool House" => OtherStructures::PoolHouse,

            "Poultry Coop" => OtherStructures::PoultryCoop,

            "Residence" => OtherStructures::Residence,

            "RV/Boat Storage" => OtherStructures::RVBoatStorage,

            "Second Garage" => OtherStructures::SecondGarage,

            "Second Residence" => OtherStructures::SecondResidence,

            "See Remarks" => OtherStructures::SeeRemarks,

            "Shed(s)" => OtherStructures::Sheds,

            "Stable(s)" => OtherStructures::Stables,

            "Storage" => OtherStructures::Storage,

            "Tennis Court(s)" => OtherStructures::TennisCourts,

            "Workshop" => OtherStructures::Workshop,

            _ => OtherStructures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            OtherStructures::AirplaneHangar => "Airplane Hangar",

            OtherStructures::Arena => "Arena",

            OtherStructures::Barns => "Barn(s)",

            OtherStructures::BoatHouse => "Boat House",

            OtherStructures::Cabana => "Cabana",

            OtherStructures::Caves => "Cave(s)",

            OtherStructures::Corrals => "Corral(s)",

            OtherStructures::CoveredArena => "Covered Arena",

            OtherStructures::Garages => "Garage(s)",

            OtherStructures::Gazebo => "Gazebo",

            OtherStructures::GrainStorage => "Grain Storage",

            OtherStructures::Greenhouse => "Greenhouse",

            OtherStructures::GuestHouse => "Guest House",

            OtherStructures::KennelDogRun => "Kennel/Dog Run",

            OtherStructures::MobileHome => "Mobile Home",

            OtherStructures::None => "None",

            OtherStructures::Other => "Other",

            OtherStructures::Outbuilding => "Outbuilding",

            OtherStructures::OutdoorKitchen => "Outdoor Kitchen",

            OtherStructures::PackingShed => "Packing Shed",

            OtherStructures::Pergola => "Pergola",

            OtherStructures::PoolHouse => "Pool House",

            OtherStructures::PoultryCoop => "Poultry Coop",

            OtherStructures::Residence => "Residence",

            OtherStructures::RVBoatStorage => "RV/Boat Storage",

            OtherStructures::SecondGarage => "Second Garage",

            OtherStructures::SecondResidence => "Second Residence",

            OtherStructures::SeeRemarks => "See Remarks",

            OtherStructures::Sheds => "Shed(s)",

            OtherStructures::Stables => "Stable(s)",

            OtherStructures::Storage => "Storage",

            OtherStructures::TennisCourts => "Tennis Court(s)",

            OtherStructures::Workshop => "Workshop",

            OtherStructures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            OtherStructures::AirplaneHangar => "Airplane Hangar".into(),

            OtherStructures::Arena => "Arena".into(),

            OtherStructures::Barns => "Barn(s)".into(),

            OtherStructures::BoatHouse => "Boat House".into(),

            OtherStructures::Cabana => "Cabana".into(),

            OtherStructures::Caves => "Cave(s)".into(),

            OtherStructures::Corrals => "Corral(s)".into(),

            OtherStructures::CoveredArena => "Covered Arena".into(),

            OtherStructures::Garages => "Garage(s)".into(),

            OtherStructures::Gazebo => "Gazebo".into(),

            OtherStructures::GrainStorage => "Grain Storage".into(),

            OtherStructures::Greenhouse => "Greenhouse".into(),

            OtherStructures::GuestHouse => "Guest House".into(),

            OtherStructures::KennelDogRun => "Kennel/Dog Run".into(),

            OtherStructures::MobileHome => "Mobile Home".into(),

            OtherStructures::None => "None".into(),

            OtherStructures::Other => "Other".into(),

            OtherStructures::Outbuilding => "Outbuilding".into(),

            OtherStructures::OutdoorKitchen => "Outdoor Kitchen".into(),

            OtherStructures::PackingShed => "Packing Shed".into(),

            OtherStructures::Pergola => "Pergola".into(),

            OtherStructures::PoolHouse => "Pool House".into(),

            OtherStructures::PoultryCoop => "Poultry Coop".into(),

            OtherStructures::Residence => "Residence".into(),

            OtherStructures::RVBoatStorage => "RV/Boat Storage".into(),

            OtherStructures::SecondGarage => "Second Garage".into(),

            OtherStructures::SecondResidence => "Second Residence".into(),

            OtherStructures::SeeRemarks => "See Remarks".into(),

            OtherStructures::Sheds => "Shed(s)".into(),

            OtherStructures::Stables => "Stable(s)".into(),

            OtherStructures::Storage => "Storage".into(),

            OtherStructures::TennisCourts => "Tennis Court(s)".into(),

            OtherStructures::Workshop => "Workshop".into(),

            OtherStructures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            OtherStructures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for OtherStructures {
    fn from(s: String) -> OtherStructures {
        match s.as_ref() {
            "Airplane Hangar" => OtherStructures::AirplaneHangar,

            "Arena" => OtherStructures::Arena,

            "Barn(s)" => OtherStructures::Barns,

            "Boat House" => OtherStructures::BoatHouse,

            "Cabana" => OtherStructures::Cabana,

            "Cave(s)" => OtherStructures::Caves,

            "Corral(s)" => OtherStructures::Corrals,

            "Covered Arena" => OtherStructures::CoveredArena,

            "Garage(s)" => OtherStructures::Garages,

            "Gazebo" => OtherStructures::Gazebo,

            "Grain Storage" => OtherStructures::GrainStorage,

            "Greenhouse" => OtherStructures::Greenhouse,

            "Guest House" => OtherStructures::GuestHouse,

            "Kennel/Dog Run" => OtherStructures::KennelDogRun,

            "Mobile Home" => OtherStructures::MobileHome,

            "None" => OtherStructures::None,

            "Other" => OtherStructures::Other,

            "Outbuilding" => OtherStructures::Outbuilding,

            "Outdoor Kitchen" => OtherStructures::OutdoorKitchen,

            "Packing Shed" => OtherStructures::PackingShed,

            "Pergola" => OtherStructures::Pergola,

            "Pool House" => OtherStructures::PoolHouse,

            "Poultry Coop" => OtherStructures::PoultryCoop,

            "Residence" => OtherStructures::Residence,

            "RV/Boat Storage" => OtherStructures::RVBoatStorage,

            "Second Garage" => OtherStructures::SecondGarage,

            "Second Residence" => OtherStructures::SecondResidence,

            "See Remarks" => OtherStructures::SeeRemarks,

            "Shed(s)" => OtherStructures::Sheds,

            "Stable(s)" => OtherStructures::Stables,

            "Storage" => OtherStructures::Storage,

            "Tennis Court(s)" => OtherStructures::TennisCourts,

            "Workshop" => OtherStructures::Workshop,

            _ => OtherStructures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for OtherStructures {
    fn from(s: &str) -> OtherStructures {
        match s {
            "Airplane Hangar" => OtherStructures::AirplaneHangar,

            "Arena" => OtherStructures::Arena,

            "Barn(s)" => OtherStructures::Barns,

            "Boat House" => OtherStructures::BoatHouse,

            "Cabana" => OtherStructures::Cabana,

            "Cave(s)" => OtherStructures::Caves,

            "Corral(s)" => OtherStructures::Corrals,

            "Covered Arena" => OtherStructures::CoveredArena,

            "Garage(s)" => OtherStructures::Garages,

            "Gazebo" => OtherStructures::Gazebo,

            "Grain Storage" => OtherStructures::GrainStorage,

            "Greenhouse" => OtherStructures::Greenhouse,

            "Guest House" => OtherStructures::GuestHouse,

            "Kennel/Dog Run" => OtherStructures::KennelDogRun,

            "Mobile Home" => OtherStructures::MobileHome,

            "None" => OtherStructures::None,

            "Other" => OtherStructures::Other,

            "Outbuilding" => OtherStructures::Outbuilding,

            "Outdoor Kitchen" => OtherStructures::OutdoorKitchen,

            "Packing Shed" => OtherStructures::PackingShed,

            "Pergola" => OtherStructures::Pergola,

            "Pool House" => OtherStructures::PoolHouse,

            "Poultry Coop" => OtherStructures::PoultryCoop,

            "Residence" => OtherStructures::Residence,

            "RV/Boat Storage" => OtherStructures::RVBoatStorage,

            "Second Garage" => OtherStructures::SecondGarage,

            "Second Residence" => OtherStructures::SecondResidence,

            "See Remarks" => OtherStructures::SeeRemarks,

            "Shed(s)" => OtherStructures::Sheds,

            "Stable(s)" => OtherStructures::Stables,

            "Storage" => OtherStructures::Storage,

            "Tennis Court(s)" => OtherStructures::TennisCourts,

            "Workshop" => OtherStructures::Workshop,

            _ => OtherStructures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a OtherStructures> for &'a str {
    fn from(s: &'a OtherStructures) -> &'a str {
        match s {
            OtherStructures::AirplaneHangar => "Airplane Hangar",

            OtherStructures::Arena => "Arena",

            OtherStructures::Barns => "Barn(s)",

            OtherStructures::BoatHouse => "Boat House",

            OtherStructures::Cabana => "Cabana",

            OtherStructures::Caves => "Cave(s)",

            OtherStructures::Corrals => "Corral(s)",

            OtherStructures::CoveredArena => "Covered Arena",

            OtherStructures::Garages => "Garage(s)",

            OtherStructures::Gazebo => "Gazebo",

            OtherStructures::GrainStorage => "Grain Storage",

            OtherStructures::Greenhouse => "Greenhouse",

            OtherStructures::GuestHouse => "Guest House",

            OtherStructures::KennelDogRun => "Kennel/Dog Run",

            OtherStructures::MobileHome => "Mobile Home",

            OtherStructures::None => "None",

            OtherStructures::Other => "Other",

            OtherStructures::Outbuilding => "Outbuilding",

            OtherStructures::OutdoorKitchen => "Outdoor Kitchen",

            OtherStructures::PackingShed => "Packing Shed",

            OtherStructures::Pergola => "Pergola",

            OtherStructures::PoolHouse => "Pool House",

            OtherStructures::PoultryCoop => "Poultry Coop",

            OtherStructures::Residence => "Residence",

            OtherStructures::RVBoatStorage => "RV/Boat Storage",

            OtherStructures::SecondGarage => "Second Garage",

            OtherStructures::SecondResidence => "Second Residence",

            OtherStructures::SeeRemarks => "See Remarks",

            OtherStructures::Sheds => "Shed(s)",

            OtherStructures::Stables => "Stable(s)",

            OtherStructures::Storage => "Storage",

            OtherStructures::TennisCourts => "Tennis Court(s)",

            OtherStructures::Workshop => "Workshop",

            OtherStructures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for OtherStructures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OtherStructures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
