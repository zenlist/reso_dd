// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Roof Lookups](https://ddwiki.reso.org/display/DDW17/Roof+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Roof {
    /// "[Aluminum](https://ddwiki.reso.org/display/DDW17/Aluminum)": The roof is made wholly/partially of aluminum.
    Aluminum,

    /// "[Asbestos Shingle](https://ddwiki.reso.org/display/DDW17/Asbestos+Shingle)": The roof is made wholly/partially of asbestos shingles.
    AsbestosShingle,

    /// "[Asphalt](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246114)": The roof is made wholly/partially of asphalt.
    Asphalt,

    /// "[Bahama](https://ddwiki.reso.org/display/DDW17/Bahama)": The roof is a Bahama roof.
    Bahama,

    /// "[Barrel](https://ddwiki.reso.org/display/DDW17/Barrel)": The roof is a Barrel roof.
    Barrel,

    /// "[Bituthene](https://ddwiki.reso.org/display/DDW17/Bituthene)": The roof is made wholly/partially of Bituthene.
    Bituthene,

    /// "[Built-Up](https://ddwiki.reso.org/display/DDW17/Built-Up)": The roof is made wholly/partially of built-up.
    BuiltUp,

    /// "[Composition](https://ddwiki.reso.org/display/DDW17/Composition)": The roof is made wholly/partially of composition.
    Composition,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246120)": The roof is made wholly/partially of concrete.
    Concrete,

    /// "[Copper](https://ddwiki.reso.org/display/DDW17/Copper)": The roof is made wholly/partially of copper.
    Copper,

    /// "[Elastomeric](https://ddwiki.reso.org/display/DDW17/Elastomeric)": The roof is made wholly/partially of elastomeric.
    Elastomeric,

    /// "[Fiberglass](https://ddwiki.reso.org/display/DDW17/Fiberglass)": The roof is made wholly/partially of fiberglass.
    Fiberglass,

    /// "[Flat](https://ddwiki.reso.org/display/DDW17/Flat)": The roof is wholly/partially flat.
    Flat,

    /// "[Flat Tile](https://ddwiki.reso.org/display/DDW17/Flat+Tile)": The roof is made wholly/partially of flat tile.
    FlatTile,

    /// "[Foam](https://ddwiki.reso.org/display/DDW17/Foam)": The roof is made wholly/partially of foam.
    Foam,

    /// "[Green Roof](https://ddwiki.reso.org/display/DDW17/Green+Roof)": The roof is wholly/partially a green roof.
    GreenRoof,

    /// "[Mansard](https://ddwiki.reso.org/display/DDW17/Mansard)": The roof is made wholly/partially of mansard.
    Mansard,

    /// "[Membrane](https://ddwiki.reso.org/display/DDW17/Membrane)": The roof is made wholly/partially of membrane.
    Membrane,

    /// "[Metal](https://ddwiki.reso.org/display/DDW17/Metal)": The roof is made wholly/partially of metal.
    Metal,

    /// "[Mixed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246131)": The roof is made wholly/partially of mixed materials.
    Mixed,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246132)": The roof materials are unstated, unknown or there are none.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246133)": The roof is made wholly/partially of materials other than those in this list.
    Other,

    /// "[Rolled/Hot Mop](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246134)": The roof is made wholly/partially of rolled/hot mop.
    RolledHotMop,

    /// "[Rubber](https://ddwiki.reso.org/display/DDW17/Rubber)": The roof is made wholly/partially of rubber.
    Rubber,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246136)": See the listing's remarks for details on the roof.
    SeeRemarks,

    /// "[Shake](https://ddwiki.reso.org/display/DDW17/Shake)": The roof is made wholly/partially of shake.
    Shake,

    /// "[Shingle](https://ddwiki.reso.org/display/DDW17/Shingle)": The roof is made wholly/partially of shingle.
    Shingle,

    /// "[Slate](https://ddwiki.reso.org/display/DDW17/Slate)": The roof is made wholly/partially of slate.
    Slate,

    /// "[Spanish Tile](https://ddwiki.reso.org/display/DDW17/Spanish+Tile)": The roof is made wholly/partially of Spanish tile.
    SpanishTile,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246141)": The roof is made wholly/partially of stone.
    Stone,

    /// "[Synthetic](https://ddwiki.reso.org/display/DDW17/Synthetic)": The roof is made wholly/partially of synthetic materials.
    Synthetic,

    /// "[Tar/Gravel](https://ddwiki.reso.org/pages/viewpage.action?pageId=29246143)": The roof is made wholly/partially of tar/gravel.
    TarGravel,

    /// "[Tile](https://ddwiki.reso.org/display/DDW17/Tile)": The roof is made wholly/partially of tile.
    Tile,

    /// "[Wood](https://ddwiki.reso.org/display/DDW17/Wood)": The roof is made wholly/partially of wood.
    Wood,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Roof {
    fn from(s: String) -> Roof {
        match s.as_ref() {
            "Aluminum" => Roof::Aluminum,

            "Asbestos Shingle" => Roof::AsbestosShingle,

            "Asphalt" => Roof::Asphalt,

            "Bahama" => Roof::Bahama,

            "Barrel" => Roof::Barrel,

            "Bituthene" => Roof::Bituthene,

            "Built-Up" => Roof::BuiltUp,

            "Composition" => Roof::Composition,

            "Concrete" => Roof::Concrete,

            "Copper" => Roof::Copper,

            "Elastomeric" => Roof::Elastomeric,

            "Fiberglass" => Roof::Fiberglass,

            "Flat" => Roof::Flat,

            "Flat Tile" => Roof::FlatTile,

            "Foam" => Roof::Foam,

            "Green Roof" => Roof::GreenRoof,

            "Mansard" => Roof::Mansard,

            "Membrane" => Roof::Membrane,

            "Metal" => Roof::Metal,

            "Mixed" => Roof::Mixed,

            "None" => Roof::None,

            "Other" => Roof::Other,

            "Rolled/Hot Mop" => Roof::RolledHotMop,

            "Rubber" => Roof::Rubber,

            "See Remarks" => Roof::SeeRemarks,

            "Shake" => Roof::Shake,

            "Shingle" => Roof::Shingle,

            "Slate" => Roof::Slate,

            "Spanish Tile" => Roof::SpanishTile,

            "Stone" => Roof::Stone,

            "Synthetic" => Roof::Synthetic,

            "Tar/Gravel" => Roof::TarGravel,

            "Tile" => Roof::Tile,

            "Wood" => Roof::Wood,

            _ => Roof::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Roof {
    fn from(s: &str) -> Roof {
        match s {
            "Aluminum" => Roof::Aluminum,

            "Asbestos Shingle" => Roof::AsbestosShingle,

            "Asphalt" => Roof::Asphalt,

            "Bahama" => Roof::Bahama,

            "Barrel" => Roof::Barrel,

            "Bituthene" => Roof::Bituthene,

            "Built-Up" => Roof::BuiltUp,

            "Composition" => Roof::Composition,

            "Concrete" => Roof::Concrete,

            "Copper" => Roof::Copper,

            "Elastomeric" => Roof::Elastomeric,

            "Fiberglass" => Roof::Fiberglass,

            "Flat" => Roof::Flat,

            "Flat Tile" => Roof::FlatTile,

            "Foam" => Roof::Foam,

            "Green Roof" => Roof::GreenRoof,

            "Mansard" => Roof::Mansard,

            "Membrane" => Roof::Membrane,

            "Metal" => Roof::Metal,

            "Mixed" => Roof::Mixed,

            "None" => Roof::None,

            "Other" => Roof::Other,

            "Rolled/Hot Mop" => Roof::RolledHotMop,

            "Rubber" => Roof::Rubber,

            "See Remarks" => Roof::SeeRemarks,

            "Shake" => Roof::Shake,

            "Shingle" => Roof::Shingle,

            "Slate" => Roof::Slate,

            "Spanish Tile" => Roof::SpanishTile,

            "Stone" => Roof::Stone,

            "Synthetic" => Roof::Synthetic,

            "Tar/Gravel" => Roof::TarGravel,

            "Tile" => Roof::Tile,

            "Wood" => Roof::Wood,

            _ => Roof::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Roof> for &'a str {
    fn from(s: &'a Roof) -> &'a str {
        match s {
            Roof::Aluminum => "Aluminum",

            Roof::AsbestosShingle => "Asbestos Shingle",

            Roof::Asphalt => "Asphalt",

            Roof::Bahama => "Bahama",

            Roof::Barrel => "Barrel",

            Roof::Bituthene => "Bituthene",

            Roof::BuiltUp => "Built-Up",

            Roof::Composition => "Composition",

            Roof::Concrete => "Concrete",

            Roof::Copper => "Copper",

            Roof::Elastomeric => "Elastomeric",

            Roof::Fiberglass => "Fiberglass",

            Roof::Flat => "Flat",

            Roof::FlatTile => "Flat Tile",

            Roof::Foam => "Foam",

            Roof::GreenRoof => "Green Roof",

            Roof::Mansard => "Mansard",

            Roof::Membrane => "Membrane",

            Roof::Metal => "Metal",

            Roof::Mixed => "Mixed",

            Roof::None => "None",

            Roof::Other => "Other",

            Roof::RolledHotMop => "Rolled/Hot Mop",

            Roof::Rubber => "Rubber",

            Roof::SeeRemarks => "See Remarks",

            Roof::Shake => "Shake",

            Roof::Shingle => "Shingle",

            Roof::Slate => "Slate",

            Roof::SpanishTile => "Spanish Tile",

            Roof::Stone => "Stone",

            Roof::Synthetic => "Synthetic",

            Roof::TarGravel => "Tar/Gravel",

            Roof::Tile => "Tile",

            Roof::Wood => "Wood",

            Roof::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Roof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Roof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_roof_format {
    use super::Roof;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(items: &Option<Vec<Roof>>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Roof>>, D::Error>
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
