// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [FireplaceFeatures Lookups](https://ddwiki.reso.org/display/DDW17/FireplaceFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FireplaceFeatures {
    /// "[Basement](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244790)": There is a fireplace in the basement.
    Basement,

    /// "[Bath](https://ddwiki.reso.org/display/DDW17/Bath)": The property includes a bathroom fireplace.
    Bath,

    /// "[Bedroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244792)": The property has a bedroom fireplace.
    Bedroom,

    /// "[Blower Fan](https://ddwiki.reso.org/display/DDW17/Blower+Fan)": The fireplace has a blower fan.
    BlowerFan,

    /// "[Circulating](https://ddwiki.reso.org/display/DDW17/Circulating)": The fireplace has a circulation system.
    Circulating,

    /// "[Decorative](https://ddwiki.reso.org/display/DDW17/Decorative)": The property has a decorative fireplace.
    Decorative,

    /// "[Den](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244796)": The property has a fireplace in the den.
    Den,

    /// "[Dining Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244797)": The property has a fireplace in the dining room.
    DiningRoom,

    /// "[Double Sided](https://ddwiki.reso.org/display/DDW17/Double+Sided)": The property has a double sided fireplace.  Double sided fireplaces often have openings in adjacent rooms.
    DoubleSided,

    /// "[Electric](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244799)": The fireplace is electric.
    Electric,

    /// "[EPA Certified Wood Stove](https://ddwiki.reso.org/display/DDW17/EPA+Certified+Wood+Stove)": The property has an EPA certified wood stove.
    EPACertifiedWoodStove,

    /// "[EPA Qualified Fireplace](https://ddwiki.reso.org/display/DDW17/EPA+Qualified+Fireplace)": The property has an EPA certified fireplace.
    EPAQualifiedFireplace,

    /// "[Factory Built](https://ddwiki.reso.org/display/DDW17/Factory+Built)": The fireplace is factory built and later installed into the property.
    FactoryBuilt,

    /// "[Family Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244803)": There is a fireplace in the family room.
    FamilyRoom,

    /// "[Fire Pit](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244804)": The property has a fire pit.
    FirePit,

    /// "[Free Standing](https://ddwiki.reso.org/display/DDW17/Free+Standing)": The fireplace is free standing, rather than built-in.
    FreeStanding,

    /// "[Gas](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244806)": The fireplace burns gas.
    Gas,

    /// "[Gas Log](https://ddwiki.reso.org/display/DDW17/Gas+Log)": The gas fireplace has a gas log.
    GasLog,

    /// "[Gas Starter](https://ddwiki.reso.org/display/DDW17/Gas+Starter)": The fireplace has a gas started, but also burns wood or other fuels.
    GasStarter,

    /// "[Glass Doors](https://ddwiki.reso.org/display/DDW17/Glass+Doors)": The fireplace has glass doors.
    GlassDoors,

    /// "[Great Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244810)": There is a fireplace in the great room.
    GreatRoom,

    /// "[Heatilator](https://ddwiki.reso.org/display/DDW17/Heatilator)": The fireplace has a built in ventilation system used to circulate heat.
    Heatilator,

    /// "[Insert](https://ddwiki.reso.org/display/DDW17/Insert)": A fireplace insert is a device inserted into an existing masonry or prefabricated fireplace.  Inserts are used for their aesthetic, insulating, circulating or other features.
    Insert,

    /// "[Kitchen](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244813)": The property has a fireplace in the kitchen.
    Kitchen,

    /// "[Library](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244814)": The property has a fireplace in the library.
    Library,

    /// "[Living Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244815)": The property has a fireplace in the living room.
    LivingRoom,

    /// "[Masonry](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244816)": The fireplace is made of masonry.
    Masonry,

    /// "[Master Bedroom](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244817)": The property has a fireplace in the master bedroom.
    MasterBedroom,

    /// "[Metal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244818)": The fireplace is make of metal.
    Metal,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244819)": The property has no fireplace.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244820)": The fireplace has features that are not included in this list.
    Other,

    /// "[Outside](https://ddwiki.reso.org/display/DDW17/Outside)": The property has an outdoor fireplace.
    Outside,

    /// "[Pellet Stove](https://ddwiki.reso.org/display/DDW17/Pellet+Stove)": The property has a stove that burns compressed wood or biomass pellets to generate heat.
    PelletStove,

    /// "[Propane](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244823)": The fireplace burns propane.
    Propane,

    /// "[Raised Hearth](https://ddwiki.reso.org/display/DDW17/Raised+Hearth)": The fireplace has a raised hearth.
    RaisedHearth,

    /// "[Recreation Room](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244825)": The property has a fireplace in the recreation room.
    RecreationRoom,

    /// "[Sealed Combustion](https://ddwiki.reso.org/display/DDW17/Sealed+Combustion)": The fireplace has sealed combustion chamber.
    SealedCombustion,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244827)": See the remarks fields for additional information about the fireplace(s).
    SeeRemarks,

    /// "[See Through](https://ddwiki.reso.org/display/DDW17/See+Through)": The property has a see-through fireplace, usually positioned between two rooms.
    SeeThrough,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244829)": The property has a fireplace made with stone.
    Stone,

    /// "[Ventless](https://ddwiki.reso.org/display/DDW17/Ventless)": The property has a ventless fireplace.
    Ventless,

    /// "[Wood Burning](https://ddwiki.reso.org/display/DDW17/Wood+Burning)": The fireplace is wood burning.
    WoodBurning,

    /// "[Wood Burning Stove](https://ddwiki.reso.org/display/DDW17/Wood+Burning+Stove)": The property includes a wood burning stove.
    WoodBurningStove,

    /// "[Zero Clearance](https://ddwiki.reso.org/display/DDW17/Zero+Clearance)": The property has a zero clearance fireplace.  Zero clearance fireplaces are built to be placed almost directly against combustible materials like wood, walls, or paneling.
    ZeroClearance,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for FireplaceFeatures {
    fn from(s: String) -> FireplaceFeatures {
        match s.as_ref() {
            "Basement" => FireplaceFeatures::Basement,

            "Bath" => FireplaceFeatures::Bath,

            "Bedroom" => FireplaceFeatures::Bedroom,

            "Blower Fan" => FireplaceFeatures::BlowerFan,

            "Circulating" => FireplaceFeatures::Circulating,

            "Decorative" => FireplaceFeatures::Decorative,

            "Den" => FireplaceFeatures::Den,

            "Dining Room" => FireplaceFeatures::DiningRoom,

            "Double Sided" => FireplaceFeatures::DoubleSided,

            "Electric" => FireplaceFeatures::Electric,

            "EPA Certified Wood Stove" => FireplaceFeatures::EPACertifiedWoodStove,

            "EPA Qualified Fireplace" => FireplaceFeatures::EPAQualifiedFireplace,

            "Factory Built" => FireplaceFeatures::FactoryBuilt,

            "Family Room" => FireplaceFeatures::FamilyRoom,

            "Fire Pit" => FireplaceFeatures::FirePit,

            "Free Standing" => FireplaceFeatures::FreeStanding,

            "Gas" => FireplaceFeatures::Gas,

            "Gas Log" => FireplaceFeatures::GasLog,

            "Gas Starter" => FireplaceFeatures::GasStarter,

            "Glass Doors" => FireplaceFeatures::GlassDoors,

            "Great Room" => FireplaceFeatures::GreatRoom,

            "Heatilator" => FireplaceFeatures::Heatilator,

            "Insert" => FireplaceFeatures::Insert,

            "Kitchen" => FireplaceFeatures::Kitchen,

            "Library" => FireplaceFeatures::Library,

            "Living Room" => FireplaceFeatures::LivingRoom,

            "Masonry" => FireplaceFeatures::Masonry,

            "Master Bedroom" => FireplaceFeatures::MasterBedroom,

            "Metal" => FireplaceFeatures::Metal,

            "None" => FireplaceFeatures::None,

            "Other" => FireplaceFeatures::Other,

            "Outside" => FireplaceFeatures::Outside,

            "Pellet Stove" => FireplaceFeatures::PelletStove,

            "Propane" => FireplaceFeatures::Propane,

            "Raised Hearth" => FireplaceFeatures::RaisedHearth,

            "Recreation Room" => FireplaceFeatures::RecreationRoom,

            "Sealed Combustion" => FireplaceFeatures::SealedCombustion,

            "See Remarks" => FireplaceFeatures::SeeRemarks,

            "See Through" => FireplaceFeatures::SeeThrough,

            "Stone" => FireplaceFeatures::Stone,

            "Ventless" => FireplaceFeatures::Ventless,

            "Wood Burning" => FireplaceFeatures::WoodBurning,

            "Wood Burning Stove" => FireplaceFeatures::WoodBurningStove,

            "Zero Clearance" => FireplaceFeatures::ZeroClearance,

            _ => FireplaceFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for FireplaceFeatures {
    fn from(s: &str) -> FireplaceFeatures {
        match s {
            "Basement" => FireplaceFeatures::Basement,

            "Bath" => FireplaceFeatures::Bath,

            "Bedroom" => FireplaceFeatures::Bedroom,

            "Blower Fan" => FireplaceFeatures::BlowerFan,

            "Circulating" => FireplaceFeatures::Circulating,

            "Decorative" => FireplaceFeatures::Decorative,

            "Den" => FireplaceFeatures::Den,

            "Dining Room" => FireplaceFeatures::DiningRoom,

            "Double Sided" => FireplaceFeatures::DoubleSided,

            "Electric" => FireplaceFeatures::Electric,

            "EPA Certified Wood Stove" => FireplaceFeatures::EPACertifiedWoodStove,

            "EPA Qualified Fireplace" => FireplaceFeatures::EPAQualifiedFireplace,

            "Factory Built" => FireplaceFeatures::FactoryBuilt,

            "Family Room" => FireplaceFeatures::FamilyRoom,

            "Fire Pit" => FireplaceFeatures::FirePit,

            "Free Standing" => FireplaceFeatures::FreeStanding,

            "Gas" => FireplaceFeatures::Gas,

            "Gas Log" => FireplaceFeatures::GasLog,

            "Gas Starter" => FireplaceFeatures::GasStarter,

            "Glass Doors" => FireplaceFeatures::GlassDoors,

            "Great Room" => FireplaceFeatures::GreatRoom,

            "Heatilator" => FireplaceFeatures::Heatilator,

            "Insert" => FireplaceFeatures::Insert,

            "Kitchen" => FireplaceFeatures::Kitchen,

            "Library" => FireplaceFeatures::Library,

            "Living Room" => FireplaceFeatures::LivingRoom,

            "Masonry" => FireplaceFeatures::Masonry,

            "Master Bedroom" => FireplaceFeatures::MasterBedroom,

            "Metal" => FireplaceFeatures::Metal,

            "None" => FireplaceFeatures::None,

            "Other" => FireplaceFeatures::Other,

            "Outside" => FireplaceFeatures::Outside,

            "Pellet Stove" => FireplaceFeatures::PelletStove,

            "Propane" => FireplaceFeatures::Propane,

            "Raised Hearth" => FireplaceFeatures::RaisedHearth,

            "Recreation Room" => FireplaceFeatures::RecreationRoom,

            "Sealed Combustion" => FireplaceFeatures::SealedCombustion,

            "See Remarks" => FireplaceFeatures::SeeRemarks,

            "See Through" => FireplaceFeatures::SeeThrough,

            "Stone" => FireplaceFeatures::Stone,

            "Ventless" => FireplaceFeatures::Ventless,

            "Wood Burning" => FireplaceFeatures::WoodBurning,

            "Wood Burning Stove" => FireplaceFeatures::WoodBurningStove,

            "Zero Clearance" => FireplaceFeatures::ZeroClearance,

            _ => FireplaceFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a FireplaceFeatures> for &'a str {
    fn from(s: &'a FireplaceFeatures) -> &'a str {
        match s {
            FireplaceFeatures::Basement => "Basement",

            FireplaceFeatures::Bath => "Bath",

            FireplaceFeatures::Bedroom => "Bedroom",

            FireplaceFeatures::BlowerFan => "Blower Fan",

            FireplaceFeatures::Circulating => "Circulating",

            FireplaceFeatures::Decorative => "Decorative",

            FireplaceFeatures::Den => "Den",

            FireplaceFeatures::DiningRoom => "Dining Room",

            FireplaceFeatures::DoubleSided => "Double Sided",

            FireplaceFeatures::Electric => "Electric",

            FireplaceFeatures::EPACertifiedWoodStove => "EPA Certified Wood Stove",

            FireplaceFeatures::EPAQualifiedFireplace => "EPA Qualified Fireplace",

            FireplaceFeatures::FactoryBuilt => "Factory Built",

            FireplaceFeatures::FamilyRoom => "Family Room",

            FireplaceFeatures::FirePit => "Fire Pit",

            FireplaceFeatures::FreeStanding => "Free Standing",

            FireplaceFeatures::Gas => "Gas",

            FireplaceFeatures::GasLog => "Gas Log",

            FireplaceFeatures::GasStarter => "Gas Starter",

            FireplaceFeatures::GlassDoors => "Glass Doors",

            FireplaceFeatures::GreatRoom => "Great Room",

            FireplaceFeatures::Heatilator => "Heatilator",

            FireplaceFeatures::Insert => "Insert",

            FireplaceFeatures::Kitchen => "Kitchen",

            FireplaceFeatures::Library => "Library",

            FireplaceFeatures::LivingRoom => "Living Room",

            FireplaceFeatures::Masonry => "Masonry",

            FireplaceFeatures::MasterBedroom => "Master Bedroom",

            FireplaceFeatures::Metal => "Metal",

            FireplaceFeatures::None => "None",

            FireplaceFeatures::Other => "Other",

            FireplaceFeatures::Outside => "Outside",

            FireplaceFeatures::PelletStove => "Pellet Stove",

            FireplaceFeatures::Propane => "Propane",

            FireplaceFeatures::RaisedHearth => "Raised Hearth",

            FireplaceFeatures::RecreationRoom => "Recreation Room",

            FireplaceFeatures::SealedCombustion => "Sealed Combustion",

            FireplaceFeatures::SeeRemarks => "See Remarks",

            FireplaceFeatures::SeeThrough => "See Through",

            FireplaceFeatures::Stone => "Stone",

            FireplaceFeatures::Ventless => "Ventless",

            FireplaceFeatures::WoodBurning => "Wood Burning",

            FireplaceFeatures::WoodBurningStove => "Wood Burning Stove",

            FireplaceFeatures::ZeroClearance => "Zero Clearance",

            FireplaceFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for FireplaceFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FireplaceFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_fireplace_features_format {
    use super::FireplaceFeatures;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<FireplaceFeatures>>,
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
    ) -> Result<Option<Vec<FireplaceFeatures>>, D::Error>
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
