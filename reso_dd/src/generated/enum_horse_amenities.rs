// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [HorseAmenities Lookups](https://ddwiki.reso.org/display/DDW17/HorseAmenities+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HorseAmenities {
    /// "[Arena](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244995)": The property allows for horses and has an arena.
    Arena,

    /// "[Barn](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244996)": The property allows horses and has a barn.
    Barn,

    /// "[Boarding Facilities](https://ddwiki.reso.org/display/DDW17/Boarding+Facilities)": The property had horse boarding facilities.
    BoardingFacilities,

    /// "[Corral(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244998)": The property allows horses and has one or more corrals.
    Corrals,

    /// "[Hay Storage](https://ddwiki.reso.org/display/DDW17/Hay+Storage)": The property allows horses and has hay storage.
    HayStorage,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245000)": The property either does not allow horses or has no amenities for horses.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245001)": The property has horse amenities other than those in this list.
    Other,

    /// "[Paddocks](https://ddwiki.reso.org/display/DDW17/Paddocks)": The property allows horses and has an enclosed living are for your horse(s).  A paddock is also known as a sacrifice area which got its name because the owner was sacrificing some of their land for the benefit of the horse.
    Paddocks,

    /// "[Palpation Chute](https://ddwiki.reso.org/display/DDW17/Palpation+Chute)": A portion of the livestock chute where the animal is held for examination or other purposes.
    PalpationChute,

    /// "[Pasture](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245004)": The property includes or has access to a pasture for horses.
    Pasture,

    /// "[Riding Trail](https://ddwiki.reso.org/display/DDW17/Riding+Trail)": The property includes or has access to a riding trail(s).
    RidingTrail,

    /// "[Round Pen](https://ddwiki.reso.org/display/DDW17/Round+Pen)": The property includes or has access to a round enclosure used for horse training.
    RoundPen,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245007)": See the remarks fields for additional information about horse amenities.
    SeeRemarks,

    /// "[Shaving Bin](https://ddwiki.reso.org/display/DDW17/Shaving+Bin)": The property includes or has access to a storage container for wood shavings that are use as ground cover for horses.
    ShavingBin,

    /// "[Stable(s)](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245009)": The property includes or has access to horse stable(s).
    Stables,

    /// "[Tack Room](https://ddwiki.reso.org/display/DDW17/Tack+Room)": The property includes or has access to a room to store equipment such as saddles, stirrups, bridles, halters, reins, bits, harnesses, martingales, breastplates, etc.
    TackRoom,

    /// "[Trailer Storage](https://ddwiki.reso.org/display/DDW17/Trailer+Storage)": The property includes or has access to a place to store a horse trailer.
    TrailerStorage,

    /// "[Wash Rack](https://ddwiki.reso.org/display/DDW17/Wash+Rack)": The property includes or has access to a rack used to confine/restrain a horse for purposes of washing the horse.
    WashRack,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for HorseAmenities {
    fn from(s: String) -> HorseAmenities {
        match s.as_ref() {
            "Arena" => HorseAmenities::Arena,

            "Barn" => HorseAmenities::Barn,

            "Boarding Facilities" => HorseAmenities::BoardingFacilities,

            "Corral(s)" => HorseAmenities::Corrals,

            "Hay Storage" => HorseAmenities::HayStorage,

            "None" => HorseAmenities::None,

            "Other" => HorseAmenities::Other,

            "Paddocks" => HorseAmenities::Paddocks,

            "Palpation Chute" => HorseAmenities::PalpationChute,

            "Pasture" => HorseAmenities::Pasture,

            "Riding Trail" => HorseAmenities::RidingTrail,

            "Round Pen" => HorseAmenities::RoundPen,

            "See Remarks" => HorseAmenities::SeeRemarks,

            "Shaving Bin" => HorseAmenities::ShavingBin,

            "Stable(s)" => HorseAmenities::Stables,

            "Tack Room" => HorseAmenities::TackRoom,

            "Trailer Storage" => HorseAmenities::TrailerStorage,

            "Wash Rack" => HorseAmenities::WashRack,

            _ => HorseAmenities::OpenEnumeration(s),
        }
    }
}

impl From<&str> for HorseAmenities {
    fn from(s: &str) -> HorseAmenities {
        match s {
            "Arena" => HorseAmenities::Arena,

            "Barn" => HorseAmenities::Barn,

            "Boarding Facilities" => HorseAmenities::BoardingFacilities,

            "Corral(s)" => HorseAmenities::Corrals,

            "Hay Storage" => HorseAmenities::HayStorage,

            "None" => HorseAmenities::None,

            "Other" => HorseAmenities::Other,

            "Paddocks" => HorseAmenities::Paddocks,

            "Palpation Chute" => HorseAmenities::PalpationChute,

            "Pasture" => HorseAmenities::Pasture,

            "Riding Trail" => HorseAmenities::RidingTrail,

            "Round Pen" => HorseAmenities::RoundPen,

            "See Remarks" => HorseAmenities::SeeRemarks,

            "Shaving Bin" => HorseAmenities::ShavingBin,

            "Stable(s)" => HorseAmenities::Stables,

            "Tack Room" => HorseAmenities::TackRoom,

            "Trailer Storage" => HorseAmenities::TrailerStorage,

            "Wash Rack" => HorseAmenities::WashRack,

            _ => HorseAmenities::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a HorseAmenities> for &'a str {
    fn from(s: &'a HorseAmenities) -> &'a str {
        match s {
            HorseAmenities::Arena => "Arena",

            HorseAmenities::Barn => "Barn",

            HorseAmenities::BoardingFacilities => "Boarding Facilities",

            HorseAmenities::Corrals => "Corral(s)",

            HorseAmenities::HayStorage => "Hay Storage",

            HorseAmenities::None => "None",

            HorseAmenities::Other => "Other",

            HorseAmenities::Paddocks => "Paddocks",

            HorseAmenities::PalpationChute => "Palpation Chute",

            HorseAmenities::Pasture => "Pasture",

            HorseAmenities::RidingTrail => "Riding Trail",

            HorseAmenities::RoundPen => "Round Pen",

            HorseAmenities::SeeRemarks => "See Remarks",

            HorseAmenities::ShavingBin => "Shaving Bin",

            HorseAmenities::Stables => "Stable(s)",

            HorseAmenities::TackRoom => "Tack Room",

            HorseAmenities::TrailerStorage => "Trailer Storage",

            HorseAmenities::WashRack => "Wash Rack",

            HorseAmenities::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for HorseAmenities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for HorseAmenities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_horse_amenities_format {
    use super::HorseAmenities;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<HorseAmenities>>,
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
    ) -> Result<Option<Vec<HorseAmenities>>, D::Error>
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
