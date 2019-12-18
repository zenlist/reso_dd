// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Fencing Lookups](https://ddwiki.reso.org/display/DDW17/Fencing+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fencing {
    /// "[Back Yard](https://ddwiki.reso.org/display/DDW17/Back+Yard)": The back yard is fenced.
    BackYard,

    /// "[Barbed Wire](https://ddwiki.reso.org/display/DDW17/Barbed+Wire)": The fencing has barbed wire.
    BarbedWire,

    /// "[Block](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244752)": The property has a block wall(s).
    Block,

    /// "[Brick](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244753)": The property has a brick wall(s).
    Brick,

    /// "[Chain Link](https://ddwiki.reso.org/display/DDW17/Chain+Link)": The property has chain link fencing.
    ChainLink,

    /// "[Cross Fenced](https://ddwiki.reso.org/display/DDW17/Cross+Fenced)": The property is cross fenced.
    CrossFenced,

    /// "[Electric](https://ddwiki.reso.org/display/DDW17/Electric)": The property has electric fencing.
    Electric,

    /// "[Fenced](https://ddwiki.reso.org/display/DDW17/Fenced)": The property is fenced.
    Fenced,

    /// "[Front Yard](https://ddwiki.reso.org/display/DDW17/Front+Yard)": The front yard is fenced.
    FrontYard,

    /// "[Full](https://ddwiki.reso.org/display/DDW17/Full)": The full property is fenced.
    Full,

    /// "[Gate](https://ddwiki.reso.org/display/DDW17/Gate)": The fencing has a gate(s).
    Gate,

    /// "[Invisible](https://ddwiki.reso.org/display/DDW17/Invisible)": The property has invisible fencing.
    Invisible,

    /// "[Masonry](https://ddwiki.reso.org/display/DDW17/Masonry)": The property has masonry wall(s).
    Masonry,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244763)": The property has no fencing.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244764)": The property has a type of fencing that is not included in this list.
    Other,

    /// "[Partial](https://ddwiki.reso.org/display/DDW17/Partial)": The property is partially fenced.
    Partial,

    /// "[Partial Cross](https://ddwiki.reso.org/display/DDW17/Partial+Cross)": The property has partial cross fencing.
    PartialCross,

    /// "[Perimeter](https://ddwiki.reso.org/display/DDW17/Perimeter)": The property has a perimeter fence.
    Perimeter,

    /// "[Pipe](https://ddwiki.reso.org/display/DDW17/Pipe)": The property has pipe fencing.
    Pipe,

    /// "[Privacy](https://ddwiki.reso.org/display/DDW17/Privacy)": The property has privacy fencing.
    Privacy,

    /// "[Security](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244770)": The property has security fencing.
    Security,

    /// "[See Remarks](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244771)": See the Public or Private remarks for details on the fencing.
    SeeRemarks,

    /// "[Slump Stone](https://ddwiki.reso.org/display/DDW17/Slump+Stone)": The property has slump stone wall(s).
    SlumpStone,

    /// "[Split Rail](https://ddwiki.reso.org/display/DDW17/Split+Rail)": The property has split rail fencing.
    SplitRail,

    /// "[Stone](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244774)": The property has stone wall(s).
    Stone,

    /// "[Vinyl](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244775)": The property has vinyl fencing.
    Vinyl,

    /// "[Wire](https://ddwiki.reso.org/display/DDW17/Wire)": The property has wire fencing.
    Wire,

    /// "[Wood](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244777)": The property has wooden fencing.
    Wood,

    /// "[Wrought Iron](https://ddwiki.reso.org/display/DDW17/Wrought+Iron)": The property has wrought iron fencing.
    WroughtIron,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Fencing {
    fn from(s: String) -> Fencing {
        match s.as_ref() {
            "Back Yard" => Fencing::BackYard,

            "Barbed Wire" => Fencing::BarbedWire,

            "Block" => Fencing::Block,

            "Brick" => Fencing::Brick,

            "Chain Link" => Fencing::ChainLink,

            "Cross Fenced" => Fencing::CrossFenced,

            "Electric" => Fencing::Electric,

            "Fenced" => Fencing::Fenced,

            "Front Yard" => Fencing::FrontYard,

            "Full" => Fencing::Full,

            "Gate" => Fencing::Gate,

            "Invisible" => Fencing::Invisible,

            "Masonry" => Fencing::Masonry,

            "None" => Fencing::None,

            "Other" => Fencing::Other,

            "Partial" => Fencing::Partial,

            "Partial Cross" => Fencing::PartialCross,

            "Perimeter" => Fencing::Perimeter,

            "Pipe" => Fencing::Pipe,

            "Privacy" => Fencing::Privacy,

            "Security" => Fencing::Security,

            "See Remarks" => Fencing::SeeRemarks,

            "Slump Stone" => Fencing::SlumpStone,

            "Split Rail" => Fencing::SplitRail,

            "Stone" => Fencing::Stone,

            "Vinyl" => Fencing::Vinyl,

            "Wire" => Fencing::Wire,

            "Wood" => Fencing::Wood,

            "Wrought Iron" => Fencing::WroughtIron,

            _ => Fencing::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Fencing {
    fn from(s: &str) -> Fencing {
        match s {
            "Back Yard" => Fencing::BackYard,

            "Barbed Wire" => Fencing::BarbedWire,

            "Block" => Fencing::Block,

            "Brick" => Fencing::Brick,

            "Chain Link" => Fencing::ChainLink,

            "Cross Fenced" => Fencing::CrossFenced,

            "Electric" => Fencing::Electric,

            "Fenced" => Fencing::Fenced,

            "Front Yard" => Fencing::FrontYard,

            "Full" => Fencing::Full,

            "Gate" => Fencing::Gate,

            "Invisible" => Fencing::Invisible,

            "Masonry" => Fencing::Masonry,

            "None" => Fencing::None,

            "Other" => Fencing::Other,

            "Partial" => Fencing::Partial,

            "Partial Cross" => Fencing::PartialCross,

            "Perimeter" => Fencing::Perimeter,

            "Pipe" => Fencing::Pipe,

            "Privacy" => Fencing::Privacy,

            "Security" => Fencing::Security,

            "See Remarks" => Fencing::SeeRemarks,

            "Slump Stone" => Fencing::SlumpStone,

            "Split Rail" => Fencing::SplitRail,

            "Stone" => Fencing::Stone,

            "Vinyl" => Fencing::Vinyl,

            "Wire" => Fencing::Wire,

            "Wood" => Fencing::Wood,

            "Wrought Iron" => Fencing::WroughtIron,

            _ => Fencing::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Fencing> for &'a str {
    fn from(s: &'a Fencing) -> &'a str {
        match s {
            Fencing::BackYard => "Back Yard",

            Fencing::BarbedWire => "Barbed Wire",

            Fencing::Block => "Block",

            Fencing::Brick => "Brick",

            Fencing::ChainLink => "Chain Link",

            Fencing::CrossFenced => "Cross Fenced",

            Fencing::Electric => "Electric",

            Fencing::Fenced => "Fenced",

            Fencing::FrontYard => "Front Yard",

            Fencing::Full => "Full",

            Fencing::Gate => "Gate",

            Fencing::Invisible => "Invisible",

            Fencing::Masonry => "Masonry",

            Fencing::None => "None",

            Fencing::Other => "Other",

            Fencing::Partial => "Partial",

            Fencing::PartialCross => "Partial Cross",

            Fencing::Perimeter => "Perimeter",

            Fencing::Pipe => "Pipe",

            Fencing::Privacy => "Privacy",

            Fencing::Security => "Security",

            Fencing::SeeRemarks => "See Remarks",

            Fencing::SlumpStone => "Slump Stone",

            Fencing::SplitRail => "Split Rail",

            Fencing::Stone => "Stone",

            Fencing::Vinyl => "Vinyl",

            Fencing::Wire => "Wire",

            Fencing::Wood => "Wood",

            Fencing::WroughtIron => "Wrought Iron",

            Fencing::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Fencing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Fencing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_fencing_format {
    use super::Fencing;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Fencing>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Fencing>>, D::Error>
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
