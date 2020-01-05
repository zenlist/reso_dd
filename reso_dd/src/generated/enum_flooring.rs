// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Flooring Lookups](https://ddwiki.reso.org/display/DDW17/Flooring+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Flooring {
    /// "[Adobe](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244835)": The property includes adobe flooring.
    Adobe,

    /// "[Bamboo](https://ddwiki.reso.org/display/DDW17/Bamboo)": The property includes bamboo flooring.
    Bamboo,

    /// "[Brick](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244837)": The property includes brick flooring.
    Brick,

    /// "[Carpet](https://ddwiki.reso.org/display/DDW17/Carpet)": The property includes carpet flooring.
    Carpet,

    /// "[Ceramic Tile](https://ddwiki.reso.org/display/DDW17/Ceramic+Tile)": The property includes ceramic tile flooring.
    CeramicTile,

    /// "[Clay](https://ddwiki.reso.org/display/DDW17/Clay)": The property includes clay flooring.
    Clay,

    /// "[Combination](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244841)": The property includes combination flooring.
    Combination,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244842)": The property includes concrete flooring.
    Concrete,

    /// "[Cork](https://ddwiki.reso.org/display/DDW17/Cork)": The property includes cork flooring.
    Cork,

    /// "[CRI Green Label Plus Certified Carpet](https://ddwiki.reso.org/display/DDW17/CRI+Green+Label+Plus+Certified+Carpet)": The property includes CRI Green Label Plus certified carpet flooring.
    CRIGreenLabelPlusCertifiedCarpet,

    /// "[Dirt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244845)": The property has dirt flooring.
    Dirt,

    /// "[FloorScore(r) Certified Flooring](https://ddwiki.reso.org/display/DDW17/FloorScore%28r%29+Certified+Flooring)": The property includes FloorScore(r) certified flooring .
    FloorScorerCertifiedFlooring,

    /// "[FSC or SFI Certified Source Hardwood](https://ddwiki.reso.org/display/DDW17/FSC+or+SFI+Certified+Source+Hardwood)": The property includes FSC or SFI certified source hardwood flooring.
    FSCorSFICertifiedSourceHardwood,

    /// "[Granite](https://ddwiki.reso.org/display/DDW17/Granite)": The property includes granite flooring.
    Granite,

    /// "[Hardwood](https://ddwiki.reso.org/display/DDW17/Hardwood)": The property includes hardwood flooring.
    Hardwood,

    /// "[Laminate](https://ddwiki.reso.org/display/DDW17/Laminate)": The property includes laminate flooring.
    Laminate,

    /// "[Linoleum](https://ddwiki.reso.org/display/DDW17/Linoleum)": The property includes linoleum flooring.
    Linoleum,

    /// "[Marble](https://ddwiki.reso.org/display/DDW17/Marble)": The property includes marble flooring.
    Marble,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244853)": The property has no flooring.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244854)": The property includes flooring that is not included in this list.
    Other,

    /// "[Painted/Stained](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244855)": The property includes painted or stained flooring.
    PaintedStained,

    /// "[Parquet](https://ddwiki.reso.org/display/DDW17/Parquet)": The property includes parquet flooring.
    Parquet,

    /// "[Pavers](https://ddwiki.reso.org/display/DDW17/Pavers)": The property includes flooring pavers.
    Pavers,

    /// "[Reclaimed Wood](https://ddwiki.reso.org/display/DDW17/Reclaimed+Wood)": The property includes reclaimed wood flooring.
    ReclaimedWood,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244859)": See the remarks field for additional information about the flooring included with the property.
    SeeRemarks,

    /// "[Simulated Wood](https://ddwiki.reso.org/display/DDW17/Simulated+Wood)": The property includes simulated wood flooring.
    SimulatedWood,

    /// "[Slate](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244861)": The property includes slate flooring.
    Slate,

    /// "[Softwood](https://ddwiki.reso.org/display/DDW17/Softwood)": The property includes softwood flooring.
    Softwood,

    /// "[Stamped](https://ddwiki.reso.org/display/DDW17/Stamped)": The property includes stamped flooring.
    Stamped,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244864)": The property includes stone flooring.
    Stone,

    /// "[Sustainable](https://ddwiki.reso.org/display/DDW17/Sustainable)": The property includes sustainable flooring.
    Sustainable,

    /// "[Terrazzo](https://ddwiki.reso.org/display/DDW17/Terrazzo)": The property includes terrazzo flooring.
    Terrazzo,

    /// "[Tile](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244867)": The property includes tile flooring.
    Tile,

    /// "[Varies](https://ddwiki.reso.org/display/DDW17/Varies)": The property flooring type varies.
    Varies,

    /// "[Vinyl](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244869)": The property includes vinyl flooring.
    Vinyl,

    /// "[Wood](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244870)": The property includes wood flooring.
    Wood,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Flooring {
    fn from_str(s: &str) -> Flooring {
        match s {
            "Adobe" => Flooring::Adobe,

            "Bamboo" => Flooring::Bamboo,

            "Brick" => Flooring::Brick,

            "Carpet" => Flooring::Carpet,

            "Ceramic Tile" => Flooring::CeramicTile,

            "Clay" => Flooring::Clay,

            "Combination" => Flooring::Combination,

            "Concrete" => Flooring::Concrete,

            "Cork" => Flooring::Cork,

            "CRI Green Label Plus Certified Carpet" => Flooring::CRIGreenLabelPlusCertifiedCarpet,

            "Dirt" => Flooring::Dirt,

            "FloorScore(r) Certified Flooring" => Flooring::FloorScorerCertifiedFlooring,

            "FSC or SFI Certified Source Hardwood" => Flooring::FSCorSFICertifiedSourceHardwood,

            "Granite" => Flooring::Granite,

            "Hardwood" => Flooring::Hardwood,

            "Laminate" => Flooring::Laminate,

            "Linoleum" => Flooring::Linoleum,

            "Marble" => Flooring::Marble,

            "None" => Flooring::None,

            "Other" => Flooring::Other,

            "Painted/Stained" => Flooring::PaintedStained,

            "Parquet" => Flooring::Parquet,

            "Pavers" => Flooring::Pavers,

            "Reclaimed Wood" => Flooring::ReclaimedWood,

            "See Remarks" => Flooring::SeeRemarks,

            "Simulated Wood" => Flooring::SimulatedWood,

            "Slate" => Flooring::Slate,

            "Softwood" => Flooring::Softwood,

            "Stamped" => Flooring::Stamped,

            "Stone" => Flooring::Stone,

            "Sustainable" => Flooring::Sustainable,

            "Terrazzo" => Flooring::Terrazzo,

            "Tile" => Flooring::Tile,

            "Varies" => Flooring::Varies,

            "Vinyl" => Flooring::Vinyl,

            "Wood" => Flooring::Wood,

            _ => Flooring::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Flooring {
        match s.as_ref() {
            "Adobe" => Flooring::Adobe,

            "Bamboo" => Flooring::Bamboo,

            "Brick" => Flooring::Brick,

            "Carpet" => Flooring::Carpet,

            "Ceramic Tile" => Flooring::CeramicTile,

            "Clay" => Flooring::Clay,

            "Combination" => Flooring::Combination,

            "Concrete" => Flooring::Concrete,

            "Cork" => Flooring::Cork,

            "CRI Green Label Plus Certified Carpet" => Flooring::CRIGreenLabelPlusCertifiedCarpet,

            "Dirt" => Flooring::Dirt,

            "FloorScore(r) Certified Flooring" => Flooring::FloorScorerCertifiedFlooring,

            "FSC or SFI Certified Source Hardwood" => Flooring::FSCorSFICertifiedSourceHardwood,

            "Granite" => Flooring::Granite,

            "Hardwood" => Flooring::Hardwood,

            "Laminate" => Flooring::Laminate,

            "Linoleum" => Flooring::Linoleum,

            "Marble" => Flooring::Marble,

            "None" => Flooring::None,

            "Other" => Flooring::Other,

            "Painted/Stained" => Flooring::PaintedStained,

            "Parquet" => Flooring::Parquet,

            "Pavers" => Flooring::Pavers,

            "Reclaimed Wood" => Flooring::ReclaimedWood,

            "See Remarks" => Flooring::SeeRemarks,

            "Simulated Wood" => Flooring::SimulatedWood,

            "Slate" => Flooring::Slate,

            "Softwood" => Flooring::Softwood,

            "Stamped" => Flooring::Stamped,

            "Stone" => Flooring::Stone,

            "Sustainable" => Flooring::Sustainable,

            "Terrazzo" => Flooring::Terrazzo,

            "Tile" => Flooring::Tile,

            "Varies" => Flooring::Varies,

            "Vinyl" => Flooring::Vinyl,

            "Wood" => Flooring::Wood,

            _ => Flooring::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Flooring::Adobe => "Adobe",

            Flooring::Bamboo => "Bamboo",

            Flooring::Brick => "Brick",

            Flooring::Carpet => "Carpet",

            Flooring::CeramicTile => "Ceramic Tile",

            Flooring::Clay => "Clay",

            Flooring::Combination => "Combination",

            Flooring::Concrete => "Concrete",

            Flooring::Cork => "Cork",

            Flooring::CRIGreenLabelPlusCertifiedCarpet => "CRI Green Label Plus Certified Carpet",

            Flooring::Dirt => "Dirt",

            Flooring::FloorScorerCertifiedFlooring => "FloorScore(r) Certified Flooring",

            Flooring::FSCorSFICertifiedSourceHardwood => "FSC or SFI Certified Source Hardwood",

            Flooring::Granite => "Granite",

            Flooring::Hardwood => "Hardwood",

            Flooring::Laminate => "Laminate",

            Flooring::Linoleum => "Linoleum",

            Flooring::Marble => "Marble",

            Flooring::None => "None",

            Flooring::Other => "Other",

            Flooring::PaintedStained => "Painted/Stained",

            Flooring::Parquet => "Parquet",

            Flooring::Pavers => "Pavers",

            Flooring::ReclaimedWood => "Reclaimed Wood",

            Flooring::SeeRemarks => "See Remarks",

            Flooring::SimulatedWood => "Simulated Wood",

            Flooring::Slate => "Slate",

            Flooring::Softwood => "Softwood",

            Flooring::Stamped => "Stamped",

            Flooring::Stone => "Stone",

            Flooring::Sustainable => "Sustainable",

            Flooring::Terrazzo => "Terrazzo",

            Flooring::Tile => "Tile",

            Flooring::Varies => "Varies",

            Flooring::Vinyl => "Vinyl",

            Flooring::Wood => "Wood",

            Flooring::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Flooring::Adobe => "Adobe".into(),

            Flooring::Bamboo => "Bamboo".into(),

            Flooring::Brick => "Brick".into(),

            Flooring::Carpet => "Carpet".into(),

            Flooring::CeramicTile => "Ceramic Tile".into(),

            Flooring::Clay => "Clay".into(),

            Flooring::Combination => "Combination".into(),

            Flooring::Concrete => "Concrete".into(),

            Flooring::Cork => "Cork".into(),

            Flooring::CRIGreenLabelPlusCertifiedCarpet => {
                "CRI Green Label Plus Certified Carpet".into()
            }

            Flooring::Dirt => "Dirt".into(),

            Flooring::FloorScorerCertifiedFlooring => "FloorScore(r) Certified Flooring".into(),

            Flooring::FSCorSFICertifiedSourceHardwood => {
                "FSC or SFI Certified Source Hardwood".into()
            }

            Flooring::Granite => "Granite".into(),

            Flooring::Hardwood => "Hardwood".into(),

            Flooring::Laminate => "Laminate".into(),

            Flooring::Linoleum => "Linoleum".into(),

            Flooring::Marble => "Marble".into(),

            Flooring::None => "None".into(),

            Flooring::Other => "Other".into(),

            Flooring::PaintedStained => "Painted/Stained".into(),

            Flooring::Parquet => "Parquet".into(),

            Flooring::Pavers => "Pavers".into(),

            Flooring::ReclaimedWood => "Reclaimed Wood".into(),

            Flooring::SeeRemarks => "See Remarks".into(),

            Flooring::SimulatedWood => "Simulated Wood".into(),

            Flooring::Slate => "Slate".into(),

            Flooring::Softwood => "Softwood".into(),

            Flooring::Stamped => "Stamped".into(),

            Flooring::Stone => "Stone".into(),

            Flooring::Sustainable => "Sustainable".into(),

            Flooring::Terrazzo => "Terrazzo".into(),

            Flooring::Tile => "Tile".into(),

            Flooring::Varies => "Varies".into(),

            Flooring::Vinyl => "Vinyl".into(),

            Flooring::Wood => "Wood".into(),

            Flooring::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Flooring::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Flooring {
    fn from(s: String) -> Flooring {
        match s.as_ref() {
            "Adobe" => Flooring::Adobe,

            "Bamboo" => Flooring::Bamboo,

            "Brick" => Flooring::Brick,

            "Carpet" => Flooring::Carpet,

            "Ceramic Tile" => Flooring::CeramicTile,

            "Clay" => Flooring::Clay,

            "Combination" => Flooring::Combination,

            "Concrete" => Flooring::Concrete,

            "Cork" => Flooring::Cork,

            "CRI Green Label Plus Certified Carpet" => Flooring::CRIGreenLabelPlusCertifiedCarpet,

            "Dirt" => Flooring::Dirt,

            "FloorScore(r) Certified Flooring" => Flooring::FloorScorerCertifiedFlooring,

            "FSC or SFI Certified Source Hardwood" => Flooring::FSCorSFICertifiedSourceHardwood,

            "Granite" => Flooring::Granite,

            "Hardwood" => Flooring::Hardwood,

            "Laminate" => Flooring::Laminate,

            "Linoleum" => Flooring::Linoleum,

            "Marble" => Flooring::Marble,

            "None" => Flooring::None,

            "Other" => Flooring::Other,

            "Painted/Stained" => Flooring::PaintedStained,

            "Parquet" => Flooring::Parquet,

            "Pavers" => Flooring::Pavers,

            "Reclaimed Wood" => Flooring::ReclaimedWood,

            "See Remarks" => Flooring::SeeRemarks,

            "Simulated Wood" => Flooring::SimulatedWood,

            "Slate" => Flooring::Slate,

            "Softwood" => Flooring::Softwood,

            "Stamped" => Flooring::Stamped,

            "Stone" => Flooring::Stone,

            "Sustainable" => Flooring::Sustainable,

            "Terrazzo" => Flooring::Terrazzo,

            "Tile" => Flooring::Tile,

            "Varies" => Flooring::Varies,

            "Vinyl" => Flooring::Vinyl,

            "Wood" => Flooring::Wood,

            _ => Flooring::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Flooring {
    fn from(s: &str) -> Flooring {
        match s {
            "Adobe" => Flooring::Adobe,

            "Bamboo" => Flooring::Bamboo,

            "Brick" => Flooring::Brick,

            "Carpet" => Flooring::Carpet,

            "Ceramic Tile" => Flooring::CeramicTile,

            "Clay" => Flooring::Clay,

            "Combination" => Flooring::Combination,

            "Concrete" => Flooring::Concrete,

            "Cork" => Flooring::Cork,

            "CRI Green Label Plus Certified Carpet" => Flooring::CRIGreenLabelPlusCertifiedCarpet,

            "Dirt" => Flooring::Dirt,

            "FloorScore(r) Certified Flooring" => Flooring::FloorScorerCertifiedFlooring,

            "FSC or SFI Certified Source Hardwood" => Flooring::FSCorSFICertifiedSourceHardwood,

            "Granite" => Flooring::Granite,

            "Hardwood" => Flooring::Hardwood,

            "Laminate" => Flooring::Laminate,

            "Linoleum" => Flooring::Linoleum,

            "Marble" => Flooring::Marble,

            "None" => Flooring::None,

            "Other" => Flooring::Other,

            "Painted/Stained" => Flooring::PaintedStained,

            "Parquet" => Flooring::Parquet,

            "Pavers" => Flooring::Pavers,

            "Reclaimed Wood" => Flooring::ReclaimedWood,

            "See Remarks" => Flooring::SeeRemarks,

            "Simulated Wood" => Flooring::SimulatedWood,

            "Slate" => Flooring::Slate,

            "Softwood" => Flooring::Softwood,

            "Stamped" => Flooring::Stamped,

            "Stone" => Flooring::Stone,

            "Sustainable" => Flooring::Sustainable,

            "Terrazzo" => Flooring::Terrazzo,

            "Tile" => Flooring::Tile,

            "Varies" => Flooring::Varies,

            "Vinyl" => Flooring::Vinyl,

            "Wood" => Flooring::Wood,

            _ => Flooring::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Flooring> for &'a str {
    fn from(s: &'a Flooring) -> &'a str {
        match s {
            Flooring::Adobe => "Adobe",

            Flooring::Bamboo => "Bamboo",

            Flooring::Brick => "Brick",

            Flooring::Carpet => "Carpet",

            Flooring::CeramicTile => "Ceramic Tile",

            Flooring::Clay => "Clay",

            Flooring::Combination => "Combination",

            Flooring::Concrete => "Concrete",

            Flooring::Cork => "Cork",

            Flooring::CRIGreenLabelPlusCertifiedCarpet => "CRI Green Label Plus Certified Carpet",

            Flooring::Dirt => "Dirt",

            Flooring::FloorScorerCertifiedFlooring => "FloorScore(r) Certified Flooring",

            Flooring::FSCorSFICertifiedSourceHardwood => "FSC or SFI Certified Source Hardwood",

            Flooring::Granite => "Granite",

            Flooring::Hardwood => "Hardwood",

            Flooring::Laminate => "Laminate",

            Flooring::Linoleum => "Linoleum",

            Flooring::Marble => "Marble",

            Flooring::None => "None",

            Flooring::Other => "Other",

            Flooring::PaintedStained => "Painted/Stained",

            Flooring::Parquet => "Parquet",

            Flooring::Pavers => "Pavers",

            Flooring::ReclaimedWood => "Reclaimed Wood",

            Flooring::SeeRemarks => "See Remarks",

            Flooring::SimulatedWood => "Simulated Wood",

            Flooring::Slate => "Slate",

            Flooring::Softwood => "Softwood",

            Flooring::Stamped => "Stamped",

            Flooring::Stone => "Stone",

            Flooring::Sustainable => "Sustainable",

            Flooring::Terrazzo => "Terrazzo",

            Flooring::Tile => "Tile",

            Flooring::Varies => "Varies",

            Flooring::Vinyl => "Vinyl",

            Flooring::Wood => "Wood",

            Flooring::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Flooring {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Flooring {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
