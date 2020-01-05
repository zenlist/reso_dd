// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [ImageOf Lookups](https://ddwiki.reso.org/display/DDW17/ImageOf+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageOf {
    /// "[Aerial View](https://ddwiki.reso.org/display/DDW17/Aerial+View)": The image/photo is an areal view of the structure or property.
    AerialView,

    /// "[Atrium](https://ddwiki.reso.org/display/DDW17/Atrium)": The image/photo is of the atrium.
    Atrium,

    /// "[Attic](https://ddwiki.reso.org/display/DDW17/Attic)": The image/photo is of the attic.
    Attic,

    /// "[Back of Structure](https://ddwiki.reso.org/display/DDW17/Back+of+Structure)": The image/photo is of the back of the structure.
    BackofStructure,

    /// "[Balcony](https://ddwiki.reso.org/display/DDW17/Balcony)": The image/photo is of a balcony.
    Balcony,

    /// "[Bar](https://ddwiki.reso.org/display/DDW17/Bar)": The image/photo is of the bar.
    Bar,

    /// "[Barn](https://ddwiki.reso.org/display/DDW17/Barn)": The image/photo is of the barn.
    Barn,

    /// "[Basement](https://ddwiki.reso.org/display/DDW17/Basement)": The image/photo is of the basement.
    Basement,

    /// "[Bathroom](https://ddwiki.reso.org/display/DDW17/Bathroom)": The image/photo is of a bathroom.
    Bathroom,

    /// "[Bedroom](https://ddwiki.reso.org/display/DDW17/Bedroom)": The image/photo is of a bedroom.
    Bedroom,

    /// "[Bonus Room](https://ddwiki.reso.org/display/DDW17/Bonus+Room)": The image/photo is of the bonus room.
    BonusRoom,

    /// "[Breakfast Area](https://ddwiki.reso.org/display/DDW17/Breakfast+Area)": The image/photo is of the breakfast area.
    BreakfastArea,

    /// "[Closet](https://ddwiki.reso.org/display/DDW17/Closet)": The image/photo is of a closet.
    Closet,

    /// "[Community](https://ddwiki.reso.org/display/DDW17/Community)": The image/photo is of the community.
    Community,

    /// "[Courtyard](https://ddwiki.reso.org/display/DDW17/Courtyard)": The image/photo is of the courtyard.
    Courtyard,

    /// "[Deck](https://ddwiki.reso.org/display/DDW17/Deck)": The image/photo is of the deck.
    Deck,

    /// "[Den](https://ddwiki.reso.org/display/DDW17/Den)": The image/photo is of the den.
    Den,

    /// "[Dining Area](https://ddwiki.reso.org/display/DDW17/Dining+Area)": The image/photo is of the dining area.
    DiningArea,

    /// "[Dining Room](https://ddwiki.reso.org/display/DDW17/Dining+Room)": The image/photo is of the dining room.
    DiningRoom,

    /// "[Dock](https://ddwiki.reso.org/display/DDW17/Dock)": The image/photo is of the dock.
    Dock,

    /// "[Entry](https://ddwiki.reso.org/display/DDW17/Entry)": The image/photo is of the entry.
    Entry,

    /// "[Exercise Room](https://ddwiki.reso.org/display/DDW17/Exercise+Room)": The image/photo is of the exercise room.
    ExerciseRoom,

    /// "[Family Room](https://ddwiki.reso.org/display/DDW17/Family+Room)": The image/photo is of the family room.
    FamilyRoom,

    /// "[Fence](https://ddwiki.reso.org/display/DDW17/Fence)": The image/photo is of the fence.
    Fence,

    /// "[Fireplace](https://ddwiki.reso.org/display/DDW17/Fireplace)": The image/photo is of a fireplace.
    Fireplace,

    /// "[Floor Plan](https://ddwiki.reso.org/display/DDW17/Floor+Plan)": The image/photo is of the floor plan.
    FloorPlan,

    /// "[Front of Structure](https://ddwiki.reso.org/display/DDW17/Front+of+Structure)": The image/photo is of the front of structure.
    FrontofStructure,

    /// "[Game Room](https://ddwiki.reso.org/display/DDW17/Game+Room)": The image/photo is of the game room.
    GameRoom,

    /// "[Garage](https://ddwiki.reso.org/display/DDW17/Garage)": The image/photo is of the garage.
    Garage,

    /// "[Garden](https://ddwiki.reso.org/display/DDW17/Garden)": The image/photo is of the garden.
    Garden,

    /// "[Golf Course](https://ddwiki.reso.org/display/DDW17/Golf+Course)": The image/photo is of a golf course.
    GolfCourse,

    /// "[Great Room](https://ddwiki.reso.org/display/DDW17/Great+Room)": The image/photo is of the great room.
    GreatRoom,

    /// "[Guest Quarters](https://ddwiki.reso.org/display/DDW17/Guest+Quarters)": The image/photo is of the guest quarters.
    GuestQuarters,

    /// "[Gym](https://ddwiki.reso.org/display/DDW17/Gym)": The image/photo is of the gym.
    Gym,

    /// "[Hobby Room](https://ddwiki.reso.org/display/DDW17/Hobby+Room)": The image/photo is of the hobby room.
    HobbyRoom,

    /// "[Inlaw](https://ddwiki.reso.org/display/DDW17/Inlaw)": The image/photo is of the inlaw / mother-in-law quarters/room.
    Inlaw,

    /// "[Kitchen](https://ddwiki.reso.org/display/DDW17/Kitchen)": The image/photo is of the kitchen.
    Kitchen,

    /// "[Lake](https://ddwiki.reso.org/display/DDW17/Lake)": The image/photo is of the lake.
    Lake,

    /// "[Laundry](https://ddwiki.reso.org/display/DDW17/Laundry)": The image/photo is of the laundry.
    Laundry,

    /// "[Library](https://ddwiki.reso.org/display/DDW17/Library)": The image/photo is of the library.
    Library,

    /// "[Living Room](https://ddwiki.reso.org/display/DDW17/Living+Room)": The image/photo is of the living room.
    LivingRoom,

    /// "[Loading Dock](https://ddwiki.reso.org/display/DDW17/Loading+Dock)": The image/photo is of the loading dock.
    LoadingDock,

    /// "[Lobby](https://ddwiki.reso.org/display/DDW17/Lobby)": The image/photo is of the lobby.
    Lobby,

    /// "[Loft](https://ddwiki.reso.org/display/DDW17/Loft)": The image/photo is of the loft.
    Loft,

    /// "[Lot](https://ddwiki.reso.org/display/DDW17/Lot)": The image/photo is of the lot.
    Lot,

    /// "[Master Bathroom](https://ddwiki.reso.org/display/DDW17/Master+Bathroom)": The image/photo is of the master bathroom.
    MasterBathroom,

    /// "[Master Bedroom](https://ddwiki.reso.org/display/DDW17/Master+Bedroom)": The image/photo is of the master bedroom.
    MasterBedroom,

    /// "[Media Room](https://ddwiki.reso.org/display/DDW17/Media+Room)": The image/photo is of the media room.
    MediaRoom,

    /// "[Mud Room](https://ddwiki.reso.org/display/DDW17/Mud+Room)": The image/photo is of the mud room.
    MudRoom,

    /// "[Nursery](https://ddwiki.reso.org/display/DDW17/Nursery)": The image/photo is of the nursery.
    Nursery,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245065)": The image/photo is of the office.
    Office,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245066)": The image/photo is of a room or aspect of the property other than those listed in the ImageOf enumerations.
    Other,

    /// "[Out Buildings](https://ddwiki.reso.org/display/DDW17/Out+Buildings)": The image/photo is of an out building(s).
    OutBuildings,

    /// "[Pantry](https://ddwiki.reso.org/display/DDW17/Pantry)": The image/photo is of the pantry.
    Pantry,

    /// "[Parking](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245069)": The image/photo is of the parking.
    Parking,

    /// "[Patio](https://ddwiki.reso.org/display/DDW17/Patio)": The image/photo is of a patio.
    Patio,

    /// "[Pier](https://ddwiki.reso.org/display/DDW17/Pier)": The image/photo is of a pier.
    Pier,

    /// "[Plat Map](https://ddwiki.reso.org/display/DDW17/Plat+Map)": The image/photo is of the plat map.
    PlatMap,

    /// "[Pond](https://ddwiki.reso.org/display/DDW17/Pond)": The image/photo is of the pond.
    Pond,

    /// "[Pool](https://ddwiki.reso.org/display/DDW17/Pool)": The image/photo is of the pool.
    Pool,

    /// "[Reception](https://ddwiki.reso.org/display/DDW17/Reception)": The image/photo is of the reception.
    Reception,

    /// "[Recreation Room](https://ddwiki.reso.org/display/DDW17/Recreation+Room)": The image/photo is of the recreation room.
    RecreationRoom,

    /// "[Sauna](https://ddwiki.reso.org/display/DDW17/Sauna)": The image/photo is of the sauna.
    Sauna,

    /// "[Showroom](https://ddwiki.reso.org/display/DDW17/Showroom)": The image/photo is of the showroom.
    Showroom,

    /// "[Side of Structure](https://ddwiki.reso.org/display/DDW17/Side+of+Structure)": The image/photo is of the side of structure.
    SideofStructure,

    /// "[Sitting Room](https://ddwiki.reso.org/display/DDW17/Sitting+Room)": The image/photo is of the sitting room.
    SittingRoom,

    /// "[Spa](https://ddwiki.reso.org/display/DDW17/Spa)": The image/photo is of the spa.
    Spa,

    /// "[Stable](https://ddwiki.reso.org/display/DDW17/Stable)": The image/photo is of the stable.
    Stable,

    /// "[Storage](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245083)": The image/photo is of the storage.
    Storage,

    /// "[Studio](https://ddwiki.reso.org/display/DDW17/Studio)": The image/photo is of the studio.
    Studio,

    /// "[Study](https://ddwiki.reso.org/display/DDW17/Study)": The image/photo is of the study.
    Study,

    /// "[Sun Room](https://ddwiki.reso.org/display/DDW17/Sun+Room)": The image/photo is of the sun room.
    SunRoom,

    /// "[View](https://ddwiki.reso.org/display/DDW17/View)": The image/photo is of the view.
    View,

    /// "[Waterfront](https://ddwiki.reso.org/display/DDW17/Waterfront)": The image/photo is of the waterfront.
    Waterfront,

    /// "[Wine Cellar](https://ddwiki.reso.org/display/DDW17/Wine+Cellar)": The image/photo is of the wine cellar.
    WineCellar,

    /// "[Workshop](https://ddwiki.reso.org/display/DDW17/Workshop)": The image/photo is of the workshop.
    Workshop,

    /// "[Yard](https://ddwiki.reso.org/display/DDW17/Yard)": The image/photo is of the yard.
    Yard,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for ImageOf {
    fn from_str(s: &str) -> ImageOf {
        match s {
            "Aerial View" => ImageOf::AerialView,

            "Atrium" => ImageOf::Atrium,

            "Attic" => ImageOf::Attic,

            "Back of Structure" => ImageOf::BackofStructure,

            "Balcony" => ImageOf::Balcony,

            "Bar" => ImageOf::Bar,

            "Barn" => ImageOf::Barn,

            "Basement" => ImageOf::Basement,

            "Bathroom" => ImageOf::Bathroom,

            "Bedroom" => ImageOf::Bedroom,

            "Bonus Room" => ImageOf::BonusRoom,

            "Breakfast Area" => ImageOf::BreakfastArea,

            "Closet" => ImageOf::Closet,

            "Community" => ImageOf::Community,

            "Courtyard" => ImageOf::Courtyard,

            "Deck" => ImageOf::Deck,

            "Den" => ImageOf::Den,

            "Dining Area" => ImageOf::DiningArea,

            "Dining Room" => ImageOf::DiningRoom,

            "Dock" => ImageOf::Dock,

            "Entry" => ImageOf::Entry,

            "Exercise Room" => ImageOf::ExerciseRoom,

            "Family Room" => ImageOf::FamilyRoom,

            "Fence" => ImageOf::Fence,

            "Fireplace" => ImageOf::Fireplace,

            "Floor Plan" => ImageOf::FloorPlan,

            "Front of Structure" => ImageOf::FrontofStructure,

            "Game Room" => ImageOf::GameRoom,

            "Garage" => ImageOf::Garage,

            "Garden" => ImageOf::Garden,

            "Golf Course" => ImageOf::GolfCourse,

            "Great Room" => ImageOf::GreatRoom,

            "Guest Quarters" => ImageOf::GuestQuarters,

            "Gym" => ImageOf::Gym,

            "Hobby Room" => ImageOf::HobbyRoom,

            "Inlaw" => ImageOf::Inlaw,

            "Kitchen" => ImageOf::Kitchen,

            "Lake" => ImageOf::Lake,

            "Laundry" => ImageOf::Laundry,

            "Library" => ImageOf::Library,

            "Living Room" => ImageOf::LivingRoom,

            "Loading Dock" => ImageOf::LoadingDock,

            "Lobby" => ImageOf::Lobby,

            "Loft" => ImageOf::Loft,

            "Lot" => ImageOf::Lot,

            "Master Bathroom" => ImageOf::MasterBathroom,

            "Master Bedroom" => ImageOf::MasterBedroom,

            "Media Room" => ImageOf::MediaRoom,

            "Mud Room" => ImageOf::MudRoom,

            "Nursery" => ImageOf::Nursery,

            "Office" => ImageOf::Office,

            "Other" => ImageOf::Other,

            "Out Buildings" => ImageOf::OutBuildings,

            "Pantry" => ImageOf::Pantry,

            "Parking" => ImageOf::Parking,

            "Patio" => ImageOf::Patio,

            "Pier" => ImageOf::Pier,

            "Plat Map" => ImageOf::PlatMap,

            "Pond" => ImageOf::Pond,

            "Pool" => ImageOf::Pool,

            "Reception" => ImageOf::Reception,

            "Recreation Room" => ImageOf::RecreationRoom,

            "Sauna" => ImageOf::Sauna,

            "Showroom" => ImageOf::Showroom,

            "Side of Structure" => ImageOf::SideofStructure,

            "Sitting Room" => ImageOf::SittingRoom,

            "Spa" => ImageOf::Spa,

            "Stable" => ImageOf::Stable,

            "Storage" => ImageOf::Storage,

            "Studio" => ImageOf::Studio,

            "Study" => ImageOf::Study,

            "Sun Room" => ImageOf::SunRoom,

            "View" => ImageOf::View,

            "Waterfront" => ImageOf::Waterfront,

            "Wine Cellar" => ImageOf::WineCellar,

            "Workshop" => ImageOf::Workshop,

            "Yard" => ImageOf::Yard,

            _ => ImageOf::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> ImageOf {
        match s.as_ref() {
            "Aerial View" => ImageOf::AerialView,

            "Atrium" => ImageOf::Atrium,

            "Attic" => ImageOf::Attic,

            "Back of Structure" => ImageOf::BackofStructure,

            "Balcony" => ImageOf::Balcony,

            "Bar" => ImageOf::Bar,

            "Barn" => ImageOf::Barn,

            "Basement" => ImageOf::Basement,

            "Bathroom" => ImageOf::Bathroom,

            "Bedroom" => ImageOf::Bedroom,

            "Bonus Room" => ImageOf::BonusRoom,

            "Breakfast Area" => ImageOf::BreakfastArea,

            "Closet" => ImageOf::Closet,

            "Community" => ImageOf::Community,

            "Courtyard" => ImageOf::Courtyard,

            "Deck" => ImageOf::Deck,

            "Den" => ImageOf::Den,

            "Dining Area" => ImageOf::DiningArea,

            "Dining Room" => ImageOf::DiningRoom,

            "Dock" => ImageOf::Dock,

            "Entry" => ImageOf::Entry,

            "Exercise Room" => ImageOf::ExerciseRoom,

            "Family Room" => ImageOf::FamilyRoom,

            "Fence" => ImageOf::Fence,

            "Fireplace" => ImageOf::Fireplace,

            "Floor Plan" => ImageOf::FloorPlan,

            "Front of Structure" => ImageOf::FrontofStructure,

            "Game Room" => ImageOf::GameRoom,

            "Garage" => ImageOf::Garage,

            "Garden" => ImageOf::Garden,

            "Golf Course" => ImageOf::GolfCourse,

            "Great Room" => ImageOf::GreatRoom,

            "Guest Quarters" => ImageOf::GuestQuarters,

            "Gym" => ImageOf::Gym,

            "Hobby Room" => ImageOf::HobbyRoom,

            "Inlaw" => ImageOf::Inlaw,

            "Kitchen" => ImageOf::Kitchen,

            "Lake" => ImageOf::Lake,

            "Laundry" => ImageOf::Laundry,

            "Library" => ImageOf::Library,

            "Living Room" => ImageOf::LivingRoom,

            "Loading Dock" => ImageOf::LoadingDock,

            "Lobby" => ImageOf::Lobby,

            "Loft" => ImageOf::Loft,

            "Lot" => ImageOf::Lot,

            "Master Bathroom" => ImageOf::MasterBathroom,

            "Master Bedroom" => ImageOf::MasterBedroom,

            "Media Room" => ImageOf::MediaRoom,

            "Mud Room" => ImageOf::MudRoom,

            "Nursery" => ImageOf::Nursery,

            "Office" => ImageOf::Office,

            "Other" => ImageOf::Other,

            "Out Buildings" => ImageOf::OutBuildings,

            "Pantry" => ImageOf::Pantry,

            "Parking" => ImageOf::Parking,

            "Patio" => ImageOf::Patio,

            "Pier" => ImageOf::Pier,

            "Plat Map" => ImageOf::PlatMap,

            "Pond" => ImageOf::Pond,

            "Pool" => ImageOf::Pool,

            "Reception" => ImageOf::Reception,

            "Recreation Room" => ImageOf::RecreationRoom,

            "Sauna" => ImageOf::Sauna,

            "Showroom" => ImageOf::Showroom,

            "Side of Structure" => ImageOf::SideofStructure,

            "Sitting Room" => ImageOf::SittingRoom,

            "Spa" => ImageOf::Spa,

            "Stable" => ImageOf::Stable,

            "Storage" => ImageOf::Storage,

            "Studio" => ImageOf::Studio,

            "Study" => ImageOf::Study,

            "Sun Room" => ImageOf::SunRoom,

            "View" => ImageOf::View,

            "Waterfront" => ImageOf::Waterfront,

            "Wine Cellar" => ImageOf::WineCellar,

            "Workshop" => ImageOf::Workshop,

            "Yard" => ImageOf::Yard,

            _ => ImageOf::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ImageOf::AerialView => "Aerial View",

            ImageOf::Atrium => "Atrium",

            ImageOf::Attic => "Attic",

            ImageOf::BackofStructure => "Back of Structure",

            ImageOf::Balcony => "Balcony",

            ImageOf::Bar => "Bar",

            ImageOf::Barn => "Barn",

            ImageOf::Basement => "Basement",

            ImageOf::Bathroom => "Bathroom",

            ImageOf::Bedroom => "Bedroom",

            ImageOf::BonusRoom => "Bonus Room",

            ImageOf::BreakfastArea => "Breakfast Area",

            ImageOf::Closet => "Closet",

            ImageOf::Community => "Community",

            ImageOf::Courtyard => "Courtyard",

            ImageOf::Deck => "Deck",

            ImageOf::Den => "Den",

            ImageOf::DiningArea => "Dining Area",

            ImageOf::DiningRoom => "Dining Room",

            ImageOf::Dock => "Dock",

            ImageOf::Entry => "Entry",

            ImageOf::ExerciseRoom => "Exercise Room",

            ImageOf::FamilyRoom => "Family Room",

            ImageOf::Fence => "Fence",

            ImageOf::Fireplace => "Fireplace",

            ImageOf::FloorPlan => "Floor Plan",

            ImageOf::FrontofStructure => "Front of Structure",

            ImageOf::GameRoom => "Game Room",

            ImageOf::Garage => "Garage",

            ImageOf::Garden => "Garden",

            ImageOf::GolfCourse => "Golf Course",

            ImageOf::GreatRoom => "Great Room",

            ImageOf::GuestQuarters => "Guest Quarters",

            ImageOf::Gym => "Gym",

            ImageOf::HobbyRoom => "Hobby Room",

            ImageOf::Inlaw => "Inlaw",

            ImageOf::Kitchen => "Kitchen",

            ImageOf::Lake => "Lake",

            ImageOf::Laundry => "Laundry",

            ImageOf::Library => "Library",

            ImageOf::LivingRoom => "Living Room",

            ImageOf::LoadingDock => "Loading Dock",

            ImageOf::Lobby => "Lobby",

            ImageOf::Loft => "Loft",

            ImageOf::Lot => "Lot",

            ImageOf::MasterBathroom => "Master Bathroom",

            ImageOf::MasterBedroom => "Master Bedroom",

            ImageOf::MediaRoom => "Media Room",

            ImageOf::MudRoom => "Mud Room",

            ImageOf::Nursery => "Nursery",

            ImageOf::Office => "Office",

            ImageOf::Other => "Other",

            ImageOf::OutBuildings => "Out Buildings",

            ImageOf::Pantry => "Pantry",

            ImageOf::Parking => "Parking",

            ImageOf::Patio => "Patio",

            ImageOf::Pier => "Pier",

            ImageOf::PlatMap => "Plat Map",

            ImageOf::Pond => "Pond",

            ImageOf::Pool => "Pool",

            ImageOf::Reception => "Reception",

            ImageOf::RecreationRoom => "Recreation Room",

            ImageOf::Sauna => "Sauna",

            ImageOf::Showroom => "Showroom",

            ImageOf::SideofStructure => "Side of Structure",

            ImageOf::SittingRoom => "Sitting Room",

            ImageOf::Spa => "Spa",

            ImageOf::Stable => "Stable",

            ImageOf::Storage => "Storage",

            ImageOf::Studio => "Studio",

            ImageOf::Study => "Study",

            ImageOf::SunRoom => "Sun Room",

            ImageOf::View => "View",

            ImageOf::Waterfront => "Waterfront",

            ImageOf::WineCellar => "Wine Cellar",

            ImageOf::Workshop => "Workshop",

            ImageOf::Yard => "Yard",

            ImageOf::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            ImageOf::AerialView => "Aerial View".into(),

            ImageOf::Atrium => "Atrium".into(),

            ImageOf::Attic => "Attic".into(),

            ImageOf::BackofStructure => "Back of Structure".into(),

            ImageOf::Balcony => "Balcony".into(),

            ImageOf::Bar => "Bar".into(),

            ImageOf::Barn => "Barn".into(),

            ImageOf::Basement => "Basement".into(),

            ImageOf::Bathroom => "Bathroom".into(),

            ImageOf::Bedroom => "Bedroom".into(),

            ImageOf::BonusRoom => "Bonus Room".into(),

            ImageOf::BreakfastArea => "Breakfast Area".into(),

            ImageOf::Closet => "Closet".into(),

            ImageOf::Community => "Community".into(),

            ImageOf::Courtyard => "Courtyard".into(),

            ImageOf::Deck => "Deck".into(),

            ImageOf::Den => "Den".into(),

            ImageOf::DiningArea => "Dining Area".into(),

            ImageOf::DiningRoom => "Dining Room".into(),

            ImageOf::Dock => "Dock".into(),

            ImageOf::Entry => "Entry".into(),

            ImageOf::ExerciseRoom => "Exercise Room".into(),

            ImageOf::FamilyRoom => "Family Room".into(),

            ImageOf::Fence => "Fence".into(),

            ImageOf::Fireplace => "Fireplace".into(),

            ImageOf::FloorPlan => "Floor Plan".into(),

            ImageOf::FrontofStructure => "Front of Structure".into(),

            ImageOf::GameRoom => "Game Room".into(),

            ImageOf::Garage => "Garage".into(),

            ImageOf::Garden => "Garden".into(),

            ImageOf::GolfCourse => "Golf Course".into(),

            ImageOf::GreatRoom => "Great Room".into(),

            ImageOf::GuestQuarters => "Guest Quarters".into(),

            ImageOf::Gym => "Gym".into(),

            ImageOf::HobbyRoom => "Hobby Room".into(),

            ImageOf::Inlaw => "Inlaw".into(),

            ImageOf::Kitchen => "Kitchen".into(),

            ImageOf::Lake => "Lake".into(),

            ImageOf::Laundry => "Laundry".into(),

            ImageOf::Library => "Library".into(),

            ImageOf::LivingRoom => "Living Room".into(),

            ImageOf::LoadingDock => "Loading Dock".into(),

            ImageOf::Lobby => "Lobby".into(),

            ImageOf::Loft => "Loft".into(),

            ImageOf::Lot => "Lot".into(),

            ImageOf::MasterBathroom => "Master Bathroom".into(),

            ImageOf::MasterBedroom => "Master Bedroom".into(),

            ImageOf::MediaRoom => "Media Room".into(),

            ImageOf::MudRoom => "Mud Room".into(),

            ImageOf::Nursery => "Nursery".into(),

            ImageOf::Office => "Office".into(),

            ImageOf::Other => "Other".into(),

            ImageOf::OutBuildings => "Out Buildings".into(),

            ImageOf::Pantry => "Pantry".into(),

            ImageOf::Parking => "Parking".into(),

            ImageOf::Patio => "Patio".into(),

            ImageOf::Pier => "Pier".into(),

            ImageOf::PlatMap => "Plat Map".into(),

            ImageOf::Pond => "Pond".into(),

            ImageOf::Pool => "Pool".into(),

            ImageOf::Reception => "Reception".into(),

            ImageOf::RecreationRoom => "Recreation Room".into(),

            ImageOf::Sauna => "Sauna".into(),

            ImageOf::Showroom => "Showroom".into(),

            ImageOf::SideofStructure => "Side of Structure".into(),

            ImageOf::SittingRoom => "Sitting Room".into(),

            ImageOf::Spa => "Spa".into(),

            ImageOf::Stable => "Stable".into(),

            ImageOf::Storage => "Storage".into(),

            ImageOf::Studio => "Studio".into(),

            ImageOf::Study => "Study".into(),

            ImageOf::SunRoom => "Sun Room".into(),

            ImageOf::View => "View".into(),

            ImageOf::Waterfront => "Waterfront".into(),

            ImageOf::WineCellar => "Wine Cellar".into(),

            ImageOf::Workshop => "Workshop".into(),

            ImageOf::Yard => "Yard".into(),

            ImageOf::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            ImageOf::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for ImageOf {
    fn from(s: String) -> ImageOf {
        match s.as_ref() {
            "Aerial View" => ImageOf::AerialView,

            "Atrium" => ImageOf::Atrium,

            "Attic" => ImageOf::Attic,

            "Back of Structure" => ImageOf::BackofStructure,

            "Balcony" => ImageOf::Balcony,

            "Bar" => ImageOf::Bar,

            "Barn" => ImageOf::Barn,

            "Basement" => ImageOf::Basement,

            "Bathroom" => ImageOf::Bathroom,

            "Bedroom" => ImageOf::Bedroom,

            "Bonus Room" => ImageOf::BonusRoom,

            "Breakfast Area" => ImageOf::BreakfastArea,

            "Closet" => ImageOf::Closet,

            "Community" => ImageOf::Community,

            "Courtyard" => ImageOf::Courtyard,

            "Deck" => ImageOf::Deck,

            "Den" => ImageOf::Den,

            "Dining Area" => ImageOf::DiningArea,

            "Dining Room" => ImageOf::DiningRoom,

            "Dock" => ImageOf::Dock,

            "Entry" => ImageOf::Entry,

            "Exercise Room" => ImageOf::ExerciseRoom,

            "Family Room" => ImageOf::FamilyRoom,

            "Fence" => ImageOf::Fence,

            "Fireplace" => ImageOf::Fireplace,

            "Floor Plan" => ImageOf::FloorPlan,

            "Front of Structure" => ImageOf::FrontofStructure,

            "Game Room" => ImageOf::GameRoom,

            "Garage" => ImageOf::Garage,

            "Garden" => ImageOf::Garden,

            "Golf Course" => ImageOf::GolfCourse,

            "Great Room" => ImageOf::GreatRoom,

            "Guest Quarters" => ImageOf::GuestQuarters,

            "Gym" => ImageOf::Gym,

            "Hobby Room" => ImageOf::HobbyRoom,

            "Inlaw" => ImageOf::Inlaw,

            "Kitchen" => ImageOf::Kitchen,

            "Lake" => ImageOf::Lake,

            "Laundry" => ImageOf::Laundry,

            "Library" => ImageOf::Library,

            "Living Room" => ImageOf::LivingRoom,

            "Loading Dock" => ImageOf::LoadingDock,

            "Lobby" => ImageOf::Lobby,

            "Loft" => ImageOf::Loft,

            "Lot" => ImageOf::Lot,

            "Master Bathroom" => ImageOf::MasterBathroom,

            "Master Bedroom" => ImageOf::MasterBedroom,

            "Media Room" => ImageOf::MediaRoom,

            "Mud Room" => ImageOf::MudRoom,

            "Nursery" => ImageOf::Nursery,

            "Office" => ImageOf::Office,

            "Other" => ImageOf::Other,

            "Out Buildings" => ImageOf::OutBuildings,

            "Pantry" => ImageOf::Pantry,

            "Parking" => ImageOf::Parking,

            "Patio" => ImageOf::Patio,

            "Pier" => ImageOf::Pier,

            "Plat Map" => ImageOf::PlatMap,

            "Pond" => ImageOf::Pond,

            "Pool" => ImageOf::Pool,

            "Reception" => ImageOf::Reception,

            "Recreation Room" => ImageOf::RecreationRoom,

            "Sauna" => ImageOf::Sauna,

            "Showroom" => ImageOf::Showroom,

            "Side of Structure" => ImageOf::SideofStructure,

            "Sitting Room" => ImageOf::SittingRoom,

            "Spa" => ImageOf::Spa,

            "Stable" => ImageOf::Stable,

            "Storage" => ImageOf::Storage,

            "Studio" => ImageOf::Studio,

            "Study" => ImageOf::Study,

            "Sun Room" => ImageOf::SunRoom,

            "View" => ImageOf::View,

            "Waterfront" => ImageOf::Waterfront,

            "Wine Cellar" => ImageOf::WineCellar,

            "Workshop" => ImageOf::Workshop,

            "Yard" => ImageOf::Yard,

            _ => ImageOf::OpenEnumeration(s),
        }
    }
}

impl From<&str> for ImageOf {
    fn from(s: &str) -> ImageOf {
        match s {
            "Aerial View" => ImageOf::AerialView,

            "Atrium" => ImageOf::Atrium,

            "Attic" => ImageOf::Attic,

            "Back of Structure" => ImageOf::BackofStructure,

            "Balcony" => ImageOf::Balcony,

            "Bar" => ImageOf::Bar,

            "Barn" => ImageOf::Barn,

            "Basement" => ImageOf::Basement,

            "Bathroom" => ImageOf::Bathroom,

            "Bedroom" => ImageOf::Bedroom,

            "Bonus Room" => ImageOf::BonusRoom,

            "Breakfast Area" => ImageOf::BreakfastArea,

            "Closet" => ImageOf::Closet,

            "Community" => ImageOf::Community,

            "Courtyard" => ImageOf::Courtyard,

            "Deck" => ImageOf::Deck,

            "Den" => ImageOf::Den,

            "Dining Area" => ImageOf::DiningArea,

            "Dining Room" => ImageOf::DiningRoom,

            "Dock" => ImageOf::Dock,

            "Entry" => ImageOf::Entry,

            "Exercise Room" => ImageOf::ExerciseRoom,

            "Family Room" => ImageOf::FamilyRoom,

            "Fence" => ImageOf::Fence,

            "Fireplace" => ImageOf::Fireplace,

            "Floor Plan" => ImageOf::FloorPlan,

            "Front of Structure" => ImageOf::FrontofStructure,

            "Game Room" => ImageOf::GameRoom,

            "Garage" => ImageOf::Garage,

            "Garden" => ImageOf::Garden,

            "Golf Course" => ImageOf::GolfCourse,

            "Great Room" => ImageOf::GreatRoom,

            "Guest Quarters" => ImageOf::GuestQuarters,

            "Gym" => ImageOf::Gym,

            "Hobby Room" => ImageOf::HobbyRoom,

            "Inlaw" => ImageOf::Inlaw,

            "Kitchen" => ImageOf::Kitchen,

            "Lake" => ImageOf::Lake,

            "Laundry" => ImageOf::Laundry,

            "Library" => ImageOf::Library,

            "Living Room" => ImageOf::LivingRoom,

            "Loading Dock" => ImageOf::LoadingDock,

            "Lobby" => ImageOf::Lobby,

            "Loft" => ImageOf::Loft,

            "Lot" => ImageOf::Lot,

            "Master Bathroom" => ImageOf::MasterBathroom,

            "Master Bedroom" => ImageOf::MasterBedroom,

            "Media Room" => ImageOf::MediaRoom,

            "Mud Room" => ImageOf::MudRoom,

            "Nursery" => ImageOf::Nursery,

            "Office" => ImageOf::Office,

            "Other" => ImageOf::Other,

            "Out Buildings" => ImageOf::OutBuildings,

            "Pantry" => ImageOf::Pantry,

            "Parking" => ImageOf::Parking,

            "Patio" => ImageOf::Patio,

            "Pier" => ImageOf::Pier,

            "Plat Map" => ImageOf::PlatMap,

            "Pond" => ImageOf::Pond,

            "Pool" => ImageOf::Pool,

            "Reception" => ImageOf::Reception,

            "Recreation Room" => ImageOf::RecreationRoom,

            "Sauna" => ImageOf::Sauna,

            "Showroom" => ImageOf::Showroom,

            "Side of Structure" => ImageOf::SideofStructure,

            "Sitting Room" => ImageOf::SittingRoom,

            "Spa" => ImageOf::Spa,

            "Stable" => ImageOf::Stable,

            "Storage" => ImageOf::Storage,

            "Studio" => ImageOf::Studio,

            "Study" => ImageOf::Study,

            "Sun Room" => ImageOf::SunRoom,

            "View" => ImageOf::View,

            "Waterfront" => ImageOf::Waterfront,

            "Wine Cellar" => ImageOf::WineCellar,

            "Workshop" => ImageOf::Workshop,

            "Yard" => ImageOf::Yard,

            _ => ImageOf::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a ImageOf> for &'a str {
    fn from(s: &'a ImageOf) -> &'a str {
        match s {
            ImageOf::AerialView => "Aerial View",

            ImageOf::Atrium => "Atrium",

            ImageOf::Attic => "Attic",

            ImageOf::BackofStructure => "Back of Structure",

            ImageOf::Balcony => "Balcony",

            ImageOf::Bar => "Bar",

            ImageOf::Barn => "Barn",

            ImageOf::Basement => "Basement",

            ImageOf::Bathroom => "Bathroom",

            ImageOf::Bedroom => "Bedroom",

            ImageOf::BonusRoom => "Bonus Room",

            ImageOf::BreakfastArea => "Breakfast Area",

            ImageOf::Closet => "Closet",

            ImageOf::Community => "Community",

            ImageOf::Courtyard => "Courtyard",

            ImageOf::Deck => "Deck",

            ImageOf::Den => "Den",

            ImageOf::DiningArea => "Dining Area",

            ImageOf::DiningRoom => "Dining Room",

            ImageOf::Dock => "Dock",

            ImageOf::Entry => "Entry",

            ImageOf::ExerciseRoom => "Exercise Room",

            ImageOf::FamilyRoom => "Family Room",

            ImageOf::Fence => "Fence",

            ImageOf::Fireplace => "Fireplace",

            ImageOf::FloorPlan => "Floor Plan",

            ImageOf::FrontofStructure => "Front of Structure",

            ImageOf::GameRoom => "Game Room",

            ImageOf::Garage => "Garage",

            ImageOf::Garden => "Garden",

            ImageOf::GolfCourse => "Golf Course",

            ImageOf::GreatRoom => "Great Room",

            ImageOf::GuestQuarters => "Guest Quarters",

            ImageOf::Gym => "Gym",

            ImageOf::HobbyRoom => "Hobby Room",

            ImageOf::Inlaw => "Inlaw",

            ImageOf::Kitchen => "Kitchen",

            ImageOf::Lake => "Lake",

            ImageOf::Laundry => "Laundry",

            ImageOf::Library => "Library",

            ImageOf::LivingRoom => "Living Room",

            ImageOf::LoadingDock => "Loading Dock",

            ImageOf::Lobby => "Lobby",

            ImageOf::Loft => "Loft",

            ImageOf::Lot => "Lot",

            ImageOf::MasterBathroom => "Master Bathroom",

            ImageOf::MasterBedroom => "Master Bedroom",

            ImageOf::MediaRoom => "Media Room",

            ImageOf::MudRoom => "Mud Room",

            ImageOf::Nursery => "Nursery",

            ImageOf::Office => "Office",

            ImageOf::Other => "Other",

            ImageOf::OutBuildings => "Out Buildings",

            ImageOf::Pantry => "Pantry",

            ImageOf::Parking => "Parking",

            ImageOf::Patio => "Patio",

            ImageOf::Pier => "Pier",

            ImageOf::PlatMap => "Plat Map",

            ImageOf::Pond => "Pond",

            ImageOf::Pool => "Pool",

            ImageOf::Reception => "Reception",

            ImageOf::RecreationRoom => "Recreation Room",

            ImageOf::Sauna => "Sauna",

            ImageOf::Showroom => "Showroom",

            ImageOf::SideofStructure => "Side of Structure",

            ImageOf::SittingRoom => "Sitting Room",

            ImageOf::Spa => "Spa",

            ImageOf::Stable => "Stable",

            ImageOf::Storage => "Storage",

            ImageOf::Studio => "Studio",

            ImageOf::Study => "Study",

            ImageOf::SunRoom => "Sun Room",

            ImageOf::View => "View",

            ImageOf::Waterfront => "Waterfront",

            ImageOf::WineCellar => "Wine Cellar",

            ImageOf::Workshop => "Workshop",

            ImageOf::Yard => "Yard",

            ImageOf::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for ImageOf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ImageOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
