// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [FoundationDetails Lookups](https://ddwiki.reso.org/display/DDW17/FoundationDetails+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FoundationDetails {
    /// "[Block](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244731)": The foundation of the property is made wholly or partially of block.
    Block,

    /// "[Brick/Mortar](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244732)": The foundation of the property is made wholly or partially of brick/mortar.
    BrickMortar,

    /// "[Combination](https://ddwiki.reso.org/display/DDW17/Combination)": The foundation of the property is made of a combination of materials.
    Combination,

    /// "[Concrete Perimeter](https://ddwiki.reso.org/display/DDW17/Concrete+Perimeter)": The foundation of the property has a concrete perimeter.
    ConcretePerimeter,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244735)": There are no details about the foundation of the property.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244742)": A foundation type not included in this list.
    Other,

    /// "[Permanent](https://ddwiki.reso.org/display/DDW17/Permanent)": The foundation is permanent and not temporary or movable.
    Permanent,

    /// "[Pillar/Post/Pier](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244737)": The foundation of the property is made wholly or partially of  pillar/post/pier.
    PillarPostPier,

    /// "[Raised](https://ddwiki.reso.org/display/DDW17/Raised)": The foundation of the property is raised.
    Raised,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244739)": See the listing's remarks for details about the foundation.
    SeeRemarks,

    /// "[Slab](https://ddwiki.reso.org/display/DDW17/Slab)": The foundation of the property is made wholly or partially of  a concrete slab.
    Slab,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244741)": The foundation of the property is made wholly or partially of  stone.
    Stone,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for FoundationDetails {
    fn from_str(s: &str) -> FoundationDetails {
        match s {
            "Block" => FoundationDetails::Block,

            "Brick/Mortar" => FoundationDetails::BrickMortar,

            "Combination" => FoundationDetails::Combination,

            "Concrete Perimeter" => FoundationDetails::ConcretePerimeter,

            "None" => FoundationDetails::None,

            "Other" => FoundationDetails::Other,

            "Permanent" => FoundationDetails::Permanent,

            "Pillar/Post/Pier" => FoundationDetails::PillarPostPier,

            "Raised" => FoundationDetails::Raised,

            "See Remarks" => FoundationDetails::SeeRemarks,

            "Slab" => FoundationDetails::Slab,

            "Stone" => FoundationDetails::Stone,

            _ => FoundationDetails::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> FoundationDetails {
        match s.as_ref() {
            "Block" => FoundationDetails::Block,

            "Brick/Mortar" => FoundationDetails::BrickMortar,

            "Combination" => FoundationDetails::Combination,

            "Concrete Perimeter" => FoundationDetails::ConcretePerimeter,

            "None" => FoundationDetails::None,

            "Other" => FoundationDetails::Other,

            "Permanent" => FoundationDetails::Permanent,

            "Pillar/Post/Pier" => FoundationDetails::PillarPostPier,

            "Raised" => FoundationDetails::Raised,

            "See Remarks" => FoundationDetails::SeeRemarks,

            "Slab" => FoundationDetails::Slab,

            "Stone" => FoundationDetails::Stone,

            _ => FoundationDetails::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            FoundationDetails::Block => "Block",

            FoundationDetails::BrickMortar => "Brick/Mortar",

            FoundationDetails::Combination => "Combination",

            FoundationDetails::ConcretePerimeter => "Concrete Perimeter",

            FoundationDetails::None => "None",

            FoundationDetails::Other => "Other",

            FoundationDetails::Permanent => "Permanent",

            FoundationDetails::PillarPostPier => "Pillar/Post/Pier",

            FoundationDetails::Raised => "Raised",

            FoundationDetails::SeeRemarks => "See Remarks",

            FoundationDetails::Slab => "Slab",

            FoundationDetails::Stone => "Stone",

            FoundationDetails::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            FoundationDetails::Block => "Block".into(),

            FoundationDetails::BrickMortar => "Brick/Mortar".into(),

            FoundationDetails::Combination => "Combination".into(),

            FoundationDetails::ConcretePerimeter => "Concrete Perimeter".into(),

            FoundationDetails::None => "None".into(),

            FoundationDetails::Other => "Other".into(),

            FoundationDetails::Permanent => "Permanent".into(),

            FoundationDetails::PillarPostPier => "Pillar/Post/Pier".into(),

            FoundationDetails::Raised => "Raised".into(),

            FoundationDetails::SeeRemarks => "See Remarks".into(),

            FoundationDetails::Slab => "Slab".into(),

            FoundationDetails::Stone => "Stone".into(),

            FoundationDetails::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            FoundationDetails::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for FoundationDetails {
    fn from(s: String) -> FoundationDetails {
        match s.as_ref() {
            "Block" => FoundationDetails::Block,

            "Brick/Mortar" => FoundationDetails::BrickMortar,

            "Combination" => FoundationDetails::Combination,

            "Concrete Perimeter" => FoundationDetails::ConcretePerimeter,

            "None" => FoundationDetails::None,

            "Other" => FoundationDetails::Other,

            "Permanent" => FoundationDetails::Permanent,

            "Pillar/Post/Pier" => FoundationDetails::PillarPostPier,

            "Raised" => FoundationDetails::Raised,

            "See Remarks" => FoundationDetails::SeeRemarks,

            "Slab" => FoundationDetails::Slab,

            "Stone" => FoundationDetails::Stone,

            _ => FoundationDetails::OpenEnumeration(s),
        }
    }
}

impl From<&str> for FoundationDetails {
    fn from(s: &str) -> FoundationDetails {
        match s {
            "Block" => FoundationDetails::Block,

            "Brick/Mortar" => FoundationDetails::BrickMortar,

            "Combination" => FoundationDetails::Combination,

            "Concrete Perimeter" => FoundationDetails::ConcretePerimeter,

            "None" => FoundationDetails::None,

            "Other" => FoundationDetails::Other,

            "Permanent" => FoundationDetails::Permanent,

            "Pillar/Post/Pier" => FoundationDetails::PillarPostPier,

            "Raised" => FoundationDetails::Raised,

            "See Remarks" => FoundationDetails::SeeRemarks,

            "Slab" => FoundationDetails::Slab,

            "Stone" => FoundationDetails::Stone,

            _ => FoundationDetails::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a FoundationDetails> for &'a str {
    fn from(s: &'a FoundationDetails) -> &'a str {
        match s {
            FoundationDetails::Block => "Block",

            FoundationDetails::BrickMortar => "Brick/Mortar",

            FoundationDetails::Combination => "Combination",

            FoundationDetails::ConcretePerimeter => "Concrete Perimeter",

            FoundationDetails::None => "None",

            FoundationDetails::Other => "Other",

            FoundationDetails::Permanent => "Permanent",

            FoundationDetails::PillarPostPier => "Pillar/Post/Pier",

            FoundationDetails::Raised => "Raised",

            FoundationDetails::SeeRemarks => "See Remarks",

            FoundationDetails::Slab => "Slab",

            FoundationDetails::Stone => "Stone",

            FoundationDetails::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for FoundationDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for FoundationDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
