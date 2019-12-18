// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [AssociationAmenities Lookups](https://ddwiki.reso.org/display/DDW17/AssociationAmenities+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssociationAmenities {
    /// "[Airport/Runway](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243763)": The home owner's association includes access or some service related to an airport or runway.
    AirportRunway,

    /// "[Barbecue](https://ddwiki.reso.org/display/DDW17/Barbecue)": The home owner's association includes use of, or access to, a barbecue.
    Barbecue,

    /// "[Basketball Court](https://ddwiki.reso.org/display/DDW17/Basketball+Court)": The home owner's association includes use of, or access to, a basketball court.
    BasketballCourt,

    /// "[Beach Access](https://ddwiki.reso.org/display/DDW17/Beach+Access)": The home owner's association includes access to a beach.
    BeachAccess,

    /// "[Beach Rights](https://ddwiki.reso.org/display/DDW17/Beach+Rights)": The home owner's association includes use of a beach that has beach rights restrictions.
    BeachRights,

    /// "[Billiard Room](https://ddwiki.reso.org/display/DDW17/Billiard+Room)": The home owner's association includes use of, or access to, a billiard room.
    BilliardRoom,

    /// "[Boat Dock](https://ddwiki.reso.org/display/DDW17/Boat+Dock)": The home owner's association includes use of, or access to, a boat dock.
    BoatDock,

    /// "[Boat Slip](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243771)": The home owner's association includes use of, or access to, a boat slip.
    BoatSlip,

    /// "[Boating](https://ddwiki.reso.org/display/DDW17/Boating)": The home owner's association includes use of, or access to, boating.
    Boating,

    /// "[Cabana](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243773)": The home owner's association includes use of, or access to, a cabana.
    Cabana,

    /// "[Cable TV](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243774)": The home owner's association includes cable based services.
    CableTV,

    /// "[Car Wash Area](https://ddwiki.reso.org/display/DDW17/Car+Wash+Area)": The home owner's association includes use of, or access to, an are to wash your car.
    CarWashArea,

    /// "[Clubhouse](https://ddwiki.reso.org/display/DDW17/Clubhouse)": The home owner's association includes use of, or access to, a clubhouse.
    Clubhouse,

    /// "[Coin Laundry](https://ddwiki.reso.org/display/DDW17/Coin+Laundry)": The home owner's association includes use of, or access to, a coin laundry.
    CoinLaundry,

    /// "[Concierge](https://ddwiki.reso.org/display/DDW17/Concierge)": The home owner's association includes use of, or access to, a concierge service.
    Concierge,

    /// "[Day Care](https://ddwiki.reso.org/display/DDW17/Day+Care)": The home owner's association includes use of, or access to, a day care service.
    DayCare,

    /// "[Dog Park](https://ddwiki.reso.org/display/DDW17/Dog+Park)": The home owner's association includes use of, or access to, a dog park.
    DogPark,

    /// "[Dry Dock](https://ddwiki.reso.org/display/DDW17/Dry+Dock)": The home owner's association includes use of, or access to, a dry dock.
    DryDock,

    /// "[Electricity](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243782)": The home owner's association includes electricity.
    Electricity,

    /// "[Elevator(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243783)": The home owner's association includes use of, or access to, an elevator(s).
    Elevators,

    /// "[Exercise Course](https://ddwiki.reso.org/display/DDW17/Exercise+Course)": The home owner's association includes use of, or access to, an exercise course.
    ExerciseCourse,

    /// "[Fitness Center](https://ddwiki.reso.org/display/DDW17/Fitness+Center)": The home owner's association includes use of, or access to, a fitness center.
    FitnessCenter,

    /// "[Game Court Exterior](https://ddwiki.reso.org/display/DDW17/Game+Court+Exterior)": The home owner's association includes use of, or access to, an outdoors game court.
    GameCourtExterior,

    /// "[Game Court Interior](https://ddwiki.reso.org/display/DDW17/Game+Court+Interior)": The home owner's association includes use of, or access to, an indoors game court.
    GameCourtInterior,

    /// "[Game Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243788)": The home owner's association includes use of, or access to, a game room           .
    GameRoom,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243789)": The home owner's association includes natural gas.
    Gas,

    /// "[Gated](https://ddwiki.reso.org/display/DDW17/Gated)": The home owner's association property/area is gated.
    Gated,

    /// "[Golf Course](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243791)": The home owner's association includes use of, or access to, a golf course.
    GolfCourse,

    /// "[Hot Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243792)": The home owner's association includes hot water.
    HotWater,

    /// "[Indoor Pool](https://ddwiki.reso.org/display/DDW17/Indoor+Pool)": The home owner's association includes use of, or access to, an indoor pool.
    IndoorPool,

    /// "[Insurance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243794)": The home owner's association includes insurance.
    Insurance,

    /// "[Jogging Path](https://ddwiki.reso.org/display/DDW17/Jogging+Path)": The home owner's association includes use of, or access to, a jogging path.
    JoggingPath,

    /// "[Landscaping](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243796)": The home owner's association includes landscaping.
    Landscaping,

    /// "[Laundry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243797)": The home owner's association includes laundry.
    Laundry,

    /// "[Maid service](https://ddwiki.reso.org/display/DDW17/Maid+service)": The home owner's association includes use of, or access to, a maid service.
    Maidservice,

    /// "[Maintenance](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243799)": The home owner's association includes maintenance.
    Maintenance,

    /// "[Maintenance Grounds](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243838)": The home owner's association includes grounds maintenance.
    MaintenanceGrounds,

    /// "[Maintenance Structure](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243772)": The home owner's association includes building maintenance.
    MaintenanceStructure,

    /// "[Management](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243800)": The home owner's association includes management services.
    Management,

    /// "[Marina](https://ddwiki.reso.org/display/DDW17/Marina)": The home owner's association includes use of, or access to, a marina.
    Marina,

    /// "[Meeting Room](https://ddwiki.reso.org/display/DDW17/Meeting+Room)": The home owner's association includes use of, or access to, a meeting room.
    MeetingRoom,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243803)": The home owner's association has no amenities.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243804)": The home owner's association includes amenities not included in this list.
    Other,

    /// "[Park](https://ddwiki.reso.org/display/DDW17/Park)": The home owner's association includes use of, or access to, a park.
    Park,

    /// "[Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243806)": The home owner's association includes use of, or access to, parking.
    Parking,

    /// "[Party Room](https://ddwiki.reso.org/display/DDW17/Party+Room)": The home owner's association includes use of, or access to, a party room.
    PartyRoom,

    /// "[Picnic Area](https://ddwiki.reso.org/display/DDW17/Picnic+Area)": The home owner's association includes use of, or access to, a picnic area.
    PicnicArea,

    /// "[Playground](https://ddwiki.reso.org/display/DDW17/Playground)": The home owner's association includes use of, or access to, a playground.
    Playground,

    /// "[Pond Seasonal](https://ddwiki.reso.org/display/DDW17/Pond+Seasonal)": The home owner's association includes use of, or access to, a seasonal pond.
    PondSeasonal,

    /// "[Pond Year Round](https://ddwiki.reso.org/display/DDW17/Pond+Year+Round)": The home owner's association includes use of, or access to, a year round pond.
    PondYearRound,

    /// "[Pool](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243812)": The home owner's association includes use of, or access to, a pool.
    Pool,

    /// "[Powered Boats Allowed](https://ddwiki.reso.org/display/DDW17/Powered+Boats+Allowed)": The home owner's association allows the use of powered boats.
    PoweredBoatsAllowed,

    /// "[Racquetball](https://ddwiki.reso.org/display/DDW17/Racquetball)": The home owner's association includes use of, or access to, a racquetball court(s).
    Racquetball,

    /// "[Recreation Facilities](https://ddwiki.reso.org/display/DDW17/Recreation+Facilities)": The home owner's association includes use of, or access to, recreation facilities.
    RecreationFacilities,

    /// "[Recreation Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243816)": The home owner's association includes use of, or access to, a recreation room.
    RecreationRoom,

    /// "[Roof Deck](https://ddwiki.reso.org/display/DDW17/Roof+Deck)": The home owner's association includes use of, or access to, a roof deck.
    RoofDeck,

    /// "[RV Parking](https://ddwiki.reso.org/display/DDW17/RV+Parking)": The home owner's association includes use of, or access to, recreational vehicle (RV) parking.
    RVParking,

    /// "[RV/Boat Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243818)": The home owner's association includes use of, or access to, RV and/or boat storage.
    RVBoatStorage,

    /// "[Sauna](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243820)": The home owner's association includes use of, or access to, a sauna.
    Sauna,

    /// "[Security](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243821)": The home owner's association includes security services.
    Security,

    /// "[Service Elevator(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243822)": The home owner's association includes use of, or access to, a service elevator(s).
    ServiceElevators,

    /// "[Shuffleboard Court](https://ddwiki.reso.org/display/DDW17/Shuffleboard+Court)": The home owner's association includes use of, or access to, a shuffleboard court.
    ShuffleboardCourt,

    /// "[Ski Accessible](https://ddwiki.reso.org/display/DDW17/Ski+Accessible)": The home owner's association includes access to skiing.
    SkiAccessible,

    /// "[Snow Removal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243825)": The home owner's association includes snow removal.
    SnowRemoval,

    /// "[Spa/Hot Tub](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243826)": The home owner's association includes use of, or access to, a spa and/or hot tub.
    SpaHotTub,

    /// "[Sport Court](https://ddwiki.reso.org/display/DDW17/Sport+Court)": The home owner's association includes use of, or access to, a sport court.
    SportCourt,

    /// "[Stable(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243828)": The home owner's association includes use of, or access to, horse stable(s).
    Stables,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243829)": The home owner's association includes storage space.
    Storage,

    /// "[Stream Seasonal](https://ddwiki.reso.org/display/DDW17/Stream+Seasonal)": The home owner's association includes use of, or access to, a seasonal stream.
    StreamSeasonal,

    /// "[Stream Year Round](https://ddwiki.reso.org/display/DDW17/Stream+Year+Round)": The home owner's association includes use of, or access to, a year round accessible stream.
    StreamYearRound,

    /// "[Taxes](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243832)": The home owner's association includes taxes.
    Taxes,

    /// "[Tennis Court(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243833)": The home owner's association includes use of, or access to, a tennis court(s).
    TennisCourts,

    /// "[Trail(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243834)": The home owner's association includes use of, or access to, a trail(s).
    Trails,

    /// "[Trash](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243835)": The home owner's association includes trash service.
    Trash,

    /// "[Water](https://ddwiki.reso.org/pages/viewpage.action?pageId=29243836)": The home owner's association includes water.
    Water,

    /// "[Workshop Area](https://ddwiki.reso.org/display/DDW17/Workshop+Area)": The home owner's association includes use of, or access to, a workshop area.
    WorkshopArea,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for AssociationAmenities {
    fn from(s: String) -> AssociationAmenities {
        match s.as_ref() {
            "Airport/Runway" => AssociationAmenities::AirportRunway,

            "Barbecue" => AssociationAmenities::Barbecue,

            "Basketball Court" => AssociationAmenities::BasketballCourt,

            "Beach Access" => AssociationAmenities::BeachAccess,

            "Beach Rights" => AssociationAmenities::BeachRights,

            "Billiard Room" => AssociationAmenities::BilliardRoom,

            "Boat Dock" => AssociationAmenities::BoatDock,

            "Boat Slip" => AssociationAmenities::BoatSlip,

            "Boating" => AssociationAmenities::Boating,

            "Cabana" => AssociationAmenities::Cabana,

            "Cable TV" => AssociationAmenities::CableTV,

            "Car Wash Area" => AssociationAmenities::CarWashArea,

            "Clubhouse" => AssociationAmenities::Clubhouse,

            "Coin Laundry" => AssociationAmenities::CoinLaundry,

            "Concierge" => AssociationAmenities::Concierge,

            "Day Care" => AssociationAmenities::DayCare,

            "Dog Park" => AssociationAmenities::DogPark,

            "Dry Dock" => AssociationAmenities::DryDock,

            "Electricity" => AssociationAmenities::Electricity,

            "Elevator(s)" => AssociationAmenities::Elevators,

            "Exercise Course" => AssociationAmenities::ExerciseCourse,

            "Fitness Center" => AssociationAmenities::FitnessCenter,

            "Game Court Exterior" => AssociationAmenities::GameCourtExterior,

            "Game Court Interior" => AssociationAmenities::GameCourtInterior,

            "Game Room" => AssociationAmenities::GameRoom,

            "Gas" => AssociationAmenities::Gas,

            "Gated" => AssociationAmenities::Gated,

            "Golf Course" => AssociationAmenities::GolfCourse,

            "Hot Water" => AssociationAmenities::HotWater,

            "Indoor Pool" => AssociationAmenities::IndoorPool,

            "Insurance" => AssociationAmenities::Insurance,

            "Jogging Path" => AssociationAmenities::JoggingPath,

            "Landscaping" => AssociationAmenities::Landscaping,

            "Laundry" => AssociationAmenities::Laundry,

            "Maid service" => AssociationAmenities::Maidservice,

            "Maintenance" => AssociationAmenities::Maintenance,

            "Maintenance Grounds" => AssociationAmenities::MaintenanceGrounds,

            "Maintenance Structure" => AssociationAmenities::MaintenanceStructure,

            "Management" => AssociationAmenities::Management,

            "Marina" => AssociationAmenities::Marina,

            "Meeting Room" => AssociationAmenities::MeetingRoom,

            "None" => AssociationAmenities::None,

            "Other" => AssociationAmenities::Other,

            "Park" => AssociationAmenities::Park,

            "Parking" => AssociationAmenities::Parking,

            "Party Room" => AssociationAmenities::PartyRoom,

            "Picnic Area" => AssociationAmenities::PicnicArea,

            "Playground" => AssociationAmenities::Playground,

            "Pond Seasonal" => AssociationAmenities::PondSeasonal,

            "Pond Year Round" => AssociationAmenities::PondYearRound,

            "Pool" => AssociationAmenities::Pool,

            "Powered Boats Allowed" => AssociationAmenities::PoweredBoatsAllowed,

            "Racquetball" => AssociationAmenities::Racquetball,

            "Recreation Facilities" => AssociationAmenities::RecreationFacilities,

            "Recreation Room" => AssociationAmenities::RecreationRoom,

            "Roof Deck" => AssociationAmenities::RoofDeck,

            "RV Parking" => AssociationAmenities::RVParking,

            "RV/Boat Storage" => AssociationAmenities::RVBoatStorage,

            "Sauna" => AssociationAmenities::Sauna,

            "Security" => AssociationAmenities::Security,

            "Service Elevator(s)" => AssociationAmenities::ServiceElevators,

            "Shuffleboard Court" => AssociationAmenities::ShuffleboardCourt,

            "Ski Accessible" => AssociationAmenities::SkiAccessible,

            "Snow Removal" => AssociationAmenities::SnowRemoval,

            "Spa/Hot Tub" => AssociationAmenities::SpaHotTub,

            "Sport Court" => AssociationAmenities::SportCourt,

            "Stable(s)" => AssociationAmenities::Stables,

            "Storage" => AssociationAmenities::Storage,

            "Stream Seasonal" => AssociationAmenities::StreamSeasonal,

            "Stream Year Round" => AssociationAmenities::StreamYearRound,

            "Taxes" => AssociationAmenities::Taxes,

            "Tennis Court(s)" => AssociationAmenities::TennisCourts,

            "Trail(s)" => AssociationAmenities::Trails,

            "Trash" => AssociationAmenities::Trash,

            "Water" => AssociationAmenities::Water,

            "Workshop Area" => AssociationAmenities::WorkshopArea,

            _ => AssociationAmenities::OpenEnumeration(s),
        }
    }
}

