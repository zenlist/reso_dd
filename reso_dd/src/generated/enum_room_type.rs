// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [RoomType Lookups](https://ddwiki.reso.org/display/DDW17/RoomType+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoomType {
    /// "[Basement](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246147)": A floor of a building below ground level.
    Basement,

    /// "[Bathroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246179)": The first bathroom, when a Master Bathroom is not designated.
    Bathroom,

    /// "[Bathroom 1](https://ddwiki.reso.org/display/DDW17/Bathroom+1)": The first bathroom, when a Master Bathroom is not designated.
    Bathroom1,

    /// "[Bathroom 2](https://ddwiki.reso.org/display/DDW17/Bathroom+2)": The second bathroom.
    Bathroom2,

    /// "[Bathroom 3](https://ddwiki.reso.org/display/DDW17/Bathroom+3)": The third bathroom.
    Bathroom3,

    /// "[Bathroom 4](https://ddwiki.reso.org/display/DDW17/Bathroom+4)": The fourth bathroom.
    Bathroom4,

    /// "[Bathroom 5](https://ddwiki.reso.org/display/DDW17/Bathroom+5)": The fifth bathroom.
    Bathroom5,

    /// "[Bedroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246178)": The type of room is a bedroom.
    Bedroom,

    /// "[Bedroom 1](https://ddwiki.reso.org/display/DDW17/Bedroom+1)": The first bedroom, when a Master Bedroom is not designated.
    Bedroom1,

    /// "[Bedroom 2](https://ddwiki.reso.org/display/DDW17/Bedroom+2)": The second bedroom.
    Bedroom2,

    /// "[Bedroom 3](https://ddwiki.reso.org/display/DDW17/Bedroom+3)": The third bedroom.
    Bedroom3,

    /// "[Bedroom 4](https://ddwiki.reso.org/display/DDW17/Bedroom+4)": The fourth bedroom.
    Bedroom4,

    /// "[Bedroom 5](https://ddwiki.reso.org/display/DDW17/Bedroom+5)": The fifth bedroom.
    Bedroom5,

    /// "[Bonus Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246158)": A room that can be used for multiple purposes.
    BonusRoom,

    /// "[Den](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246159)": Used for a variety of purposes, a den is typically a secluded comfortable room use as a study or entertainment room.
    Den,

    /// "[Dining Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246160)": A room, in a home, where meals are eaten.
    DiningRoom,

    /// "[Exercise Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246161)": A room that is specifically geared to contain exercise equipment.
    ExerciseRoom,

    /// "[Family Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246162)": A comfortable room in a dwelling, for frequent leisure use.
    FamilyRoom,

    /// "[Game Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246163)": A room that, typically a bonus room, that is specifically equipped for game play.  This may include billiards, Ping-Pong, video games, board games or other recreational activities.
    GameRoom,

    /// "[Great Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246164)": The term great room denotes a room space within an abode which combines the specific functions of several of the more traditional room spaces (e.g. the family room, the living room, the study, etc.) into a singular unified space. Great rooms are typically at or near the center of the house, feature raised ceilings, and have been common in American homes since the early 1990s.
    GreatRoom,

    /// "[Gym](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246165)": A room that, in addition to exercise equipment, has other characteristics of a gymnasium.
    Gym,

    /// "[Kitchen](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246166)": The room used for the preparation and storage of food. Cookery.
    Kitchen,

    /// "[Laundry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246167)": A utility room specifically equipment and used for laundry equipment (washer and dryer).
    Laundry,

    /// "[Library](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246168)": A room that is specifically geared to house books and other media typically found in a library.
    Library,

    /// "[Living Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246169)": A room in a private house used for general social and leisure activities.
    LivingRoom,

    /// "[Loft](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246170)": A loft can be an upper story or attic in a building, directly under the roof. Alternatively, a loft apartment refers to large adaptable open space, often converted for residential use (a converted loft) from some other use, often light industrial. Adding to the confusion, some converted lofts include upper open loft areas.
    Loft,

    /// "[Master Bathroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246171)": Typically the largest of the bathrooms and attached to the master bedroom.
    MasterBathroom,

    /// "[Master Bedroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246172)": Typically the largest of the bedrooms with an attached bathroom.
    MasterBedroom,

    /// "[Media Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246173)": A room that is specifically geared for the watching of movies, TV or other forms of multimedia.
    MediaRoom,

    /// "[Office](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246174)": A room used for business.
    Office,

    /// "[Sauna](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246175)": A small room or house designed as a place to experience dry or wet heat sessions, or an establishment with one or more of these and auxiliary facilities.
    Sauna,

    /// "[Utility Room](https://ddwiki.reso.org/display/DDW17/Utility+Room)": A room that usually contains laundry, HVAC, water heating or some other utilitarian equipment.  In some areas this is simply the laundry room.  In other areas it may be used for many other purposes, all having some utility.
    UtilityRoom,

    /// "[Workshop](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246177)": A room containing tools or equipment used for the manufacturing or repair of goods.
    Workshop,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for RoomType {
    fn from(s: String) -> RoomType {
        match s.as_ref() {
            "Basement" => RoomType::Basement,

            "Bathroom" => RoomType::Bathroom,

            "Bathroom 1" => RoomType::Bathroom1,

            "Bathroom 2" => RoomType::Bathroom2,

            "Bathroom 3" => RoomType::Bathroom3,

            "Bathroom 4" => RoomType::Bathroom4,

            "Bathroom 5" => RoomType::Bathroom5,

            "Bedroom" => RoomType::Bedroom,

            "Bedroom 1" => RoomType::Bedroom1,

            "Bedroom 2" => RoomType::Bedroom2,

            "Bedroom 3" => RoomType::Bedroom3,

            "Bedroom 4" => RoomType::Bedroom4,

            "Bedroom 5" => RoomType::Bedroom5,

            "Bonus Room" => RoomType::BonusRoom,

            "Den" => RoomType::Den,

            "Dining Room" => RoomType::DiningRoom,

            "Exercise Room" => RoomType::ExerciseRoom,

            "Family Room" => RoomType::FamilyRoom,

            "Game Room" => RoomType::GameRoom,

            "Great Room" => RoomType::GreatRoom,

            "Gym" => RoomType::Gym,

            "Kitchen" => RoomType::Kitchen,

            "Laundry" => RoomType::Laundry,

            "Library" => RoomType::Library,

            "Living Room" => RoomType::LivingRoom,

            "Loft" => RoomType::Loft,

            "Master Bathroom" => RoomType::MasterBathroom,

            "Master Bedroom" => RoomType::MasterBedroom,

            "Media Room" => RoomType::MediaRoom,

            "Office" => RoomType::Office,

            "Sauna" => RoomType::Sauna,

            "Utility Room" => RoomType::UtilityRoom,

            "Workshop" => RoomType::Workshop,

            _ => RoomType::OpenEnumeration(s),
        }
    }
}

