// THIS IS A GENERATED FILE
// If anything in this file needs to be updated, it needs to be fixed in reso_dd_generator
use serde::{Deserialize, Serialize};

/// [Levels Lookups](https://ddwiki.reso.org/display/DDW17/Levels+Lookups)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Levels {
    /// "[Multi/Split](https://ddwiki.reso.org/pages/viewpage.action?pageId=29245361)": A split-level home (also called a tri-level home) is a style of house in which the floor levels are staggered, so that the "main" level of the house (e.g. the level that usually contains the front entry), is partway between the upper and lower floors.
    MultiSplit,

    /// "[One](https://ddwiki.reso.org/display/DDW17/One)": The property being sold has one level.  A discreet horizontal plane of interior living space (excluding basements).
    One,

    /// "[One and One Half](https://ddwiki.reso.org/display/DDW17/One+and+One+Half)": A 1.5 story house is where the height of any of the walls on the second floor are less than the height of the walls on the first floor.  First floor walls that extend up to the second floor level are not included in the height comparison.  Depending on your State, Provence or other local regulations there may be a specific height difference required.  For example some areas required that the second floor walls be less than 70% the height of the first floor walls in order to be called a 1.5 story structure.  Other areas are 50% and it is clear that this may vary substantially from region to region.  Another common trait is angled ceilings that compensate for the short walls making all but the edges of the room high enough for normal use.
    OneandOneHalf,

    /// "[Three Or More](https://ddwiki.reso.org/display/DDW17/Three+Or+More)": The property being sold has three or more levels.  A discreet horizontal plane of interior living space (excluding basements).
    ThreeOrMore,

    /// "[Two](https://ddwiki.reso.org/display/DDW17/Two)": The property being sold has two levels.  A discreet horizontal plane of interior living space (excluding basements).
    Two,

    /// A value that was not defined by the enumeration
    OpenEnumeration(String),
}

impl From<String> for Levels {
    fn from(s: String) -> Levels {
        match s.as_ref() {
            "Multi/Split" => Levels::MultiSplit,

            "One" => Levels::One,

            "One and One Half" => Levels::OneandOneHalf,

            "Three Or More" => Levels::ThreeOrMore,

            "Two" => Levels::Two,

            _ => Levels::OpenEnumeration(s),
        }
    }
}

impl From<&str> for Levels {
    fn from(s: &str) -> Levels {
        match s {
            "Multi/Split" => Levels::MultiSplit,

            "One" => Levels::One,

            "One and One Half" => Levels::OneandOneHalf,

            "Three Or More" => Levels::ThreeOrMore,

            "Two" => Levels::Two,

            _ => Levels::OpenEnumeration(s.into()),
        }
    }
}

impl<'a> From<&'a Levels> for &'a str {
    fn from(s: &'a Levels) -> &'a str {
        match s {
            Levels::MultiSplit => "Multi/Split",

            Levels::One => "One",

            Levels::OneandOneHalf => "One and One Half",

            Levels::ThreeOrMore => "Three Or More",

            Levels::Two => "Two",

            Levels::OpenEnumeration(s) => s,
        }
    }
}

impl Serialize for Levels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Levels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(From::from(s))
    }
}

pub(crate) mod option_vec_levels_format {
    use super::Levels;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(dead_code)]
    pub(crate) fn serialize<S>(
        items: &Option<Vec<Levels>>,
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
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Levels>>, D::Error>
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
