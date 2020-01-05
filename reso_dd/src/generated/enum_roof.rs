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

impl crate::ResoEnumeration for Roof {
    fn from_str(s: &str) -> Roof {
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

    fn from_string(s: String) -> Roof {
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

    fn to_str(&self) -> &str {
        match self {
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

            Roof::OpenEnumeration(ref s) => s,
        }
    }

    fn into_string(self) -> String {
        match self {
            Roof::Aluminum => "Aluminum".into(),

            Roof::AsbestosShingle => "Asbestos Shingle".into(),

            Roof::Asphalt => "Asphalt".into(),

            Roof::Bahama => "Bahama".into(),

            Roof::Barrel => "Barrel".into(),

            Roof::Bituthene => "Bituthene".into(),

            Roof::BuiltUp => "Built-Up".into(),

            Roof::Composition => "Composition".into(),

            Roof::Concrete => "Concrete".into(),

            Roof::Copper => "Copper".into(),

            Roof::Elastomeric => "Elastomeric".into(),

            Roof::Fiberglass => "Fiberglass".into(),

            Roof::Flat => "Flat".into(),

            Roof::FlatTile => "Flat Tile".into(),

            Roof::Foam => "Foam".into(),

            Roof::GreenRoof => "Green Roof".into(),

            Roof::Mansard => "Mansard".into(),

            Roof::Membrane => "Membrane".into(),

            Roof::Metal => "Metal".into(),

            Roof::Mixed => "Mixed".into(),

            Roof::None => "None".into(),

            Roof::Other => "Other".into(),

            Roof::RolledHotMop => "Rolled/Hot Mop".into(),

            Roof::Rubber => "Rubber".into(),

            Roof::SeeRemarks => "See Remarks".into(),

            Roof::Shake => "Shake".into(),

            Roof::Shingle => "Shingle".into(),

            Roof::Slate => "Slate".into(),

            Roof::SpanishTile => "Spanish Tile".into(),

            Roof::Stone => "Stone".into(),

            Roof::Synthetic => "Synthetic".into(),

            Roof::TarGravel => "Tar/Gravel".into(),

            Roof::Tile => "Tile".into(),

            Roof::Wood => "Wood".into(),

            Roof::OpenEnumeration(s) => s,
        }
    }

    fn fallback_value(&self) -> Option<&str> {
        match self {
            Roof::OpenEnumeration(ref s) => Some(s),
            _ => None,
        }
    }
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
