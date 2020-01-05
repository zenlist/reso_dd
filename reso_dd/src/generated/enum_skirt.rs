// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Skirt Lookups](https://ddwiki.reso.org/display/DDW17/Skirt+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Skirt {
    /// "[Aluminum](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246275)": The mobile/manufactured home has a skirt made of aluminum.
    Aluminum,

    /// "[Block](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246276)": The mobile/manufactured home has a skirt made of block.
    Block,

    /// "[Brick](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246277)": The mobile/manufactured home has a skirt made of brick.
    Brick,

    /// "[Combination](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246278)": The mobile/manufactured home has a skirt made of a combination of materials.
    Combination,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246279)": The mobile/manufactured home has a skirt made of concrete.
    Concrete,

    /// "[Fiberglass](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246280)": The mobile/manufactured home has a skirt made of fiberglass.
    Fiberglass,

    /// "[Frame](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246281)": The mobile/manufactured home has a skirt that is framed.
    Frame,

    /// "[Glass](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246282)": The mobile/manufactured home has a skirt made of glass.
    Glass,

    /// "[Masonite](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246283)": The mobile/manufactured home has a skirt made of Masonite.
    Masonite,

    /// "[Metal](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246284)": The mobile/manufactured home has a skirt made of metal.
    Metal,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246285)": The mobile/manufactured home does not have a skirt.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246286)": The mobile/manufactured home has a skirt made of materials other than those in this list.
    Other,

    /// "[Steel](https://ddwiki.reso.org/display/DDW17/Steel)": The mobile/manufactured home has a skirt made of steel.
    Steel,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246288)": The mobile/manufactured home has a skirt made of stone.
    Stone,

    /// "[Stucco](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246289)": The mobile/manufactured home has a skirt made of stucco.
    Stucco,

    /// "[Synthetic](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246290)": The mobile/manufactured home has a skirt made of synthetic materials.
    Synthetic,

    /// "[Unknown](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246291)": The mobile/manufactured home has a skirt made of unknown materials.
    Unknown,

    /// "[Vinyl](https://ddwiki.reso.org/display/DDW17/Vinyl)": The mobile/manufactured home has a skirt made of vinyl.
    Vinyl,

    /// "[Wood](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246293)": The mobile/manufactured home has a skirt made of wood.
    Wood,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl crate::ResoEnumeration for Skirt {
    fn from_str(s: &str) -> Skirt {
        match s {
            "Aluminum" => Skirt::Aluminum,

            "Block" => Skirt::Block,

            "Brick" => Skirt::Brick,

            "Combination" => Skirt::Combination,

            "Concrete" => Skirt::Concrete,

            "Fiberglass" => Skirt::Fiberglass,

            "Frame" => Skirt::Frame,

            "Glass" => Skirt::Glass,

            "Masonite" => Skirt::Masonite,

            "Metal" => Skirt::Metal,

            "None" => Skirt::None,

            "Other" => Skirt::Other,

            "Steel" => Skirt::Steel,

            "Stone" => Skirt::Stone,

            "Stucco" => Skirt::Stucco,

            "Synthetic" => Skirt::Synthetic,

            "Unknown" => Skirt::Unknown,

            "Vinyl" => Skirt::Vinyl,

            "Wood" => Skirt::Wood,

            _ => Skirt::OpenEnumeration(s.into()),
        }
    }

    fn from_string(s: String) -> Skirt {
        match s.as_ref() {
            "Aluminum" => Skirt::Aluminum,

            "Block" => Skirt::Block,

            "Brick" => Skirt::Brick,

            "Combination" => Skirt::Combination,

            "Concrete" => Skirt::Concrete,

            "Fiberglass" => Skirt::Fiberglass,

            "Frame" => Skirt::Frame,

            "Glass" => Skirt::Glass,

            "Masonite" => Skirt::Masonite,

            "Metal" => Skirt::Metal,

            "None" => Skirt::None,

            "Other" => Skirt::Other,

            "Steel" => Skirt::Steel,

            "Stone" => Skirt::Stone,

            "Stucco" => Skirt::Stucco,

            "Synthetic" => Skirt::Synthetic,

            "Unknown" => Skirt::Unknown,

            "Vinyl" => Skirt::Vinyl,

            "Wood" => Skirt::Wood,

            _ => Skirt::OpenEnumeration(s),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Skirt::Aluminum => "Aluminum",

            Skirt::Block => "Block",

            Skirt::Brick => "Brick",

            Skirt::Combination => "Combination",

            Skirt::Concrete => "Concrete",

            Skirt::Fiberglass => "Fiberglass",

            Skirt::Frame => "Frame",

            Skirt::Glass => "Glass",

            Skirt::Masonite => "Masonite",

            Skirt::Metal => "Metal",

            Skirt::None => "None",

            Skirt::Other => "Other",

            Skirt::Steel => "Steel",

            Skirt::Stone => "Stone",

            Skirt::Stucco => "Stucco",

            Skirt::Synthetic => "Synthetic",

            Skirt::Unknown => "Unknown",

            Skirt::Vinyl => "Vinyl",

            Skirt::Wood => "Wood",

            Skirt::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Skirt::Aluminum => "Aluminum".into(),

            Skirt::Block => "Block".into(),

            Skirt::Brick => "Brick".into(),

            Skirt::Combination => "Combination".into(),

            Skirt::Concrete => "Concrete".into(),

            Skirt::Fiberglass => "Fiberglass".into(),

            Skirt::Frame => "Frame".into(),

            Skirt::Glass => "Glass".into(),

            Skirt::Masonite => "Masonite".into(),

            Skirt::Metal => "Metal".into(),

            Skirt::None => "None".into(),

            Skirt::Other => "Other".into(),

            Skirt::Steel => "Steel".into(),

            Skirt::Stone => "Stone".into(),

            Skirt::Stucco => "Stucco".into(),

            Skirt::Synthetic => "Synthetic".into(),

            Skirt::Unknown => "Unknown".into(),

            Skirt::Vinyl => "Vinyl".into(),

            Skirt::Wood => "Wood".into(),

            Skirt::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Skirt::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<String> for Skirt {
    fn from(s: String) -> Skirt {
        match s.as_ref() {
            "Aluminum" => Skirt::Aluminum,

            "Block" => Skirt::Block,

            "Brick" => Skirt::Brick,

            "Combination" => Skirt::Combination,

            "Concrete" => Skirt::Concrete,

            "Fiberglass" => Skirt::Fiberglass,

            "Frame" => Skirt::Frame,

            "Glass" => Skirt::Glass,

            "Masonite" => Skirt::Masonite,

            "Metal" => Skirt::Metal,

            "None" => Skirt::None,

            "Other" => Skirt::Other,

            "Steel" => Skirt::Steel,

            "Stone" => Skirt::Stone,

            "Stucco" => Skirt::Stucco,

            "Synthetic" => Skirt::Synthetic,

            "Unknown" => Skirt::Unknown,

            "Vinyl" => Skirt::Vinyl,

            "Wood" => Skirt::Wood,

            _ => Skirt::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Skirt {
    fn from(s: &str) -> Skirt {
        match s {
            "Aluminum" => Skirt::Aluminum,

            "Block" => Skirt::Block,

            "Brick" => Skirt::Brick,

            "Combination" => Skirt::Combination,

            "Concrete" => Skirt::Concrete,

            "Fiberglass" => Skirt::Fiberglass,

            "Frame" => Skirt::Frame,

            "Glass" => Skirt::Glass,

            "Masonite" => Skirt::Masonite,

            "Metal" => Skirt::Metal,

            "None" => Skirt::None,

            "Other" => Skirt::Other,

            "Steel" => Skirt::Steel,

            "Stone" => Skirt::Stone,

            "Stucco" => Skirt::Stucco,

            "Synthetic" => Skirt::Synthetic,

            "Unknown" => Skirt::Unknown,

            "Vinyl" => Skirt::Vinyl,

            "Wood" => Skirt::Wood,

            _ => Skirt::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Skirt> for &'a str {
    fn from(s: &'a Skirt) -> &'a str {
        match s {
            Skirt::Aluminum => "Aluminum",

            Skirt::Block => "Block",

            Skirt::Brick => "Brick",

            Skirt::Combination => "Combination",

            Skirt::Concrete => "Concrete",

            Skirt::Fiberglass => "Fiberglass",

            Skirt::Frame => "Frame",

            Skirt::Glass => "Glass",

            Skirt::Masonite => "Masonite",

            Skirt::Metal => "Metal",

            Skirt::None => "None",

            Skirt::Other => "Other",

            Skirt::Steel => "Steel",

            Skirt::Stone => "Stone",

            Skirt::Stucco => "Stucco",

            Skirt::Synthetic => "Synthetic",

            Skirt::Unknown => "Unknown",

            Skirt::Vinyl => "Vinyl",

            Skirt::Wood => "Wood",

            Skirt::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Skirt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Skirt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}