impl From<&str> for AssociationAmenities {
    fn from(s: &str) -> AssociationAmenities {
        match s {
            "Airport/Runway" => AssociationAmenities::AirportRunway,

            "Barbecue" => AssociationAmenities::Barbecue,

            "Basketball Court" => AssociationAmenities::BasketballCourt,

            "Beach Access" => AssociationAmenities::BeachAccess,

            "Beach Rights" => AssociationAmenities::BeachRights,

            "Billiard Room" => AssociationAmenities::BilliardRoom,

            "Boat Dock" => AssociationAmenities::BoatDock,

            "Boat Slip" => AssociationAmenities::BoatSlip,

            "Boating" => AssociationAmenities::Boating,

            "Cabana" => AssociationAmenities::Cabana,

            "Cable TV" => AssociationAmenities::CableTV,

            "Car Wash Area" => AssociationAmenities::CarWashArea,

            "Clubhouse" => AssociationAmenities::Clubhouse,

            "Coin Laundry" => AssociationAmenities::CoinLaundry,

            "Concierge" => AssociationAmenities::Concierge,

            "Day Care" => AssociationAmenities::DayCare,

            "Dog Park" => AssociationAmenities::DogPark,

            "Dry Dock" => AssociationAmenities::DryDock,

            "Electricity" => AssociationAmenities::Electricity,

            "Elevator(s)" => AssociationAmenities::Elevators,

            "Exercise Course" => AssociationAmenities::ExerciseCourse,

            "Fitness Center" => AssociationAmenities::FitnessCenter,

            "Game Court Exterior" => AssociationAmenities::GameCourtExterior,

            "Game Court Interior" => AssociationAmenities::GameCourtInterior,

            "Game Room" => AssociationAmenities::GameRoom,

            "Gas" => AssociationAmenities::Gas,

            "Gated" => AssociationAmenities::Gated,

            "Golf Course" => AssociationAmenities::GolfCourse,

            "Hot Water" => AssociationAmenities::HotWater,

            "Indoor Pool" => AssociationAmenities::IndoorPool,

            "Insurance" => AssociationAmenities::Insurance,

            "Jogging Path" => AssociationAmenities::JoggingPath,

            "Landscaping" => AssociationAmenities::Landscaping,

            "Laundry" => AssociationAmenities::Laundry,

            "Maid service" => AssociationAmenities::Maidservice,

            "Maintenance" => AssociationAmenities::Maintenance,

            "Maintenance Grounds" => AssociationAmenities::MaintenanceGrounds,

            "Maintenance Structure" => AssociationAmenities::MaintenanceStructure,

            "Management" => AssociationAmenities::Management,

            "Marina" => AssociationAmenities::Marina,

            "Meeting Room" => AssociationAmenities::MeetingRoom,

            "None" => AssociationAmenities::None,

            "Other" => AssociationAmenities::Other,

            "Park" => AssociationAmenities::Park,

            "Parking" => AssociationAmenities::Parking,

            "Party Room" => AssociationAmenities::PartyRoom,

            "Picnic Area" => AssociationAmenities::PicnicArea,

            "Playground" => AssociationAmenities::Playground,

            "Pond Seasonal" => AssociationAmenities::PondSeasonal,

            "Pond Year Round" => AssociationAmenities::PondYearRound,

            "Pool" => AssociationAmenities::Pool,

            "Powered Boats Allowed" => AssociationAmenities::PoweredBoatsAllowed,

            "Racquetball" => AssociationAmenities::Racquetball,

            "Recreation Facilities" => AssociationAmenities::RecreationFacilities,

            "Recreation Room" => AssociationAmenities::RecreationRoom,

            "Roof Deck" => AssociationAmenities::RoofDeck,

            "RV Parking" => AssociationAmenities::RVParking,

            "RV/Boat Storage" => AssociationAmenities::RVBoatStorage,

            "Sauna" => AssociationAmenities::Sauna,

            "Security" => AssociationAmenities::Security,

            "Service Elevator(s)" => AssociationAmenities::ServiceElevators,

            "Shuffleboard Court" => AssociationAmenities::ShuffleboardCourt,

            "Ski Accessible" => AssociationAmenities::SkiAccessible,

            "Snow Removal" => AssociationAmenities::SnowRemoval,

            "Spa/Hot Tub" => AssociationAmenities::SpaHotTub,

            "Sport Court" => AssociationAmenities::SportCourt,

            "Stable(s)" => AssociationAmenities::Stables,

            "Storage" => AssociationAmenities::Storage,

            "Stream Seasonal" => AssociationAmenities::StreamSeasonal,

            "Stream Year Round" => AssociationAmenities::StreamYearRound,

            "Taxes" => AssociationAmenities::Taxes,

            "Tennis Court(s)" => AssociationAmenities::TennisCourts,

            "Trail(s)" => AssociationAmenities::Trails,

            "Trash" => AssociationAmenities::Trash,

            "Water" => AssociationAmenities::Water,

            "Workshop Area" => AssociationAmenities::WorkshopArea,

            _ => AssociationAmenities::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a AssociationAmenities> for &'a str {
    fn from(s: &'a AssociationAmenities) -> &'a str {
        match s {
            AssociationAmenities::AirportRunway => "Airport/Runway",

            AssociationAmenities::Barbecue => "Barbecue",

            AssociationAmenities::BasketballCourt => "Basketball Court",

            AssociationAmenities::BeachAccess => "Beach Access",

            AssociationAmenities::BeachRights => "Beach Rights",

            AssociationAmenities::BilliardRoom => "Billiard Room",

            AssociationAmenities::BoatDock => "Boat Dock",

            AssociationAmenities::BoatSlip => "Boat Slip",

            AssociationAmenities::Boating => "Boating",

            AssociationAmenities::Cabana => "Cabana",

            AssociationAmenities::CableTV => "Cable TV",

            AssociationAmenities::CarWashArea => "Car Wash Area",

            AssociationAmenities::Clubhouse => "Clubhouse",

            AssociationAmenities::CoinLaundry => "Coin Laundry",

            AssociationAmenities::Concierge => "Concierge",

            AssociationAmenities::DayCare => "Day Care",

            AssociationAmenities::DogPark => "Dog Park",

            AssociationAmenities::DryDock => "Dry Dock",

            AssociationAmenities::Electricity => "Electricity",

            AssociationAmenities::Elevators => "Elevator(s)",

            AssociationAmenities::ExerciseCourse => "Exercise Course",

            AssociationAmenities::FitnessCenter => "Fitness Center",

            AssociationAmenities::GameCourtExterior => "Game Court Exterior",

            AssociationAmenities::GameCourtInterior => "Game Court Interior",

            AssociationAmenities::GameRoom => "Game Room",

            AssociationAmenities::Gas => "Gas",

            AssociationAmenities::Gated => "Gated",

            AssociationAmenities::GolfCourse => "Golf Course",

            AssociationAmenities::HotWater => "Hot Water",

            AssociationAmenities::IndoorPool => "Indoor Pool",

            AssociationAmenities::Insurance => "Insurance",

            AssociationAmenities::JoggingPath => "Jogging Path",

            AssociationAmenities::Landscaping => "Landscaping",

            AssociationAmenities::Laundry => "Laundry",

            AssociationAmenities::Maidservice => "Maid service",

            AssociationAmenities::Maintenance => "Maintenance",

            AssociationAmenities::MaintenanceGrounds => "Maintenance Grounds",

            AssociationAmenities::MaintenanceStructure => "Maintenance Structure",

            AssociationAmenities::Management => "Management",

            AssociationAmenities::Marina => "Marina",

            AssociationAmenities::MeetingRoom => "Meeting Room",

            AssociationAmenities::None => "None",

            AssociationAmenities::Other => "Other",

            AssociationAmenities::Park => "Park",

            AssociationAmenities::Parking => "Parking",

            AssociationAmenities::PartyRoom => "Party Room",

            AssociationAmenities::PicnicArea => "Picnic Area",

            AssociationAmenities::Playground => "Playground",

            AssociationAmenities::PondSeasonal => "Pond Seasonal",

            AssociationAmenities::PondYearRound => "Pond Year Round",

            AssociationAmenities::Pool => "Pool",

            AssociationAmenities::PoweredBoatsAllowed => "Powered Boats Allowed",

            AssociationAmenities::Racquetball => "Racquetball",

            AssociationAmenities::RecreationFacilities => "Recreation Facilities",

            AssociationAmenities::RecreationRoom => "Recreation Room",

            AssociationAmenities::RoofDeck => "Roof Deck",

            AssociationAmenities::RVParking => "RV Parking",

            AssociationAmenities::RVBoatStorage => "RV/Boat Storage",

            AssociationAmenities::Sauna => "Sauna",

            AssociationAmenities::Security => "Security",

            AssociationAmenities::ServiceElevators => "Service Elevator(s)",

            AssociationAmenities::ShuffleboardCourt => "Shuffleboard Court",

            AssociationAmenities::SkiAccessible => "Ski Accessible",

            AssociationAmenities::SnowRemoval => "Snow Removal",

            AssociationAmenities::SpaHotTub => "Spa/Hot Tub",

            AssociationAmenities::SportCourt => "Sport Court",

            AssociationAmenities::Stables => "Stable(s)",

            AssociationAmenities::Storage => "Storage",

            AssociationAmenities::StreamSeasonal => "Stream Seasonal",

            AssociationAmenities::StreamYearRound => "Stream Year Round",

            AssociationAmenities::Taxes => "Taxes",

            AssociationAmenities::TennisCourts => "Tennis Court(s)",

            AssociationAmenities::Trails => "Trail(s)",

            AssociationAmenities::Trash => "Trash",

            AssociationAmenities::Water => "Water",

            AssociationAmenities::WorkshopArea => "Workshop Area",

            AssociationAmenities::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for AssociationAmenities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for AssociationAmenities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_association_amenities_format {
    use super::AssociationAmenities;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<AssociationAmenities>>,
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
    ) -> Result<Option<Vec<AssociationAmenities>>, D::Error>
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