impl From<&str> for RoomType {
    fn from(s: &str) -> RoomType {
        match s {
            "Basement" => RoomType::Basement,

            "Bathroom" => RoomType::Bathroom,

            "Bathroom 1" => RoomType::Bathroom1,

            "Bathroom 2" => RoomType::Bathroom2,

            "Bathroom 3" => RoomType::Bathroom3,

            "Bathroom 4" => RoomType::Bathroom4,

            "Bathroom 5" => RoomType::Bathroom5,

            "Bedroom" => RoomType::Bedroom,

            "Bedroom 1" => RoomType::Bedroom1,

            "Bedroom 2" => RoomType::Bedroom2,

            "Bedroom 3" => RoomType::Bedroom3,

            "Bedroom 4" => RoomType::Bedroom4,

            "Bedroom 5" => RoomType::Bedroom5,

            "Bonus Room" => RoomType::BonusRoom,

            "Den" => RoomType::Den,

            "Dining Room" => RoomType::DiningRoom,

            "Exercise Room" => RoomType::ExerciseRoom,

            "Family Room" => RoomType::FamilyRoom,

            "Game Room" => RoomType::GameRoom,

            "Great Room" => RoomType::GreatRoom,

            "Gym" => RoomType::Gym,

            "Kitchen" => RoomType::Kitchen,

            "Laundry" => RoomType::Laundry,

            "Library" => RoomType::Library,

            "Living Room" => RoomType::LivingRoom,

            "Loft" => RoomType::Loft,

            "Master Bathroom" => RoomType::MasterBathroom,

            "Master Bedroom" => RoomType::MasterBedroom,

            "Media Room" => RoomType::MediaRoom,

            "Office" => RoomType::Office,

            "Sauna" => RoomType::Sauna,

            "Utility Room" => RoomType::UtilityRoom,

            "Workshop" => RoomType::Workshop,

            _ => RoomType::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a RoomType> for &'a str {
    fn from(s: &'a RoomType) -> &'a str {
        match s {
            RoomType::Basement => "Basement",

            RoomType::Bathroom => "Bathroom",

            RoomType::Bathroom1 => "Bathroom 1",

            RoomType::Bathroom2 => "Bathroom 2",

            RoomType::Bathroom3 => "Bathroom 3",

            RoomType::Bathroom4 => "Bathroom 4",

            RoomType::Bathroom5 => "Bathroom 5",

            RoomType::Bedroom => "Bedroom",

            RoomType::Bedroom1 => "Bedroom 1",

            RoomType::Bedroom2 => "Bedroom 2",

            RoomType::Bedroom3 => "Bedroom 3",

            RoomType::Bedroom4 => "Bedroom 4",

            RoomType::Bedroom5 => "Bedroom 5",

            RoomType::BonusRoom => "Bonus Room",

            RoomType::Den => "Den",

            RoomType::DiningRoom => "Dining Room",

            RoomType::ExerciseRoom => "Exercise Room",

            RoomType::FamilyRoom => "Family Room",

            RoomType::GameRoom => "Game Room",

            RoomType::GreatRoom => "Great Room",

            RoomType::Gym => "Gym",

            RoomType::Kitchen => "Kitchen",

            RoomType::Laundry => "Laundry",

            RoomType::Library => "Library",

            RoomType::LivingRoom => "Living Room",

            RoomType::Loft => "Loft",

            RoomType::MasterBathroom => "Master Bathroom",

            RoomType::MasterBedroom => "Master Bedroom",

            RoomType::MediaRoom => "Media Room",

            RoomType::Office => "Office",

            RoomType::Sauna => "Sauna",

            RoomType::UtilityRoom => "Utility Room",

            RoomType::Workshop => "Workshop",

            RoomType::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for RoomType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RoomType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_room_type_format {
    use super::RoomType;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<RoomType>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<RoomType>>, D::Error>
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
