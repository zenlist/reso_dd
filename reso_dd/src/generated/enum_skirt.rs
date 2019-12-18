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

pub(crate) mod option_vec_skirt_format {
    use super::Skirt;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(items: &Option<Vec<Skirt>>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Skirt>>, D::Error>
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
