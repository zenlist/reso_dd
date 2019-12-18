// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ParkingFeatures Lookups](https://ddwiki.reso.org/display/DDW17/ParkingFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParkingFeatures {
    /// "[Additional Parking](https://ddwiki.reso.org/display/DDW17/Additional+Parking)": The property has additional parking.
    AdditionalParking,

    /// "[Aggregate](https://ddwiki.reso.org/display/DDW17/Aggregate)": While aggregate is a type of concrete, it is different in application, maintenance and durability.  Aggregate, aka exposed aggregate concrete, is a mixture poured much in the same way as concrete, but which later has its top surface removed in order to expose the aggregate underneath.
    Aggregate,

    /// "[Alley Access](https://ddwiki.reso.org/display/DDW17/Alley+Access)": The property has alley access.
    AlleyAccess,

    /// "[Asphalt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245967)": The property has asphalt parking.
    Asphalt,

    /// "[Assigned](https://ddwiki.reso.org/display/DDW17/Assigned)": The property has assigned parking spaces.
    Assigned,

    /// "[Attached](https://ddwiki.reso.org/display/DDW17/Attached)": The property has attached parking.
    Attached,

    /// "[Attached Carport](https://ddwiki.reso.org/display/DDW17/Attached+Carport)": The property has an attached carport.
    AttachedCarport,

    /// "[Basement](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246035)": A basement garage is partially or mostly below grade, with its entrance level with the basement floor.
    Basement,

    /// "[Boat](https://ddwiki.reso.org/display/DDW17/Boat)": The property has a space to park/store a boat.
    Boat,

    /// "[Carport](https://ddwiki.reso.org/display/DDW17/Carport)": The property has a carport.
    Carport,

    /// "[Circular Driveway](https://ddwiki.reso.org/display/DDW17/Circular+Driveway)": The property has a circular driveway.
    CircularDriveway,

    /// "[Common](https://ddwiki.reso.org/display/DDW17/Common)": The property has common/shared parking.
    Common,

    /// "[Community Structure](https://ddwiki.reso.org/display/DDW17/Community+Structure)": The property has a community parking structure.
    CommunityStructure,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245976)": The property has concrete paved parking.
    Concrete,

    /// "[Converted Garage](https://ddwiki.reso.org/display/DDW17/Converted+Garage)": The property has a converted garage.
    ConvertedGarage,

    /// "[Covered](https://ddwiki.reso.org/display/DDW17/Covered)": The property has covered parking.
    Covered,

    /// "[Deck](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245979)": The property has deck for parking.
    Deck,

    /// "[Deeded](https://ddwiki.reso.org/display/DDW17/Deeded)": The property has deeded parking.
    Deeded,

    /// "[Detached](https://ddwiki.reso.org/display/DDW17/Detached)": The property has detached parking.
    Detached,

    /// "[Detached Carport](https://ddwiki.reso.org/display/DDW17/Detached+Carport)": The property has a detached carport.
    DetachedCarport,

    /// "[Direct Access](https://ddwiki.reso.org/display/DDW17/Direct+Access)": The parking has direct access to the property or structure.
    DirectAccess,

    /// "[Drive Through](https://ddwiki.reso.org/display/DDW17/Drive+Through)": The property has drive through parking.
    DriveThrough,

    /// "[Driveway](https://ddwiki.reso.org/display/DDW17/Driveway)": The property has a driveway.
    Driveway,

    /// "[Electric Gate](https://ddwiki.reso.org/display/DDW17/Electric+Gate)": The property has an electric gate.
    ElectricGate,

    /// "[Electric Vehicle Charging Station(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245987)": The property has one or more electric vehicle charging station.
    ElectricVehicleChargingStations,

    /// "[Enclosed](https://ddwiki.reso.org/display/DDW17/Enclosed)": The property has enclosed parking.
    Enclosed,

    /// "[Garage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245989)": The property has a garage.
    Garage,

    /// "[Garage Door Opener](https://ddwiki.reso.org/display/DDW17/Garage+Door+Opener)": The garage has an automatic garage door opener.
    GarageDoorOpener,

    /// "[Garage Faces Front](https://ddwiki.reso.org/display/DDW17/Garage+Faces+Front)": The property has garage that faces the front of the property.
    GarageFacesFront,

    /// "[Garage Faces Rear](https://ddwiki.reso.org/display/DDW17/Garage+Faces+Rear)": The property has garage that faces the rear of the property.
    GarageFacesRear,

    /// "[Garage Faces Side](https://ddwiki.reso.org/display/DDW17/Garage+Faces+Side)": The property has garage that faces the side of the property.
    GarageFacesSide,

    /// "[Gated](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245994)": The property has gated parking.
    Gated,

    /// "[Golf Cart Garage](https://ddwiki.reso.org/display/DDW17/Golf+Cart+Garage)": The property has a golf cart garage.
    GolfCartGarage,

    /// "[Gravel](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245996)": The property has parking on gravel.
    Gravel,

    /// "[Guest](https://ddwiki.reso.org/display/DDW17/Guest)": The property has guest parking.
    Guest,

    /// "[Heated Garage](https://ddwiki.reso.org/display/DDW17/Heated+Garage)": The property has a heated garage.
    HeatedGarage,

    /// "[Inside Entrance](https://ddwiki.reso.org/display/DDW17/Inside+Entrance)": The property has parking with an inside entrance.
    InsideEntrance,

    /// "[Kitchen Level](https://ddwiki.reso.org/display/DDW17/Kitchen+Level)": The property has parking at the kitchen level.
    KitchenLevel,

    /// "[Leased](https://ddwiki.reso.org/display/DDW17/Leased)": The property has leased parking.
    Leased,

    /// "[Lighted](https://ddwiki.reso.org/display/DDW17/Lighted)": The property has lighted parking.
    Lighted,

    /// "[No Garage](https://ddwiki.reso.org/display/DDW17/No+Garage)": The property has no garage.
    NoGarage,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246004)": The property does not include parking or no parking is available.
    None,

    /// "[Off Site](https://ddwiki.reso.org/display/DDW17/Off+Site)": The property has off site parking.
    OffSite,

    /// "[Off Street](https://ddwiki.reso.org/display/DDW17/Off+Street)": The property has off street parking.
    OffStreet,

    /// "[On Site](https://ddwiki.reso.org/display/DDW17/On+Site)": The property has on site parking.
    OnSite,

    /// "[On Street](https://ddwiki.reso.org/display/DDW17/On+Street)": The property has on street parking only.
    OnStreet,

    /// "[Open](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246009)": The property has open or unassigned parking.
    Open,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246010)": The property has parking features other than those included in this list.
    Other,

    /// "[Outside](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246011)": The property has outside parking which is not enclosed.
    Outside,

    /// "[Oversized](https://ddwiki.reso.org/display/DDW17/Oversized)": The property has parking for oversized vehicles.
    Oversized,

    /// "[Parking Lot](https://ddwiki.reso.org/display/DDW17/Parking+Lot)": The property has access to a parking lot.
    ParkingLot,

    /// "[Parking Pad](https://ddwiki.reso.org/display/DDW17/Parking+Pad)": The property has a parking pad.
    ParkingPad,

    /// "[Paved](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246015)": The property has paved parking.
    Paved,

    /// "[Paver Block](https://ddwiki.reso.org/display/DDW17/Paver+Block)": The property has parking on paver blocks.
    PaverBlock,

    /// "[Permit Required](https://ddwiki.reso.org/display/DDW17/Permit+Required)": Parking at the property or on the street requires a permit.
    PermitRequired,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246018)": The property has private parking.
    Private,

    /// "[RV Access/Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246019)": The property has access/parking for recreational vehicles.
    RVAccessParking,

    /// "[RV Carport](https://ddwiki.reso.org/display/DDW17/RV+Carport)": The property has a carport for recreational vehicles.
    RVCarport,

    /// "[RV Garage](https://ddwiki.reso.org/display/DDW17/RV+Garage)": The property has a garage for recreational vehicles.
    RVGarage,

    /// "[RV Gated](https://ddwiki.reso.org/display/DDW17/RV+Gated)": The property has gated parking for recreational vehicles.
    RVGated,

    /// "[Secured](https://ddwiki.reso.org/display/DDW17/Secured)": The property has secure parking.
    Secured,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246024)": See remarks for additional information about parking.
    SeeRemarks,

    /// "[Shared Driveway](https://ddwiki.reso.org/display/DDW17/Shared+Driveway)": The property has a shared driveway.
    SharedDriveway,

    /// "[Side By Side](https://ddwiki.reso.org/display/DDW17/Side+By+Side)": The property has side by side parking spaces.
    SideBySide,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246027)": The property has storage in the parking area.
    Storage,

    /// "[Tandem](https://ddwiki.reso.org/display/DDW17/Tandem)": The property has tandem parking.
    Tandem,

    /// "[Unassigned](https://ddwiki.reso.org/display/DDW17/Unassigned)": The property has unassigned or open parking.
    Unassigned,

    /// "[Underground](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246030)": The property has underground parking.
    Underground,

    /// "[Unpaved](https://ddwiki.reso.org/display/DDW17/Unpaved)": The property has parking on an unpaved surface.
    Unpaved,

    /// "[Valet](https://ddwiki.reso.org/display/DDW17/Valet)": The property has valet parking available.
    Valet,

    /// "[Varies by Unit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246033)": The parking varies from unit to unit.
    VariesbyUnit,

    /// "[Workshop in Garage](https://ddwiki.reso.org/display/DDW17/Workshop+in+Garage)": The property has workshop in the garage.
    WorkshopinGarage,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for ParkingFeatures {
    fn from(s: String) -> ParkingFeatures {
        match s.as_ref() {
            "Additional Parking" => ParkingFeatures::AdditionalParking,

            "Aggregate" => ParkingFeatures::Aggregate,

            "Alley Access" => ParkingFeatures::AlleyAccess,

            "Asphalt" => ParkingFeatures::Asphalt,

            "Assigned" => ParkingFeatures::Assigned,

            "Attached" => ParkingFeatures::Attached,

            "Attached Carport" => ParkingFeatures::AttachedCarport,

            "Basement" => ParkingFeatures::Basement,

            "Boat" => ParkingFeatures::Boat,

            "Carport" => ParkingFeatures::Carport,

            "Circular Driveway" => ParkingFeatures::CircularDriveway,

            "Common" => ParkingFeatures::Common,

            "Community Structure" => ParkingFeatures::CommunityStructure,

            "Concrete" => ParkingFeatures::Concrete,

            "Converted Garage" => ParkingFeatures::ConvertedGarage,

            "Covered" => ParkingFeatures::Covered,

            "Deck" => ParkingFeatures::Deck,

            "Deeded" => ParkingFeatures::Deeded,

            "Detached" => ParkingFeatures::Detached,

            "Detached Carport" => ParkingFeatures::DetachedCarport,

            "Direct Access" => ParkingFeatures::DirectAccess,

            "Drive Through" => ParkingFeatures::DriveThrough,

            "Driveway" => ParkingFeatures::Driveway,

            "Electric Gate" => ParkingFeatures::ElectricGate,

            "Electric Vehicle Charging Station(s)" => {
                ParkingFeatures::ElectricVehicleChargingStations
            }

            "Enclosed" => ParkingFeatures::Enclosed,

            "Garage" => ParkingFeatures::Garage,

            "Garage Door Opener" => ParkingFeatures::GarageDoorOpener,

            "Garage Faces Front" => ParkingFeatures::GarageFacesFront,

            "Garage Faces Rear" => ParkingFeatures::GarageFacesRear,

            "Garage Faces Side" => ParkingFeatures::GarageFacesSide,

            "Gated" => ParkingFeatures::Gated,

            "Golf Cart Garage" => ParkingFeatures::GolfCartGarage,

            "Gravel" => ParkingFeatures::Gravel,

            "Guest" => ParkingFeatures::Guest,

            "Heated Garage" => ParkingFeatures::HeatedGarage,

            "Inside Entrance" => ParkingFeatures::InsideEntrance,

            "Kitchen Level" => ParkingFeatures::KitchenLevel,

            "Leased" => ParkingFeatures::Leased,

            "Lighted" => ParkingFeatures::Lighted,

            "No Garage" => ParkingFeatures::NoGarage,

            "None" => ParkingFeatures::None,

            "Off Site" => ParkingFeatures::OffSite,

            "Off Street" => ParkingFeatures::OffStreet,

            "On Site" => ParkingFeatures::OnSite,

            "On Street" => ParkingFeatures::OnStreet,

            "Open" => ParkingFeatures::Open,

            "Other" => ParkingFeatures::Other,

            "Outside" => ParkingFeatures::Outside,

            "Oversized" => ParkingFeatures::Oversized,

            "Parking Lot" => ParkingFeatures::ParkingLot,

            "Parking Pad" => ParkingFeatures::ParkingPad,

            "Paved" => ParkingFeatures::Paved,

            "Paver Block" => ParkingFeatures::PaverBlock,

            "Permit Required" => ParkingFeatures::PermitRequired,

            "Private" => ParkingFeatures::Private,

            "RV Access/Parking" => ParkingFeatures::RVAccessParking,

            "RV Carport" => ParkingFeatures::RVCarport,

            "RV Garage" => ParkingFeatures::RVGarage,

            "RV Gated" => ParkingFeatures::RVGated,

            "Secured" => ParkingFeatures::Secured,

            "See Remarks" => ParkingFeatures::SeeRemarks,

            "Shared Driveway" => ParkingFeatures::SharedDriveway,

            "Side By Side" => ParkingFeatures::SideBySide,

            "Storage" => ParkingFeatures::Storage,

            "Tandem" => ParkingFeatures::Tandem,

            "Unassigned" => ParkingFeatures::Unassigned,

            "Underground" => ParkingFeatures::Underground,

            "Unpaved" => ParkingFeatures::Unpaved,

            "Valet" => ParkingFeatures::Valet,

            "Varies by Unit" => ParkingFeatures::VariesbyUnit,

            "Workshop in Garage" => ParkingFeatures::WorkshopinGarage,

            _ => ParkingFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ParkingFeatures {
    fn from(s: &str) -> ParkingFeatures {
        match s {
            "Additional Parking" => ParkingFeatures::AdditionalParking,

            "Aggregate" => ParkingFeatures::Aggregate,

            "Alley Access" => ParkingFeatures::AlleyAccess,

            "Asphalt" => ParkingFeatures::Asphalt,

            "Assigned" => ParkingFeatures::Assigned,

            "Attached" => ParkingFeatures::Attached,

            "Attached Carport" => ParkingFeatures::AttachedCarport,

            "Basement" => ParkingFeatures::Basement,

            "Boat" => ParkingFeatures::Boat,

            "Carport" => ParkingFeatures::Carport,

            "Circular Driveway" => ParkingFeatures::CircularDriveway,

            "Common" => ParkingFeatures::Common,

            "Community Structure" => ParkingFeatures::CommunityStructure,

            "Concrete" => ParkingFeatures::Concrete,

            "Converted Garage" => ParkingFeatures::ConvertedGarage,

            "Covered" => ParkingFeatures::Covered,

            "Deck" => ParkingFeatures::Deck,

            "Deeded" => ParkingFeatures::Deeded,

            "Detached" => ParkingFeatures::Detached,

            "Detached Carport" => ParkingFeatures::DetachedCarport,

            "Direct Access" => ParkingFeatures::DirectAccess,

            "Drive Through" => ParkingFeatures::DriveThrough,

            "Driveway" => ParkingFeatures::Driveway,

            "Electric Gate" => ParkingFeatures::ElectricGate,

            "Electric Vehicle Charging Station(s)" => {
                ParkingFeatures::ElectricVehicleChargingStations
            }

            "Enclosed" => ParkingFeatures::Enclosed,

            "Garage" => ParkingFeatures::Garage,

            "Garage Door Opener" => ParkingFeatures::GarageDoorOpener,

            "Garage Faces Front" => ParkingFeatures::GarageFacesFront,

            "Garage Faces Rear" => ParkingFeatures::GarageFacesRear,

            "Garage Faces Side" => ParkingFeatures::GarageFacesSide,

            "Gated" => ParkingFeatures::Gated,

            "Golf Cart Garage" => ParkingFeatures::GolfCartGarage,

            "Gravel" => ParkingFeatures::Gravel,

            "Guest" => ParkingFeatures::Guest,

            "Heated Garage" => ParkingFeatures::HeatedGarage,

            "Inside Entrance" => ParkingFeatures::InsideEntrance,

            "Kitchen Level" => ParkingFeatures::KitchenLevel,

            "Leased" => ParkingFeatures::Leased,

            "Lighted" => ParkingFeatures::Lighted,

            "No Garage" => ParkingFeatures::NoGarage,

            "None" => ParkingFeatures::None,

            "Off Site" => ParkingFeatures::OffSite,

            "Off Street" => ParkingFeatures::OffStreet,

            "On Site" => ParkingFeatures::OnSite,

            "On Street" => ParkingFeatures::OnStreet,

            "Open" => ParkingFeatures::Open,

            "Other" => ParkingFeatures::Other,

            "Outside" => ParkingFeatures::Outside,

            "Oversized" => ParkingFeatures::Oversized,

            "Parking Lot" => ParkingFeatures::ParkingLot,

            "Parking Pad" => ParkingFeatures::ParkingPad,

            "Paved" => ParkingFeatures::Paved,

            "Paver Block" => ParkingFeatures::PaverBlock,

            "Permit Required" => ParkingFeatures::PermitRequired,

            "Private" => ParkingFeatures::Private,

            "RV Access/Parking" => ParkingFeatures::RVAccessParking,

            "RV Carport" => ParkingFeatures::RVCarport,

            "RV Garage" => ParkingFeatures::RVGarage,

            "RV Gated" => ParkingFeatures::RVGated,

            "Secured" => ParkingFeatures::Secured,

            "See Remarks" => ParkingFeatures::SeeRemarks,

            "Shared Driveway" => ParkingFeatures::SharedDriveway,

            "Side By Side" => ParkingFeatures::SideBySide,

            "Storage" => ParkingFeatures::Storage,

            "Tandem" => ParkingFeatures::Tandem,

            "Unassigned" => ParkingFeatures::Unassigned,

            "Underground" => ParkingFeatures::Underground,

            "Unpaved" => ParkingFeatures::Unpaved,

            "Valet" => ParkingFeatures::Valet,

            "Varies by Unit" => ParkingFeatures::VariesbyUnit,

            "Workshop in Garage" => ParkingFeatures::WorkshopinGarage,

            _ => ParkingFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ParkingFeatures> for &'a str {
    fn from(s: &'a ParkingFeatures) -> &'a str {
        match s {
            ParkingFeatures::AdditionalParking => "Additional Parking",

            ParkingFeatures::Aggregate => "Aggregate",

            ParkingFeatures::AlleyAccess => "Alley Access",

            ParkingFeatures::Asphalt => "Asphalt",

            ParkingFeatures::Assigned => "Assigned",

            ParkingFeatures::Attached => "Attached",

            ParkingFeatures::AttachedCarport => "Attached Carport",

            ParkingFeatures::Basement => "Basement",

            ParkingFeatures::Boat => "Boat",

            ParkingFeatures::Carport => "Carport",

            ParkingFeatures::CircularDriveway => "Circular Driveway",

            ParkingFeatures::Common => "Common",

            ParkingFeatures::CommunityStructure => "Community Structure",

            ParkingFeatures::Concrete => "Concrete",

            ParkingFeatures::ConvertedGarage => "Converted Garage",

            ParkingFeatures::Covered => "Covered",

            ParkingFeatures::Deck => "Deck",

            ParkingFeatures::Deeded => "Deeded",

            ParkingFeatures::Detached => "Detached",

            ParkingFeatures::DetachedCarport => "Detached Carport",

            ParkingFeatures::DirectAccess => "Direct Access",

            ParkingFeatures::DriveThrough => "Drive Through",

            ParkingFeatures::Driveway => "Driveway",

            ParkingFeatures::ElectricGate => "Electric Gate",

            ParkingFeatures::ElectricVehicleChargingStations => {
                "Electric Vehicle Charging Station(s)"
            }

            ParkingFeatures::Enclosed => "Enclosed",

            ParkingFeatures::Garage => "Garage",

            ParkingFeatures::GarageDoorOpener => "Garage Door Opener",

            ParkingFeatures::GarageFacesFront => "Garage Faces Front",

            ParkingFeatures::GarageFacesRear => "Garage Faces Rear",

            ParkingFeatures::GarageFacesSide => "Garage Faces Side",

            ParkingFeatures::Gated => "Gated",

            ParkingFeatures::GolfCartGarage => "Golf Cart Garage",

            ParkingFeatures::Gravel => "Gravel",

            ParkingFeatures::Guest => "Guest",

            ParkingFeatures::HeatedGarage => "Heated Garage",

            ParkingFeatures::InsideEntrance => "Inside Entrance",

            ParkingFeatures::KitchenLevel => "Kitchen Level",

            ParkingFeatures::Leased => "Leased",

            ParkingFeatures::Lighted => "Lighted",

            ParkingFeatures::NoGarage => "No Garage",

            ParkingFeatures::None => "None",

            ParkingFeatures::OffSite => "Off Site",

            ParkingFeatures::OffStreet => "Off Street",

            ParkingFeatures::OnSite => "On Site",

            ParkingFeatures::OnStreet => "On Street",

            ParkingFeatures::Open => "Open",

            ParkingFeatures::Other => "Other",

            ParkingFeatures::Outside => "Outside",

            ParkingFeatures::Oversized => "Oversized",

            ParkingFeatures::ParkingLot => "Parking Lot",

            ParkingFeatures::ParkingPad => "Parking Pad",

            ParkingFeatures::Paved => "Paved",

            ParkingFeatures::PaverBlock => "Paver Block",

            ParkingFeatures::PermitRequired => "Permit Required",

            ParkingFeatures::Private => "Private",

            ParkingFeatures::RVAccessParking => "RV Access/Parking",

            ParkingFeatures::RVCarport => "RV Carport",

            ParkingFeatures::RVGarage => "RV Garage",

            ParkingFeatures::RVGated => "RV Gated",

            ParkingFeatures::Secured => "Secured",

            ParkingFeatures::SeeRemarks => "See Remarks",

            ParkingFeatures::SharedDriveway => "Shared Driveway",

            ParkingFeatures::SideBySide => "Side By Side",

            ParkingFeatures::Storage => "Storage",

            ParkingFeatures::Tandem => "Tandem",

            ParkingFeatures::Unassigned => "Unassigned",

            ParkingFeatures::Underground => "Underground",

            ParkingFeatures::Unpaved => "Unpaved",

            ParkingFeatures::Valet => "Valet",

            ParkingFeatures::VariesbyUnit => "Varies by Unit",

            ParkingFeatures::WorkshopinGarage => "Workshop in Garage",

            ParkingFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ParkingFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ParkingFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_parking_features_format {
    use super::ParkingFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<ParkingFeatures>>,
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
    ) -> Result<Option<Vec<ParkingFeatures>>, D::Error>
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
