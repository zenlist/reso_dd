// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [PoolFeatures Lookups](https://ddwiki.reso.org/display/DDW17/PoolFeatures+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PoolFeatures {
    /// "[Above Ground](https://ddwiki.reso.org/display/DDW17/Above+Ground)": The pool is above ground.
    AboveGround,

    /// "[Association](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246056)": The pool is an association pool.
    Association,

    /// "[Black Bottom](https://ddwiki.reso.org/display/DDW17/Black+Bottom)": The pool has a black bottom.
    BlackBottom,

    /// "[Cabana](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246058)": The pool has a cabana.
    Cabana,

    /// "[Community](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246059)": The pool is a community/shared pool.
    Community,

    /// "[Diving Board](https://ddwiki.reso.org/display/DDW17/Diving+Board)": The pool has a diving board.
    DivingBoard,

    /// "[Electric Heat](https://ddwiki.reso.org/display/DDW17/Electric+Heat)": The pool is heated by electricity.
    ElectricHeat,

    /// "[ENERGY STAR Qualified pool pump](https://ddwiki.reso.org/display/DDW17/ENERGY+STAR+Qualified+pool+pump)": The pool has an ENERGY STAR Qualified pool pump.
    ENERGYSTARQualifiedpoolpump,

    /// "[Fenced](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246063)": The pool is fenced.
    Fenced,

    /// "[Fiberglass](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246064)": The pool is made of or lined with fiberglass.
    Fiberglass,

    /// "[Filtered](https://ddwiki.reso.org/display/DDW17/Filtered)": The pool has a filtration system.
    Filtered,

    /// "[Gas Heat](https://ddwiki.reso.org/display/DDW17/Gas+Heat)": The pool is heated by gas.
    GasHeat,

    /// "[Gunite](https://ddwiki.reso.org/display/DDW17/Gunite)": The pool has a gunite surface.
    Gunite,

    /// "[Heated](https://ddwiki.reso.org/display/DDW17/Heated)": The pool is heated.
    Heated,

    /// "[In Ground](https://ddwiki.reso.org/display/DDW17/In+Ground)": The pool is built into the ground.
    InGround,

    /// "[Indoor](https://ddwiki.reso.org/display/DDW17/Indoor)": The pool is indoors or within a structure.
    Indoor,

    /// "[Infinity](https://ddwiki.reso.org/display/DDW17/Infinity)": Also named a negative edge, zero edge or infinity edge, an infinity pool has one or more edges where water flows over the edge creating a visual effect of water with no boundary.
    Infinity,

    /// "[Lap](https://ddwiki.reso.org/display/DDW17/Lap)": The pool is specifically designed for swimming laps.
    Lap,

    /// "[Liner](https://ddwiki.reso.org/display/DDW17/Liner)": The pool has a liner.
    Liner,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246074)": There is no pool included with the property.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246075)": There are pool features other than those included in this list.
    Other,

    /// "[Outdoor Pool](https://ddwiki.reso.org/display/DDW17/Outdoor+Pool)": The pool is outdoors.
    OutdoorPool,

    /// "[Pool Cover](https://ddwiki.reso.org/display/DDW17/Pool+Cover)": The pool has a cover.
    PoolCover,

    /// "[Pool Sweep](https://ddwiki.reso.org/display/DDW17/Pool+Sweep)": The pool has an automatic sweep or cleaner.
    PoolSweep,

    /// "[Pool/Spa Combo](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246077)": The pool includes a spa.
    PoolSpaCombo,

    /// "[Private](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246080)": The pool is privately owned and/or secluded.
    Private,

    /// "[Salt Water](https://ddwiki.reso.org/display/DDW17/Salt+Water)": The pool has a salt water system.
    SaltWater,

    /// "[Screen Enclosure](https://ddwiki.reso.org/display/DDW17/Screen+Enclosure)": The pool has a screened enclosure.
    ScreenEnclosure,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246084)": See the remarks fields for more information about the pool.
    SeeRemarks,

    /// "[Solar Cover](https://ddwiki.reso.org/display/DDW17/Solar+Cover)": The pool has a solar cover.
    SolarCover,

    /// "[Solar Heat](https://ddwiki.reso.org/display/DDW17/Solar+Heat)": The pool has some form of solar heating.
    SolarHeat,

    /// "[Sport](https://ddwiki.reso.org/display/DDW17/Sport)": The pool has two shallow ends on opposite sides of the pool with a deeper center.
    Sport,

    /// "[Tile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246088)": The pool is tiled.
    Tile,

    /// "[Vinyl](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246089)": The pool has a vinyl surface.
    Vinyl,

    /// "[Waterfall](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246090)": The pool has a waterfall.
    Waterfall,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for PoolFeatures {
    fn from_str(s: &str) -> PoolFeatures {
        match s {
            "Above Ground" => PoolFeatures::AboveGround,

            "Association" => PoolFeatures::Association,

            "Black Bottom" => PoolFeatures::BlackBottom,

            "Cabana" => PoolFeatures::Cabana,

            "Community" => PoolFeatures::Community,

            "Diving Board" => PoolFeatures::DivingBoard,

            "Electric Heat" => PoolFeatures::ElectricHeat,

            "ENERGY STAR Qualified pool pump" => PoolFeatures::ENERGYSTARQualifiedpoolpump,

            "Fenced" => PoolFeatures::Fenced,

            "Fiberglass" => PoolFeatures::Fiberglass,

            "Filtered" => PoolFeatures::Filtered,

            "Gas Heat" => PoolFeatures::GasHeat,

            "Gunite" => PoolFeatures::Gunite,

            "Heated" => PoolFeatures::Heated,

            "In Ground" => PoolFeatures::InGround,

            "Indoor" => PoolFeatures::Indoor,

            "Infinity" => PoolFeatures::Infinity,

            "Lap" => PoolFeatures::Lap,

            "Liner" => PoolFeatures::Liner,

            "None" => PoolFeatures::None,

            "Other" => PoolFeatures::Other,

            "Outdoor Pool" => PoolFeatures::OutdoorPool,

            "Pool Cover" => PoolFeatures::PoolCover,

            "Pool Sweep" => PoolFeatures::PoolSweep,

            "Pool/Spa Combo" => PoolFeatures::PoolSpaCombo,

            "Private" => PoolFeatures::Private,

            "Salt Water" => PoolFeatures::SaltWater,

            "Screen Enclosure" => PoolFeatures::ScreenEnclosure,

            "See Remarks" => PoolFeatures::SeeRemarks,

            "Solar Cover" => PoolFeatures::SolarCover,

            "Solar Heat" => PoolFeatures::SolarHeat,

            "Sport" => PoolFeatures::Sport,

            "Tile" => PoolFeatures::Tile,

            "Vinyl" => PoolFeatures::Vinyl,

            "Waterfall" => PoolFeatures::Waterfall,

            _ => PoolFeatures::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> PoolFeatures {
        match s.as_ref() {
            "Above Ground" => PoolFeatures::AboveGround,

            "Association" => PoolFeatures::Association,

            "Black Bottom" => PoolFeatures::BlackBottom,

            "Cabana" => PoolFeatures::Cabana,

            "Community" => PoolFeatures::Community,

            "Diving Board" => PoolFeatures::DivingBoard,

            "Electric Heat" => PoolFeatures::ElectricHeat,

            "ENERGY STAR Qualified pool pump" => PoolFeatures::ENERGYSTARQualifiedpoolpump,

            "Fenced" => PoolFeatures::Fenced,

            "Fiberglass" => PoolFeatures::Fiberglass,

            "Filtered" => PoolFeatures::Filtered,

            "Gas Heat" => PoolFeatures::GasHeat,

            "Gunite" => PoolFeatures::Gunite,

            "Heated" => PoolFeatures::Heated,

            "In Ground" => PoolFeatures::InGround,

            "Indoor" => PoolFeatures::Indoor,

            "Infinity" => PoolFeatures::Infinity,

            "Lap" => PoolFeatures::Lap,

            "Liner" => PoolFeatures::Liner,

            "None" => PoolFeatures::None,

            "Other" => PoolFeatures::Other,

            "Outdoor Pool" => PoolFeatures::OutdoorPool,

            "Pool Cover" => PoolFeatures::PoolCover,

            "Pool Sweep" => PoolFeatures::PoolSweep,

            "Pool/Spa Combo" => PoolFeatures::PoolSpaCombo,

            "Private" => PoolFeatures::Private,

            "Salt Water" => PoolFeatures::SaltWater,

            "Screen Enclosure" => PoolFeatures::ScreenEnclosure,

            "See Remarks" => PoolFeatures::SeeRemarks,

            "Solar Cover" => PoolFeatures::SolarCover,

            "Solar Heat" => PoolFeatures::SolarHeat,

            "Sport" => PoolFeatures::Sport,

            "Tile" => PoolFeatures::Tile,

            "Vinyl" => PoolFeatures::Vinyl,

            "Waterfall" => PoolFeatures::Waterfall,

            _ => PoolFeatures::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            PoolFeatures::AboveGround => "Above Ground",

            PoolFeatures::Association => "Association",

            PoolFeatures::BlackBottom => "Black Bottom",

            PoolFeatures::Cabana => "Cabana",

            PoolFeatures::Community => "Community",

            PoolFeatures::DivingBoard => "Diving Board",

            PoolFeatures::ElectricHeat => "Electric Heat",

            PoolFeatures::ENERGYSTARQualifiedpoolpump => "ENERGY STAR Qualified pool pump",

            PoolFeatures::Fenced => "Fenced",

            PoolFeatures::Fiberglass => "Fiberglass",

            PoolFeatures::Filtered => "Filtered",

            PoolFeatures::GasHeat => "Gas Heat",

            PoolFeatures::Gunite => "Gunite",

            PoolFeatures::Heated => "Heated",

            PoolFeatures::InGround => "In Ground",

            PoolFeatures::Indoor => "Indoor",

            PoolFeatures::Infinity => "Infinity",

            PoolFeatures::Lap => "Lap",

            PoolFeatures::Liner => "Liner",

            PoolFeatures::None => "None",

            PoolFeatures::Other => "Other",

            PoolFeatures::OutdoorPool => "Outdoor Pool",

            PoolFeatures::PoolCover => "Pool Cover",

            PoolFeatures::PoolSweep => "Pool Sweep",

            PoolFeatures::PoolSpaCombo => "Pool/Spa Combo",

            PoolFeatures::Private => "Private",

            PoolFeatures::SaltWater => "Salt Water",

            PoolFeatures::ScreenEnclosure => "Screen Enclosure",

            PoolFeatures::SeeRemarks => "See Remarks",

            PoolFeatures::SolarCover => "Solar Cover",

            PoolFeatures::SolarHeat => "Solar Heat",

            PoolFeatures::Sport => "Sport",

            PoolFeatures::Tile => "Tile",

            PoolFeatures::Vinyl => "Vinyl",

            PoolFeatures::Waterfall => "Waterfall",

            PoolFeatures::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            PoolFeatures::AboveGround => "Above Ground".into(),

            PoolFeatures::Association => "Association".into(),

            PoolFeatures::BlackBottom => "Black Bottom".into(),

            PoolFeatures::Cabana => "Cabana".into(),

            PoolFeatures::Community => "Community".into(),

            PoolFeatures::DivingBoard => "Diving Board".into(),

            PoolFeatures::ElectricHeat => "Electric Heat".into(),

            PoolFeatures::ENERGYSTARQualifiedpoolpump => "ENERGY STAR Qualified pool pump".into(),

            PoolFeatures::Fenced => "Fenced".into(),

            PoolFeatures::Fiberglass => "Fiberglass".into(),

            PoolFeatures::Filtered => "Filtered".into(),

            PoolFeatures::GasHeat => "Gas Heat".into(),

            PoolFeatures::Gunite => "Gunite".into(),

            PoolFeatures::Heated => "Heated".into(),

            PoolFeatures::InGround => "In Ground".into(),

            PoolFeatures::Indoor => "Indoor".into(),

            PoolFeatures::Infinity => "Infinity".into(),

            PoolFeatures::Lap => "Lap".into(),

            PoolFeatures::Liner => "Liner".into(),

            PoolFeatures::None => "None".into(),

            PoolFeatures::Other => "Other".into(),

            PoolFeatures::OutdoorPool => "Outdoor Pool".into(),

            PoolFeatures::PoolCover => "Pool Cover".into(),

            PoolFeatures::PoolSweep => "Pool Sweep".into(),

            PoolFeatures::PoolSpaCombo => "Pool/Spa Combo".into(),

            PoolFeatures::Private => "Private".into(),

            PoolFeatures::SaltWater => "Salt Water".into(),

            PoolFeatures::ScreenEnclosure => "Screen Enclosure".into(),

            PoolFeatures::SeeRemarks => "See Remarks".into(),

            PoolFeatures::SolarCover => "Solar Cover".into(),

            PoolFeatures::SolarHeat => "Solar Heat".into(),

            PoolFeatures::Sport => "Sport".into(),

            PoolFeatures::Tile => "Tile".into(),

            PoolFeatures::Vinyl => "Vinyl".into(),

            PoolFeatures::Waterfall => "Waterfall".into(),

            PoolFeatures::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            PoolFeatures::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for PoolFeatures {
    fn from(s: String) -> PoolFeatures {
        match s.as_ref() {
            "Above Ground" => PoolFeatures::AboveGround,

            "Association" => PoolFeatures::Association,

            "Black Bottom" => PoolFeatures::BlackBottom,

            "Cabana" => PoolFeatures::Cabana,

            "Community" => PoolFeatures::Community,

            "Diving Board" => PoolFeatures::DivingBoard,

            "Electric Heat" => PoolFeatures::ElectricHeat,

            "ENERGY STAR Qualified pool pump" => PoolFeatures::ENERGYSTARQualifiedpoolpump,

            "Fenced" => PoolFeatures::Fenced,

            "Fiberglass" => PoolFeatures::Fiberglass,

            "Filtered" => PoolFeatures::Filtered,

            "Gas Heat" => PoolFeatures::GasHeat,

            "Gunite" => PoolFeatures::Gunite,

            "Heated" => PoolFeatures::Heated,

            "In Ground" => PoolFeatures::InGround,

            "Indoor" => PoolFeatures::Indoor,

            "Infinity" => PoolFeatures::Infinity,

            "Lap" => PoolFeatures::Lap,

            "Liner" => PoolFeatures::Liner,

            "None" => PoolFeatures::None,

            "Other" => PoolFeatures::Other,

            "Outdoor Pool" => PoolFeatures::OutdoorPool,

            "Pool Cover" => PoolFeatures::PoolCover,

            "Pool Sweep" => PoolFeatures::PoolSweep,

            "Pool/Spa Combo" => PoolFeatures::PoolSpaCombo,

            "Private" => PoolFeatures::Private,

            "Salt Water" => PoolFeatures::SaltWater,

            "Screen Enclosure" => PoolFeatures::ScreenEnclosure,

            "See Remarks" => PoolFeatures::SeeRemarks,

            "Solar Cover" => PoolFeatures::SolarCover,

            "Solar Heat" => PoolFeatures::SolarHeat,

            "Sport" => PoolFeatures::Sport,

            "Tile" => PoolFeatures::Tile,

            "Vinyl" => PoolFeatures::Vinyl,

            "Waterfall" => PoolFeatures::Waterfall,

            _ => PoolFeatures::OpenEnumeration(s),
        }
    }
}

impl From<&str> for PoolFeatures {
    fn from(s: &str) -> PoolFeatures {
        match s {
            "Above Ground" => PoolFeatures::AboveGround,

            "Association" => PoolFeatures::Association,

            "Black Bottom" => PoolFeatures::BlackBottom,

            "Cabana" => PoolFeatures::Cabana,

            "Community" => PoolFeatures::Community,

            "Diving Board" => PoolFeatures::DivingBoard,

            "Electric Heat" => PoolFeatures::ElectricHeat,

            "ENERGY STAR Qualified pool pump" => PoolFeatures::ENERGYSTARQualifiedpoolpump,

            "Fenced" => PoolFeatures::Fenced,

            "Fiberglass" => PoolFeatures::Fiberglass,

            "Filtered" => PoolFeatures::Filtered,

            "Gas Heat" => PoolFeatures::GasHeat,

            "Gunite" => PoolFeatures::Gunite,

            "Heated" => PoolFeatures::Heated,

            "In Ground" => PoolFeatures::InGround,

            "Indoor" => PoolFeatures::Indoor,

            "Infinity" => PoolFeatures::Infinity,

            "Lap" => PoolFeatures::Lap,

            "Liner" => PoolFeatures::Liner,

            "None" => PoolFeatures::None,

            "Other" => PoolFeatures::Other,

            "Outdoor Pool" => PoolFeatures::OutdoorPool,

            "Pool Cover" => PoolFeatures::PoolCover,

            "Pool Sweep" => PoolFeatures::PoolSweep,

            "Pool/Spa Combo" => PoolFeatures::PoolSpaCombo,

            "Private" => PoolFeatures::Private,

            "Salt Water" => PoolFeatures::SaltWater,

            "Screen Enclosure" => PoolFeatures::ScreenEnclosure,

            "See Remarks" => PoolFeatures::SeeRemarks,

            "Solar Cover" => PoolFeatures::SolarCover,

            "Solar Heat" => PoolFeatures::SolarHeat,

            "Sport" => PoolFeatures::Sport,

            "Tile" => PoolFeatures::Tile,

            "Vinyl" => PoolFeatures::Vinyl,

            "Waterfall" => PoolFeatures::Waterfall,

            _ => PoolFeatures::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a PoolFeatures> for &'a str {
    fn from(s: &'a PoolFeatures) -> &'a str {
        match s {
            PoolFeatures::AboveGround => "Above Ground",

            PoolFeatures::Association => "Association",

            PoolFeatures::BlackBottom => "Black Bottom",

            PoolFeatures::Cabana => "Cabana",

            PoolFeatures::Community => "Community",

            PoolFeatures::DivingBoard => "Diving Board",

            PoolFeatures::ElectricHeat => "Electric Heat",

            PoolFeatures::ENERGYSTARQualifiedpoolpump => "ENERGY STAR Qualified pool pump",

            PoolFeatures::Fenced => "Fenced",

            PoolFeatures::Fiberglass => "Fiberglass",

            PoolFeatures::Filtered => "Filtered",

            PoolFeatures::GasHeat => "Gas Heat",

            PoolFeatures::Gunite => "Gunite",

            PoolFeatures::Heated => "Heated",

            PoolFeatures::InGround => "In Ground",

            PoolFeatures::Indoor => "Indoor",

            PoolFeatures::Infinity => "Infinity",

            PoolFeatures::Lap => "Lap",

            PoolFeatures::Liner => "Liner",

            PoolFeatures::None => "None",

            PoolFeatures::Other => "Other",

            PoolFeatures::OutdoorPool => "Outdoor Pool",

            PoolFeatures::PoolCover => "Pool Cover",

            PoolFeatures::PoolSweep => "Pool Sweep",

            PoolFeatures::PoolSpaCombo => "Pool/Spa Combo",

            PoolFeatures::Private => "Private",

            PoolFeatures::SaltWater => "Salt Water",

            PoolFeatures::ScreenEnclosure => "Screen Enclosure",

            PoolFeatures::SeeRemarks => "See Remarks",

            PoolFeatures::SolarCover => "Solar Cover",

            PoolFeatures::SolarHeat => "Solar Heat",

            PoolFeatures::Sport => "Sport",

            PoolFeatures::Tile => "Tile",

            PoolFeatures::Vinyl => "Vinyl",

            PoolFeatures::Waterfall => "Waterfall",

            PoolFeatures::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for PoolFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for PoolFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
