// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Basement Lookups](https://ddwiki.reso.org/display/DDW17/Basement+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Basement {
    /// "[Apartment](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244002)": The basement is setup as an apartment living space.
    Apartment,

    /// "[Bath/Stubbed](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244003)": The basement is stubbed for a bathroom.
    BathStubbed,

    /// "[Block](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244004)": The basement has block construction.
    Block,

    /// "[Concrete](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244005)": The basement has a concrete floor and/or walls.
    Concrete,

    /// "[Crawl Space](https://ddwiki.reso.org/display/DDW17/Crawl+Space)": The basement is/has a crawl space.
    CrawlSpace,

    /// "[Daylight](https://ddwiki.reso.org/display/DDW17/Daylight)": The basement has natural lighting.
    Daylight,

    /// "[Dirt Floor](https://ddwiki.reso.org/display/DDW17/Dirt+Floor)": The basement has a dirt floor.
    DirtFloor,

    /// "[Exterior Entry](https://ddwiki.reso.org/display/DDW17/Exterior+Entry)": The basement has an exterior entry.
    ExteriorEntry,

    /// "[Finished](https://ddwiki.reso.org/display/DDW17/Finished)": The basement is finished to a given standard of competition.  Examples may include underlayment and flooring; walls are framed, insulated, drywalled and painted; etc.
    Finished,

    /// "[French Drain](https://ddwiki.reso.org/display/DDW17/French+Drain)": The basement has a French drain.
    FrenchDrain,

    /// "[Full](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244012)": The basement fills the entire space under the house.
    Full,

    /// "[Interior Entry](https://ddwiki.reso.org/display/DDW17/Interior+Entry)": The basement has an interior entry.
    InteriorEntry,

    /// "[None](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244014)": The property has no basement.
    None,

    /// "[Other](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244015)": The basement has features or attributes other than those listed in this field.
    Other,

    /// "[Partial](https://ddwiki.reso.org/pages/viewpage.action?pageId=29244016)": The basement partially fills the space under the house.
    Partial,

    /// "[Partially Finished](https://ddwiki.reso.org/display/DDW17/Partially+Finished)": The basement is partially finished.  Some finishing work is done but not all.  Examples may include underlayment and flooring; walls are framed, insulated, drywalled and painted; etc.
    PartiallyFinished,

    /// "[Storage Space](https://ddwiki.reso.org/display/DDW17/Storage+Space)": The basement has storage space.
    StorageSpace,

    /// "[Sump Pump](https://ddwiki.reso.org/display/DDW17/Sump+Pump)": The basement has a sump pump.
    SumpPump,

    /// "[Unfinished](https://ddwiki.reso.org/display/DDW17/Unfinished)": The basement is unfinished.
    Unfinished,

    /// "[Walk-Out Access](https://ddwiki.reso.org/display/DDW17/Walk-Out+Access)": A walk-out basement is a structure where the basement space directly accessible from the outside with the entryway level with the ground.
    WalkOutAccess,

    /// "[Walk-Up Access](https://ddwiki.reso.org/display/DDW17/Walk-Up+Access)": A walk-up basement is a structure where the basement space directly accessible from the outside with the entryway below ground and usually exterior stairs leading up to ground level.
    WalkUpAccess,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Basement {
    fn from(s: String) -> Basement {
        match s.as_ref() {
            "Apartment" => Basement::Apartment,

            "Bath/Stubbed" => Basement::BathStubbed,

            "Block" => Basement::Block,

            "Concrete" => Basement::Concrete,

            "Crawl Space" => Basement::CrawlSpace,

            "Daylight" => Basement::Daylight,

            "Dirt Floor" => Basement::DirtFloor,

            "Exterior Entry" => Basement::ExteriorEntry,

            "Finished" => Basement::Finished,

            "French Drain" => Basement::FrenchDrain,

            "Full" => Basement::Full,

            "Interior Entry" => Basement::InteriorEntry,

            "None" => Basement::None,

            "Other" => Basement::Other,

            "Partial" => Basement::Partial,

            "Partially Finished" => Basement::PartiallyFinished,

            "Storage Space" => Basement::StorageSpace,

            "Sump Pump" => Basement::SumpPump,

            "Unfinished" => Basement::Unfinished,

            "Walk-Out Access" => Basement::WalkOutAccess,

            "Walk-Up Access" => Basement::WalkUpAccess,

            _ => Basement::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Basement {
    fn from(s: &str) -> Basement {
        match s {
            "Apartment" => Basement::Apartment,

            "Bath/Stubbed" => Basement::BathStubbed,

            "Block" => Basement::Block,

            "Concrete" => Basement::Concrete,

            "Crawl Space" => Basement::CrawlSpace,

            "Daylight" => Basement::Daylight,

            "Dirt Floor" => Basement::DirtFloor,

            "Exterior Entry" => Basement::ExteriorEntry,

            "Finished" => Basement::Finished,

            "French Drain" => Basement::FrenchDrain,

            "Full" => Basement::Full,

            "Interior Entry" => Basement::InteriorEntry,

            "None" => Basement::None,

            "Other" => Basement::Other,

            "Partial" => Basement::Partial,

            "Partially Finished" => Basement::PartiallyFinished,

            "Storage Space" => Basement::StorageSpace,

            "Sump Pump" => Basement::SumpPump,

            "Unfinished" => Basement::Unfinished,

            "Walk-Out Access" => Basement::WalkOutAccess,

            "Walk-Up Access" => Basement::WalkUpAccess,

            _ => Basement::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Basement> for &'a str {
    fn from(s: &'a Basement) -> &'a str {
        match s {
            Basement::Apartment => "Apartment",

            Basement::BathStubbed => "Bath/Stubbed",

            Basement::Block => "Block",

            Basement::Concrete => "Concrete",

            Basement::CrawlSpace => "Crawl Space",

            Basement::Daylight => "Daylight",

            Basement::DirtFloor => "Dirt Floor",

            Basement::ExteriorEntry => "Exterior Entry",

            Basement::Finished => "Finished",

            Basement::FrenchDrain => "French Drain",

            Basement::Full => "Full",

            Basement::InteriorEntry => "Interior Entry",

            Basement::None => "None",

            Basement::Other => "Other",

            Basement::Partial => "Partial",

            Basement::PartiallyFinished => "Partially Finished",

            Basement::StorageSpace => "Storage Space",

            Basement::SumpPump => "Sump Pump",

            Basement::Unfinished => "Unfinished",

            Basement::WalkOutAccess => "Walk-Out Access",

            Basement::WalkUpAccess => "Walk-Up Access",

            Basement::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Basement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Basement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_basement_format {
    use super::Basement;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Basement>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Basement>>, D::Error>
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
