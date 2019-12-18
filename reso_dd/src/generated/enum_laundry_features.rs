// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [LaundryFeatures Lookups](https://ddwiki.reso.org/display/DDW17/LaundryFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LaundryFeatures {
    /// "[Common Area](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245476)": Laundry facilities are in a common area.
    CommonArea,

    /// "[Electric Dryer Hookup](https://ddwiki.reso.org/display/DDW17/Electric+Dryer+Hookup)": The property has electric cloths dryer connections.
    ElectricDryerHookup,

    /// "[Gas Dryer Hookup](https://ddwiki.reso.org/display/DDW17/Gas+Dryer+Hookup)": The property has gas cloths dryer connections.
    GasDryerHookup,

    /// "[In Basement](https://ddwiki.reso.org/display/DDW17/In+Basement)": Laundry is located in the basement.
    InBasement,

    /// "[In Bathroom](https://ddwiki.reso.org/display/DDW17/In+Bathroom)": Laundry is located in the bathroom.
    InBathroom,

    /// "[In Carport](https://ddwiki.reso.org/display/DDW17/In+Carport)": Laundry is located in the carport.
    InCarport,

    /// "[In Garage](https://ddwiki.reso.org/display/DDW17/In+Garage)": Laundry is located in the garage.
    InGarage,

    /// "[In Hall](https://ddwiki.reso.org/display/DDW17/In+Hall)": Laundry is located in the hall.
    InHall,

    /// "[In Kitchen](https://ddwiki.reso.org/display/DDW17/In+Kitchen)": Laundry is located in the kitchen.
    InKitchen,

    /// "[In Unit](https://ddwiki.reso.org/display/DDW17/In+Unit)": Laundry is located within the unit.
    InUnit,

    /// "[Inside](https://ddwiki.reso.org/display/DDW17/Inside)": Laundry is located indoors.
    Inside,

    /// "[Laundry Chute](https://ddwiki.reso.org/display/DDW17/Laundry+Chute)": The property has a laundry chute.
    LaundryChute,

    /// "[Laundry Closet](https://ddwiki.reso.org/display/DDW17/Laundry+Closet)": The property has a laundry closet.
    LaundryCloset,

    /// "[Laundry Room](https://ddwiki.reso.org/display/DDW17/Laundry+Room)": The property has a laundry room.
    LaundryRoom,

    /// "[Lower Level](https://ddwiki.reso.org/display/DDW17/Lower+Level)": Laundry is on the lower level.
    LowerLevel,

    /// "[Main Level](https://ddwiki.reso.org/display/DDW17/Main+Level)": Laundry is on the main level.
    MainLevel,

    /// "[Multiple Locations](https://ddwiki.reso.org/display/DDW17/Multiple+Locations)": Laundry is in multiple locations.
    MultipleLocations,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245493)": There are no laundry features.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245494)": There are laundry features other than those in this list.
    Other,

    /// "[Outside](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245495)": Laundry is located outside.
    Outside,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245496)": See remarks for additional information about laundry.
    SeeRemarks,

    /// "[Sink](https://ddwiki.reso.org/display/DDW17/Sink)": The laundry area has a sink.
    Sink,

    /// "[Upper Level](https://ddwiki.reso.org/display/DDW17/Upper+Level)": Laundry is on the upper level.
    UpperLevel,

    /// "[Washer Hookup](https://ddwiki.reso.org/display/DDW17/Washer+Hookup)": The property has a hookups for a cloths washer.
    WasherHookup,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for LaundryFeatures {
    fn from(s: String) -> LaundryFeatures {
        match s.as_ref() {
            "Common Area" => LaundryFeatures::CommonArea,

            "Electric Dryer Hookup" => LaundryFeatures::ElectricDryerHookup,

            "Gas Dryer Hookup" => LaundryFeatures::GasDryerHookup,

            "In Basement" => LaundryFeatures::InBasement,

            "In Bathroom" => LaundryFeatures::InBathroom,

            "In Carport" => LaundryFeatures::InCarport,

            "In Garage" => LaundryFeatures::InGarage,

            "In Hall" => LaundryFeatures::InHall,

            "In Kitchen" => LaundryFeatures::InKitchen,

            "In Unit" => LaundryFeatures::InUnit,

            "Inside" => LaundryFeatures::Inside,

            "Laundry Chute" => LaundryFeatures::LaundryChute,

            "Laundry Closet" => LaundryFeatures::LaundryCloset,

            "Laundry Room" => LaundryFeatures::LaundryRoom,

            "Lower Level" => LaundryFeatures::LowerLevel,

            "Main Level" => LaundryFeatures::MainLevel,

            "Multiple Locations" => LaundryFeatures::MultipleLocations,

            "None" => LaundryFeatures::None,

            "Other" => LaundryFeatures::Other,

            "Outside" => LaundryFeatures::Outside,

            "See Remarks" => LaundryFeatures::SeeRemarks,

            "Sink" => LaundryFeatures::Sink,

            "Upper Level" => LaundryFeatures::UpperLevel,

            "Washer Hookup" => LaundryFeatures::WasherHookup,

            _ => LaundryFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for LaundryFeatures {
    fn from(s: &str) -> LaundryFeatures {
        match s {
            "Common Area" => LaundryFeatures::CommonArea,

            "Electric Dryer Hookup" => LaundryFeatures::ElectricDryerHookup,

            "Gas Dryer Hookup" => LaundryFeatures::GasDryerHookup,

            "In Basement" => LaundryFeatures::InBasement,

            "In Bathroom" => LaundryFeatures::InBathroom,

            "In Carport" => LaundryFeatures::InCarport,

            "In Garage" => LaundryFeatures::InGarage,

            "In Hall" => LaundryFeatures::InHall,

            "In Kitchen" => LaundryFeatures::InKitchen,

            "In Unit" => LaundryFeatures::InUnit,

            "Inside" => LaundryFeatures::Inside,

            "Laundry Chute" => LaundryFeatures::LaundryChute,

            "Laundry Closet" => LaundryFeatures::LaundryCloset,

            "Laundry Room" => LaundryFeatures::LaundryRoom,

            "Lower Level" => LaundryFeatures::LowerLevel,

            "Main Level" => LaundryFeatures::MainLevel,

            "Multiple Locations" => LaundryFeatures::MultipleLocations,

            "None" => LaundryFeatures::None,

            "Other" => LaundryFeatures::Other,

            "Outside" => LaundryFeatures::Outside,

            "See Remarks" => LaundryFeatures::SeeRemarks,

            "Sink" => LaundryFeatures::Sink,

            "Upper Level" => LaundryFeatures::UpperLevel,

            "Washer Hookup" => LaundryFeatures::WasherHookup,

            _ => LaundryFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a LaundryFeatures> for &'a str {
    fn from(s: &'a LaundryFeatures) -> &'a str {
        match s {
            LaundryFeatures::CommonArea => "Common Area",

            LaundryFeatures::ElectricDryerHookup => "Electric Dryer Hookup",

            LaundryFeatures::GasDryerHookup => "Gas Dryer Hookup",

            LaundryFeatures::InBasement => "In Basement",

            LaundryFeatures::InBathroom => "In Bathroom",

            LaundryFeatures::InCarport => "In Carport",

            LaundryFeatures::InGarage => "In Garage",

            LaundryFeatures::InHall => "In Hall",

            LaundryFeatures::InKitchen => "In Kitchen",

            LaundryFeatures::InUnit => "In Unit",

            LaundryFeatures::Inside => "Inside",

            LaundryFeatures::LaundryChute => "Laundry Chute",

            LaundryFeatures::LaundryCloset => "Laundry Closet",

            LaundryFeatures::LaundryRoom => "Laundry Room",

            LaundryFeatures::LowerLevel => "Lower Level",

            LaundryFeatures::MainLevel => "Main Level",

            LaundryFeatures::MultipleLocations => "Multiple Locations",

            LaundryFeatures::None => "None",

            LaundryFeatures::Other => "Other",

            LaundryFeatures::Outside => "Outside",

            LaundryFeatures::SeeRemarks => "See Remarks",

            LaundryFeatures::Sink => "Sink",

            LaundryFeatures::UpperLevel => "Upper Level",

            LaundryFeatures::WasherHookup => "Washer Hookup",

            LaundryFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for LaundryFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for LaundryFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_laundry_features_format {
    use super::LaundryFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<LaundryFeatures>>,
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
    ) -> Result<Option<Vec<LaundryFeatures>>, D::Error>
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
